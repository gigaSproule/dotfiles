use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

use nix::unistd::{chown, Gid, Uid};
use walkdir::WalkDir;

use crate::system;
use crate::system::System;

pub(crate) fn get_group_id() -> u32 {
    env::var("SUDO_GID").unwrap().parse::<u32>().unwrap()
}

pub(crate) fn get_user_id() -> u32 {
    env::var("SUDO_UID").unwrap().parse::<u32>().unwrap()
}

pub(crate) fn get_username() -> String {
    env::var("SUDO_USER").unwrap()
}

pub(crate) fn add_to_file(file: &str, content: &str) -> Result<(), std::io::Error> {
    if !system::file_contains(file, content) {
        let mut actual_file = OpenOptions::new().append(true).open(&file)?;
        writeln!(actual_file, "{}", content)?;
    }
    Ok(())
}

pub(crate) fn add_to_path(system: &impl System, file: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if system::file_contains(file, "export PATH") {
        let original_file = File::open(file)?;
        let original_lines = BufReader::new(original_file).lines();
        let new_lines = original_lines
            .map(|line| {
                let unwrapped_line = line.unwrap();
                if unwrapped_line.starts_with("export PATH=") && !unwrapped_line.contains(&path) {
                    let mut split_line = unwrapped_line.split("=");
                    split_line.next();
                    let unwrapped_next_split = split_line.next().unwrap();
                    format!("export PATH={}{}", unwrapped_next_split, path)
                } else {
                    unwrapped_line
                }
            })
            .collect::<Vec<String>>();

        let mut new_file = OpenOptions::new().write(true).open(file)?;
        new_file.write_all(new_lines.join("\n").as_bytes())?;
    } else {
        let mut append_file = OpenOptions::new().append(true).open(format!(
            "{}/{}",
            system.get_home_dir(),
            file
        ))?;
        writeln!(append_file, "export PATH=$PATH:{}\n", path)?;
    }
    std::env::set_var("PATH", format!("{}:{}", std::env::var("PATH")?, path));
    Ok(())
}

pub(crate) fn add_variable_to_file(file: &str, key: &str, value: &str) -> Result<(), std::io::Error> {
    if !system::file_contains(&file, key) {
        let mut actual_file = OpenOptions::new().append(true).open(&file)?;
        writeln!(actual_file, "export {}={}", key, value)?;
    }
    Ok(())
}

pub(crate) fn execute(
    command: &str,
    super_user: bool,
    dry_run: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    execute_path(
        command,
        super_user,
        &env::current_dir()
            .expect("Could not get current directory")
            .into_os_string()
            .into_string()
            .expect("Could not convert current directory path to a string"),
        dry_run,
    )
}

pub(crate) fn execute_path(
    command: &str,
    super_user: bool,
    path: &str,
    dry_run: bool
) -> Result<String, Box<dyn std::error::Error>> {
    let mut actual_command = if !super_user {
        let split = command.split_whitespace().collect::<Vec<&str>>();
        let sudo_user = get_username();
        let mut args = vec!["-u", &sudo_user];
        args.extend(split);
        let mut return_command = Command::new("sudo");
        return_command.args(&args);
        return_command
    } else {
        let mut split = command.split_whitespace();
        let mut return_command = Command::new(
            split
                .nth(0)
                .expect("Could not find the first part of the command"),
        );
        return_command.args(split.collect::<Vec<&str>>());
        return_command
    };

    let child = actual_command.current_dir(path);
    system::run_command(child, dry_run)
}

pub(crate) fn recursively_chmod(
    path: &str,
    directory_permission: &u32,
    file_permission: &u32,
) -> Result<(), std::io::Error> {
    for entry in WalkDir::new(path).follow_links(true) {
        let entr = entry?;
        let child_path = entr.path();
        if entr.file_type().is_dir() {
            fs::set_permissions(child_path, fs::Permissions::from_mode(*directory_permission)).unwrap();
        } else {
            fs::set_permissions(child_path, fs::Permissions::from_mode(*file_permission)).unwrap();
        }
    }
    Ok(())
}

