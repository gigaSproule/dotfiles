use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Output, Stdio};

use nix::unistd::{Gid, Uid};
use walkdir::WalkDir;

use crate::system;
use crate::system::System;

#[link(name = "c")]
extern "C" {
    fn geteuid() -> u32;
    fn getegid() -> u32;
}

pub(crate) fn get_group_id() -> u32 {
    // unsafe { getegid() }
    0
}

pub(crate) fn get_user_id() -> u32 {
    // unsafe { geteuid() }
    0
}

pub(crate) fn add_to_path(file: &str, path: &str) -> Result<(), std::io::Error> {
    let mut file =
        OpenOptions::new()
            .append(true)
            .open(format!("{}/{}", system::get_home_dir(), file))?;

    writeln!(file, "export PATH = $PATH: {}\n", path)?;
    Ok(())
}

pub(crate) fn copy_config(system: &dyn System, src: &str, dst: &str) -> Result<(), std::io::Error> {
    let actual_src = format!("{:?}/{}", std::env::current_exe(), src);
    let actual_dst = format!("{}/{}", system::get_home_dir(), dst);
    fs::create_dir_all(&actual_dst);
    fs::copy(&actual_src, &actual_dst);
    let user_id = get_user_id();
    let group_id = get_group_id();
    recursively_chown(&actual_dst, &user_id, &group_id)
}

pub(crate) fn execute(command: &str, super_user: bool) -> Output {
    let mut actual_command = if super_user {
        let mut return_command = Command::new("sudo");
        return_command.arg("--");
        return_command
    } else {
        let mut return_command = Command::new("sh");
        return_command.arg("-c");
        return_command
    };
    actual_command
        .args(&[command])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect(format!("Failed to execute process `{}`", command).as_str())
}

pub(crate) async fn install_nodejs(system: &dyn System) -> Result<(), Box<dyn std::error::Error>> {
    system::download_file(
        "https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh",
        "nvm-install.sh",
    ).await?;
    recursively_chmod("nvm-install.sh", &0o644, &0o644)?;
    system.execute("./nvm-install.sh", false);
    fs::remove_file("nvm-install.sh")?;
    setup_nodejs(system)?;
    Ok(())
}

pub(crate) async fn install_rust(system: &dyn System) -> Result<(), Box<dyn std::error::Error>> {
    system::download_file("https://sh.rustup.rs", "rustup-install").await?;
    recursively_chmod("rustup-install", &0o644, &0o644)?;
    system.execute("./rustup-install -y", false);
    fs::remove_file("rustup-install")?;
    add_to_path(".zshrc.custom", "$HOME/.cargo/bin")?;
    add_to_path(".bashrc.custom", "$HOME/.cargo/bin")?;
    Ok(())
}

pub(crate) fn recursively_chmod(
    path: &str,
    directory_permission: &u32,
    file_permission: &u32,
) -> Result<(), std::io::Error> {
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(*directory_permission);
    for entry in WalkDir::new(path).follow_links(true) {
        let entr = entry?;
        let child_path = entr.path();
        let mut child_perms = fs::metadata(child_path)?.permissions();
        if entr.file_type().is_dir() {
            child_perms.set_mode(*directory_permission);
        } else {
            child_perms.set_mode(*file_permission);
        }
    }
    Ok(())
}

pub(crate) fn recursively_chown(path: &str, user: &u32, group: &u32) -> Result<(), std::io::Error> {
    nix::unistd::chown(path, Some(Uid::from_raw(*user)), Some(Gid::from_raw(*group)))?;
    for entry in WalkDir::new(path).follow_links(true) {
        let entr = entry?;
        let child_path = entr.path();
        nix::unistd::chown(child_path, Some(Uid::from_raw(*user)), Some(Gid::from_raw(*group)))?;
    }
    Ok(())
}

pub(crate) fn set_java_home(file: &str, jdk_path: &str) -> Result<(), std::io::Error> {
    let mut file =
        OpenOptions::new()
            .append(true)
            .open(format!("{}/{}", system::get_home_dir(), file))?;

    let mut buff = String::new();
    file.read_to_string(&mut buff)?;
    if !buff.contains("JAVA_HOME") {
        writeln!(file, "export JAVA_HOME={}", jdk_path)?;
    }
    Ok(())
}

pub(crate) fn setup_nodejs(system: &dyn System) -> Result<(), std::io::Error> {
    let mut zshrc = OpenOptions::new()
        .append(true)
        .open(format!("{}/.zshrc.custom", system::get_home_dir()))?;
    writeln!(zshrc, "export NVM_DIR=\"$([ -z \"${{XDG_CONFIG_HOME-}}\" ] && printf %s \"${{HOME}}/.nvm\" || printf %s \"${{XDG_CONFIG_HOME}}/nvm\")\"")?;
    writeln!(
        zshrc,
        "[ -s \"$NVM_DIR/nvm.sh\" ] && \\. \"$NVM_DIR/nvm.sh\" # This loads nvm"
    )?;

    let mut bashrc = OpenOptions::new()
        .append(true)
        .open(format!("{}/.bashrc.custom", system::get_home_dir()))?;
    writeln!(bashrc, "export NVM_DIR=\"$([ -z \"${{XDG_CONFIG_HOME-}}\" ] && printf %s \"${{HOME}}/.nvm\" || printf %s \"${{XDG_CONFIG_HOME}}/nvm\")\"")?;
    writeln!(
        bashrc,
        "[ -s \"$NVM_DIR/nvm.sh\" ] && \\. \"$NVM_DIR/nvm.sh\" # This loads nvm"
    )?;

    system.execute("nvm install node --latest-npm", false);
    system.execute("npm install --global yarn", false);
    Ok(())
}

pub(crate) fn setup_tmux(system: &dyn System) {
    copy_config(system, "tmux/tmux.conf", ".tmux.conf");
}

pub(crate) async fn setup_zsh(system: &dyn System, zsh_bin: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let zsh = zsh_bin.unwrap_or("/usr/bin/zsh");
    system::download_file(
        "https://raw.githubusercontent.com/loket/oh-my-zsh/feature/batch-mode/tools/install.sh",
        "oh-my-zsh.sh",
    ).await?;
    recursively_chmod("./oh-my-zsh.sh", &0o644, &0o644)?;
    system.execute("./oh-my-zsh.sh", false);
    copy_config(system, "zsh/zshrc", ".zshrc");
    system.execute(&format!("chsh -s {}", zsh), true);
    system.execute(&format!("chsh -s {} {}", zsh, whoami::username()), true);
    fs::remove_file("oh-my-zsh.sh")?;
    Ok(())
}

pub(crate) fn symlink(source: &str, destination: &str) {
    execute(&format!("ln -sfn {} {}", source, destination), true);
}