pub(crate) fn recursively_chown(path: &str, user: &u32, group: &u32) -> Result<(), std::io::Error> {
    chown(
        path,
        Some(Uid::from_raw(*user)),
        Some(Gid::from_raw(*group)),
    )?;
    for entry in WalkDir::new(path).follow_links(true) {
        let entr = entry?;
        let child_path = entr.path();
        chown(
            child_path,
            Some(Uid::from_raw(*user)),
            Some(Gid::from_raw(*group)),
        )?;
    }
    Ok(())
}

pub(crate) fn set_java_home(system: &impl System, file: &str, jdk_path: &str) -> Result<(), std::io::Error> {
    let path = format!("{}/{}", system.get_home_dir(), file);
    println!("Appending JAVA_HOME as {} to {}", jdk_path, &path);
    add_variable_to_file(&path, "JAVA_HOME", jdk_path)?;
    std::env::set_var("JAVA_HOME", jdk_path);
    Ok(())
}

pub(crate) fn setup_bash(
    system: &impl System,
) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = system.get_home_dir();
    let bashrc = format!("{}/.bashrc", home_dir);
    println!("Creating bashrc at {}", &bashrc);
    let mut bashrc_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&bashrc)?;
    writeln!(bashrc_file, "export PATH=$PATH:${{HOME}}/bin:${{HOME}}/.local/bin")?;
    writeln!(bashrc_file, "")?;

    let user_id = get_user_id();
    let group_id = get_group_id();
    recursively_chown(
        &bashrc,
        &user_id,
        &group_id,
    )?;

    let bashrc_custom = format!("{}/.bashrc.custom", home_dir);
    let bashrc_custom_path = std::path::Path::new(&bashrc_custom);
    if !bashrc_custom_path.exists() {
        println!("Creating bashrc custom at {}", bashrc_custom);
        let mut bashrc_custom_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(bashrc_custom_path)?;
        writeln!(
            bashrc_custom_file,
            "# File to contain custom config that won't get overwritten"
        )?;
        writeln!(bashrc_custom_file, "")?;

        let user_id = get_user_id();
        let group_id = get_group_id();
        recursively_chown(
            &bashrc_custom,
            &user_id,
            &group_id,
        )?;
    }
    Ok(())
}

pub(crate) fn setup_tmux(system: &impl System) -> Result<(), std::io::Error> {
    let tmux_conf = format!("{}/.tmux.conf", system.get_home_dir());
    println!("Creating tmux conf at {}", tmux_conf);
    let mut tmux_conf_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&tmux_conf)?;
    writeln!(tmux_conf_file, "# set command prefix for tmux")?;
    writeln!(tmux_conf_file, "set-option -g prefix C-a")?;
    writeln!(tmux_conf_file, "unbind C-a")?;
    writeln!(tmux_conf_file, "bind-key C-a send-prefix")?;
    writeln!(tmux_conf_file, "")?;
    writeln!(tmux_conf_file, "# set vi mode keys")?;
    writeln!(tmux_conf_file, "setw -g mode-keys vi")?;
    writeln!(tmux_conf_file, "")?;
    writeln!(
        tmux_conf_file,
        "# set some bindings for moving around terminals (vim-like)"
    )?;
    writeln!(tmux_conf_file, "bind h select-pane -L")?;
    writeln!(tmux_conf_file, "bind j select-pane -D")?;
    writeln!(tmux_conf_file, "bind k select-pane -U")?;
    writeln!(tmux_conf_file, "bind l select-pane -R")?;
    writeln!(tmux_conf_file, "")?;
    writeln!(tmux_conf_file, "bind C-M-h resize-pane -L 5")?;
    writeln!(tmux_conf_file, "bind C-h resize-pane -L 1")?;
    writeln!(tmux_conf_file, "bind C-M-j resize-pane -D 5")?;
    writeln!(tmux_conf_file, "bind C-j resize-pane -D 1")?;
    writeln!(tmux_conf_file, "bind C-M-k resize-pane -U 5")?;
    writeln!(tmux_conf_file, "bind C-k resize-pane -U 1")?;
    writeln!(tmux_conf_file, "bind C-M-l resize-pane -R 5")?;
    writeln!(tmux_conf_file, "bind C-l resize-pane -R 1")?;
    writeln!(tmux_conf_file, "")?;
    writeln!(tmux_conf_file, "# Define my custom menu bar")?;
    writeln!(tmux_conf_file, "# status bar colors")?;
    writeln!(tmux_conf_file, "set -g status-bg black")?;
    writeln!(tmux_conf_file, "set -g status-fg white")?;
    writeln!(tmux_conf_file, "")?;
    writeln!(tmux_conf_file, "# alignment settings")?;
    writeln!(tmux_conf_file, "set-option -g status-justify centre")?;
    writeln!(tmux_conf_file, "")?;
    writeln!(tmux_conf_file, "# status left options")?;
    writeln!(
        tmux_conf_file,
        "set-option -g status-left '#[fg=green][#[bg=black,fg=cyan]#S#[fg=green]]'"
    )?;
    writeln!(tmux_conf_file, "set-option -g status-left-length 20")?;
    writeln!(tmux_conf_file, "")?;
    writeln!(tmux_conf_file, "# window list options")?;
    writeln!(tmux_conf_file, "setw -g automatic-rename on")?;
    writeln!(tmux_conf_file, "set-window-option -g window-status-format '#[fg=cyan,dim]#I#[fg=blue]:#[default]#W#[fg=grey,dim]#F'")?;
    writeln!(tmux_conf_file, "set-window-option -g window-status-current-format '#[bg=blue,fg=cyan,bold]#I#[bg=blue,fg=cyan]:#[fg=colour230]#W#[fg=dim]#F'")?;
    writeln!(tmux_conf_file, "set -g base-index 1")?;
    writeln!(tmux_conf_file, "")?;
    writeln!(tmux_conf_file, "# status right options")?;
    writeln!(tmux_conf_file, "set -g status-right '#[fg=green][#[fg=blue]%Y-%m-%d #[fg=white]%H:%M#[default]  #($HOME/bin/battery)#[fg=green]]'")?;
    writeln!(tmux_conf_file, "")?;
    writeln!(tmux_conf_file, "# bind a reload key")?;
    writeln!(
        tmux_conf_file,
        "bind R source-file ~/.tmux.conf \\; display-message \"  Config reloaded..\"."
    )?;
    writeln!(tmux_conf_file, "")?;
    writeln!(tmux_conf_file, "# Set Copy-Mode settings")?;
    writeln!(tmux_conf_file, "bind [ copy-mode")?;
    writeln!(tmux_conf_file, "#bind -T vi-copy v begin-selection")?;
    writeln!(tmux_conf_file, "#bind -T vi-copy y copy-selection")?;
    writeln!(tmux_conf_file, "#bind -T vi-copy V rectangle-toggle")?;
    writeln!(tmux_conf_file, "bind ] paste-buffer")?;
    writeln!(tmux_conf_file, "")?;
    writeln!(tmux_conf_file, "# buffer")?;
    writeln!(tmux_conf_file, "bind Space choose-buffer")?;
    writeln!(tmux_conf_file, "")?;
    writeln!(tmux_conf_file, "set -g mouse on")?;
    writeln!(
        tmux_conf_file,
        "bind m set-option -g mouse on \\; display 'Mouse: ON'"
    )?;
    writeln!(
        tmux_conf_file,
        "bind M set-option -g mouse off \\; display 'Mouse: OFF'"
    )?;
    writeln!(tmux_conf_file, "bind -n WheelUpPane if-shell -F -t = \"#{{mouse_any_flag}}\" \"send-keys -M\" \"if -Ft= '#{{pane_in_mode}}' 'send-keys -M' 'select-pane -t=; copy-mode -e; send-keys -M'\"")?;
    writeln!(
        tmux_conf_file,
        "bind -n WheelDownPane select-pane -t= \\; send-keys -M"
    )?;
    writeln!(
        tmux_conf_file,
        "#bind -T vi-copy    C-WheelUpPane   halfpage-up"
    )?;
    writeln!(
        tmux_conf_file,
        "#bind -T vi-copy    C-WheelDownPane halfpage-down"
    )?;
    writeln!(tmux_conf_file, "")?;
    writeln!(
        tmux_conf_file,
        "if-shell -b '[ -f $HOME/.tmux.custom.conf ]' \\"
    )?;
    writeln!(tmux_conf_file, "    \"source-file ~/.tmux.custom.conf\"")?;
    writeln!(tmux_conf_file, "")?;

    let user_id = get_user_id();
    let group_id = get_group_id();
    recursively_chown(
        &tmux_conf,
        &user_id,
        &group_id,
    )?;

    let tmux_conf_custom = format!("{}/.tmux.custom.conf", system.get_home_dir());
    let tmux_conf_custom_path = std::path::Path::new(&tmux_conf_custom);
    if !tmux_conf_custom_path.exists() {
        println!("Creating tmux custom conf at {}", tmux_conf_custom);
        let mut tmux_conf_custom_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(tmux_conf_custom_path)?;
        writeln!(
            tmux_conf_custom_file,
            "# File to contain custom config that won't get overwritten"
        )?;
        writeln!(tmux_conf_custom_file, "")?;

        let user_id = get_user_id();
        let group_id = get_group_id();
        recursively_chown(
            &tmux_conf_custom,
            &user_id,
            &group_id,
        )?;
    }

    Ok(())
}

pub(crate) async fn setup_zsh(
    system: &impl System,
    zsh_bin: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let zsh = zsh_bin.unwrap_or("/usr/bin/zsh");
    system::download_file(
        "https://raw.githubusercontent.com/loket/oh-my-zsh/feature/batch-mode/tools/install.sh",
        "oh-my-zsh.sh",
    ).await?;
    recursively_chmod("./oh-my-zsh.sh", &0o755, &0o755)?;
    system.execute("./oh-my-zsh.sh", false)?;
    system.execute(&format!("chsh -s {}", zsh), true)?;
    system.execute(&format!("chsh -s {} {}", zsh, get_username()), true)?;
    fs::remove_file("oh-my-zsh.sh")?;
    let zshrc = format!("{}/.zshrc", system.get_home_dir());
    println!("Creating zshrc at {}", zshrc);
    let mut zshrc_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&zshrc)?;
    writeln!(zshrc_file, "export ZSH=$HOME/.oh-my-zsh")?;
    writeln!(zshrc_file, "ZSH_THEME=\"robbyrussell\"")?;
    writeln!(zshrc_file, "plugins=(common-aliases docker docker-compose git git-flow gradle jira kubectl mvn pip web-search)")?;
    writeln!(zshrc_file, "export PATH=\"/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:${{HOME}}/bin:${{HOME}}/.local/bin\"")?;
    writeln!(zshrc_file, "source $ZSH/oh-my-zsh.sh")?;
    writeln!(zshrc_file, "")?;
    writeln!(zshrc_file, "function gfp() {{")?;
    writeln!(zshrc_file, "    for i in `git remote`; do")?;
    writeln!(zshrc_file, "        git fetch --prune $i")?;
    writeln!(zshrc_file, "    done")?;
    writeln!(zshrc_file, "}}")?;
    writeln!(zshrc_file, "")?;
    writeln!(zshrc_file, "function gco() {{")?;
    writeln!(zshrc_file, "    git checkout")?;
    writeln!(zshrc_file, "}}")?;
    writeln!(zshrc_file, "")?;
    writeln!(zshrc_file, "function gpod() {{")?;
    writeln!(zshrc_file, "    git pull origin develop")?;
    writeln!(zshrc_file, "}}")?;
    writeln!(zshrc_file, "")?;
    writeln!(zshrc_file, "function grprt() {{")?;
    writeln!(zshrc_file, "    lsof -i :$1 -S")?;
    writeln!(zshrc_file, "}}")?;
    writeln!(zshrc_file, "")?;
    writeln!(zshrc_file, "function gitCurrentBranch() {{")?;
    writeln!(zshrc_file, "    git rev-parse --abbrev-ref HEAD")?;
    writeln!(zshrc_file, "}}")?;
    writeln!(zshrc_file, "")?;
    writeln!(zshrc_file, "function gitGraph() {{")?;
    writeln!(zshrc_file, "    git log --graph --oneline --all")?;
    writeln!(zshrc_file, "}}")?;
    writeln!(zshrc_file, "")?;
    writeln!(zshrc_file, "function gitDummyCommit() {{")?;
    writeln!(zshrc_file, "    git commit --allow-empty -m ${{1}}")?;
    writeln!(zshrc_file, "}}")?;
    writeln!(zshrc_file, "")?;
    writeln!(zshrc_file, "function gitDeleteRemote() {{")?;
    writeln!(zshrc_file, "    git push -d origin ${{1}}")?;
    writeln!(zshrc_file, "}}")?;
    writeln!(zshrc_file, "")?;
    writeln!(zshrc_file, "function gitDeleteLocal() {{")?;
    writeln!(zshrc_file, "    git branch -d ${{1}}")?;
    writeln!(zshrc_file, "}}")?;
    writeln!(zshrc_file, "")?;
    writeln!(zshrc_file, "function gitDeleteUntracked() {{")?;
    writeln!(zshrc_file, "    git fetch -p && for branch in $(git for-each-ref --format '%(refname) %(upstream:track)' refs/heads | awk '$2 == \"[gone]\" {{sub(\"refs/heads/\", \"\", $1); print $1}}'); do git branch -D $branch; done")?;
    writeln!(zshrc_file, "}}")?;
    writeln!(zshrc_file, "")?;
    writeln!(zshrc_file, "function migrateGitRepo() {{")?;
    writeln!(zshrc_file, "    if [ -z $1 ]; then")?;
    writeln!(zshrc_file, "        echo \"Please provide the new git repo URL\"")?;
    writeln!(zshrc_file, "        return")?;
    writeln!(zshrc_file, "    fi")?;
    writeln!(zshrc_file, "")?;
    writeln!(
        zshrc_file,
        "    for remote in `git branch -r | grep -v master `; do"
    )?;
    writeln!(zshrc_file, "        git checkout --track $remote")?;
    writeln!(zshrc_file, "    done")?;
    writeln!(zshrc_file, "")?;
    writeln!(zshrc_file, "    git remote rm origin")?;
    writeln!(zshrc_file, "    git remote add origin $1")?;
    writeln!(zshrc_file, "    git remote show origin")?;
    writeln!(zshrc_file, "    git push origin '*:*'")?;
    writeln!(zshrc_file, "}}")?;
    writeln!(zshrc_file, "")?;
    writeln!(zshrc_file, "if [ -f $HOME/.zshrc.custom ]; then")?;
    writeln!(zshrc_file, "    source $HOME/.zshrc.custom")?;
    writeln!(zshrc_file, "fi")?;
    writeln!(zshrc_file, "")?;

    let user_id = get_user_id();
    let group_id = get_group_id();
    recursively_chown(
        &zshrc,
        &user_id,
        &group_id,
    )?;

    let zshrc_custom = format!("{}/.zshrc.custom", system.get_home_dir());
    let zshrc_custom_path = std::path::Path::new(&zshrc_custom);
    if !zshrc_custom_path.exists() {
        println!("Creating zshrc custom at {}", zshrc_custom);
        let mut zshrc_custom_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(zshrc_custom_path)?;
        writeln!(
            zshrc_custom_file,
            "# File to contain custom config that won't get overwritten"
        )?;
        writeln!(zshrc_custom_file, "")?;

        let user_id = get_user_id();
        let group_id = get_group_id();
        recursively_chown(
            &zshrc_custom,
            &user_id,
            &group_id,
        )?;
    }
    Ok(())
}

pub(crate) fn symlink(system: &impl System, source: &str, destination: &str) -> Result<String, Box<dyn std::error::Error>> {
    system.execute(&format!("ln -sfn {} {}", source, destination), true)
}
