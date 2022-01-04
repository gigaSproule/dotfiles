use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
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

pub(crate) fn add_to_path(file: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if file_contains(file, "export PATH") {
        let original_file = File::open(file)?;
        let original_lines = BufReader::new(original_file).lines();
        let new_lines = original_lines
            .map(|line| {
                let unwrapped_line = line.unwrap();
                return if unwrapped_line.starts_with("export PATH") {
                    format!("export PATH=$PATH:{}", path)
                } else {
                    unwrapped_line
                };
            })
            .collect::<Vec<String>>();

        let mut new_file = OpenOptions::new().write(true).open(file)?;
        new_file.write_all(new_lines.join("\n").as_bytes())?;
    } else {
        let mut append_file = OpenOptions::new().create(true).append(true).open(format!(
            "{}/{}",
            get_home_dir(),
            file
        ))?;
        writeln!(append_file, "export PATH = $PATH:{}\n", path)?;
    }
    std::env::set_var("PATH", format!("{}:{}", std::env::var("PATH")?, path));
    Ok(())
}

pub(crate) fn execute(
    command: &str,
    super_user: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    execute_path(
        command,
        super_user,
        &env::current_dir()
            .expect("Could not get current directory")
            .into_os_string()
            .into_string()
            .expect("Could not convert current directory path to a string"),
    )
}

pub(crate) fn execute_path(
    command: &str,
    super_user: bool,
    path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut actual_command = if !super_user {
        let split = command.split_whitespace().collect::<Vec<&str>>();
        let sudo_user = get_username();
        let mut args = vec!["-u", &sudo_user];
        args.extend(split);
        let mut return_command = Command::new("sudo");
        return_command.args(&args);
        let joined = args.join(" ");
        println!("sudo {}", &joined);
        return_command
    } else {
        let mut split = command.split_whitespace();
        let mut return_command = Command::new(
            split
                .nth(0)
                .expect("Could not find the first part of the command"),
        );
        return_command.args(split.collect::<Vec<&str>>());
        println!("{}", command);
        return_command
    };

    let child = actual_command.current_dir(path);
    system::run_command(child)
}

pub(crate) fn get_home_dir() -> String {
    let passwd_entry = execute(&format!("getent passwd {}", get_username()), true).unwrap();
    passwd_entry.split(":").nth(5).unwrap().to_string()
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

pub(crate) fn set_java_home(file: &str, jdk_path: &str) -> Result<(), std::io::Error> {
    let path = format!("{}/{}", get_home_dir(), file);
    println!("Appending JAVA_HOME as {} to {}", jdk_path, &path);

    if file_contains(&path, "JAVA_HOME") {
        let mut actual_file = OpenOptions::new().create(true).append(true).open(&path)?;
        writeln!(actual_file, "export JAVA_HOME={}", jdk_path)?;
    }
    std::env::set_var("JAVA_HOME", jdk_path);
    Ok(())
}

fn file_contains(file: &str, contains: &str) -> bool {
    let file_result = OpenOptions::new().read(true).open(file);
    if file_result.is_err() {
        return false;
    }
    let mut actual_file = file_result.unwrap();
    if actual_file.metadata().is_err() {
        return false;
    }
    let mut buff = String::new();
    let result = actual_file.read_to_string(&mut buff);
    if result.is_err() {
        return false;
    }
    buff.contains(contains)
}

pub(crate) fn setup_tmux() -> Result<(), std::io::Error> {
    let tmux_conf = format!("{}/.tmux.conf", get_home_dir());
    println!("Creating tmux conf at {}", tmux_conf);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(tmux_conf)?;
    writeln!(file, "# set command prefix for tmux")?;
    writeln!(file, "set-option -g prefix C-a")?;
    writeln!(file, "unbind C-a")?;
    writeln!(file, "bind-key C-a send-prefix")?;
    writeln!(file, "")?;
    writeln!(file, "# set vi mode keys")?;
    writeln!(file, "setw -g mode-keys vi")?;
    writeln!(file, "")?;
    writeln!(
        file,
        "# set some bindings for moving around terminals (vim-like)"
    )?;
    writeln!(file, "bind h select-pane -L")?;
    writeln!(file, "bind j select-pane -D")?;
    writeln!(file, "bind k select-pane -U")?;
    writeln!(file, "bind l select-pane -R")?;
    writeln!(file, "")?;
    writeln!(file, "bind C-M-h resize-pane -L 5")?;
    writeln!(file, "bind C-h resize-pane -L 1")?;
    writeln!(file, "bind C-M-j resize-pane -D 5")?;
    writeln!(file, "bind C-j resize-pane -D 1")?;
    writeln!(file, "bind C-M-k resize-pane -U 5")?;
    writeln!(file, "bind C-k resize-pane -U 1")?;
    writeln!(file, "bind C-M-l resize-pane -R 5")?;
    writeln!(file, "bind C-l resize-pane -R 1")?;
    writeln!(file, "")?;
    writeln!(file, "# Define my custom menu bar")?;
    writeln!(file, "# status bar colors")?;
    writeln!(file, "set -g status-bg black")?;
    writeln!(file, "set -g status-fg white")?;
    writeln!(file, "")?;
    writeln!(file, "# alignment settings")?;
    writeln!(file, "set-option -g status-justify centre")?;
    writeln!(file, "")?;
    writeln!(file, "# status left options")?;
    writeln!(
        file,
        "set-option -g status-left '#[fg=green][#[bg=black,fg=cyan]#S#[fg=green]]'"
    )?;
    writeln!(file, "set-option -g status-left-length 20")?;
    writeln!(file, "")?;
    writeln!(file, "# window list options")?;
    writeln!(file, "setw -g automatic-rename on")?;
    writeln!(file, "set-window-option -g window-status-format '#[fg=cyan,dim]#I#[fg=blue]:#[default]#W#[fg=grey,dim]#F'")?;
    writeln!(file, "set-window-option -g window-status-current-format '#[bg=blue,fg=cyan,bold]#I#[bg=blue,fg=cyan]:#[fg=colour230]#W#[fg=dim]#F'")?;
    writeln!(file, "set -g base-index 1")?;
    writeln!(file, "")?;
    writeln!(file, "# status right options")?;
    writeln!(file, "set -g status-right '#[fg=green][#[fg=blue]%Y-%m-%d #[fg=white]%H:%M#[default]  #($HOME/bin/battery)#[fg=green]]'")?;
    writeln!(file, "")?;
    writeln!(file, "# bind a reload key")?;
    writeln!(
        file,
        "bind R source-file ~/.tmux.conf \\; display-message \"  Config reloaded..\"."
    )?;
    writeln!(file, "")?;
    writeln!(file, "# Set Copy-Mode settings")?;
    writeln!(file, "bind [ copy-mode")?;
    writeln!(file, "#bind -T vi-copy v begin-selection")?;
    writeln!(file, "#bind -T vi-copy y copy-selection")?;
    writeln!(file, "#bind -T vi-copy V rectangle-toggle")?;
    writeln!(file, "bind ] paste-buffer")?;
    writeln!(file, "")?;
    writeln!(file, "# buffer")?;
    writeln!(file, "bind Space choose-buffer")?;
    writeln!(file, "")?;
    writeln!(file, "set -g mouse on")?;
    writeln!(
        file,
        "bind m set-option -g mouse on \\; display 'Mouse: ON'"
    )?;
    writeln!(
        file,
        "bind M set-option -g mouse off \\; display 'Mouse: OFF'"
    )?;
    writeln!(file, "bind -n WheelUpPane if-shell -F -t = \"#{{mouse_any_flag}}\" \"send-keys -M\" \"if -Ft= '#{{pane_in_mode}}' 'send-keys -M' 'select-pane -t=; copy-mode -e; send-keys -M'\"")?;
    writeln!(
        file,
        "bind -n WheelDownPane select-pane -t= \\; send-keys -M"
    )?;
    writeln!(file, "#bind -T vi-copy    C-WheelUpPane   halfpage-up")?;
    writeln!(file, "#bind -T vi-copy    C-WheelDownPane halfpage-down")?;
    writeln!(file, "")?;
    writeln!(file, "if-shell -b '[ -f $HOME/.tmux.custom.conf ]' \\")?;
    writeln!(file, "    \"source-file ~/.tmux.custom.conf\"")?;
    writeln!(file, "")?;
    Ok(())
}

pub(crate) async fn setup_zsh(
    system: &dyn System,
    zsh_bin: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let zsh = zsh_bin.unwrap_or("/usr/bin/zsh");
    system::download_file(
        "https://raw.githubusercontent.com/loket/oh-my-zsh/feature/batch-mode/tools/install.sh",
        "oh-my-zsh.sh",
    )
    .await?;
    recursively_chmod("./oh-my-zsh.sh", &0o644, &0o644)?;
    system.execute("./oh-my-zsh.sh", false)?;
    system.execute(&format!("chsh -s {}", zsh), true)?;
    system.execute(&format!("chsh -s {} {}", zsh, get_username()), true)?;
    fs::remove_file("oh-my-zsh.sh")?;
    let zshrc = format!("{}/.zshrc", get_home_dir());
    println!("Creating zshrc at {}", zshrc);
    let mut file = OpenOptions::new().create(true).append(true).open(zshrc)?;
    writeln!(file, "export ZSH=$HOME/.oh-my-zsh")?;
    writeln!(file, "ZSH_THEME=\"robbyrussell\"")?;
    writeln!(file, "plugins=(common-aliases docker docker-compose git git-flow gradle jira kubectl mvn pip web-search)")?;
    writeln!(file, "export PATH=\"/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:${{HOME}}/bin:${{HOME}}/.local/bin\"")?;
    writeln!(file, "source $ZSH/oh-my-zsh.sh")?;
    writeln!(file, "")?;
    writeln!(file, "function gfp() {{")?;
    writeln!(file, "    for i in `git remote`; do")?;
    writeln!(file, "        git fetch --prune $i")?;
    writeln!(file, "    done")?;
    writeln!(file, "}}")?;
    writeln!(file, "")?;
    writeln!(file, "function gco() {{")?;
    writeln!(file, "    git checkout")?;
    writeln!(file, "}}")?;
    writeln!(file, "")?;
    writeln!(file, "function gpod() {{")?;
    writeln!(file, "    git pull origin develop")?;
    writeln!(file, "}}")?;
    writeln!(file, "")?;
    writeln!(file, "function grprt() {{")?;
    writeln!(file, "    lsof -i :$1 -S")?;
    writeln!(file, "}}")?;
    writeln!(file, "")?;
    writeln!(file, "function gitCurrentBranch() {{")?;
    writeln!(file, "    git rev-parse --abbrev-ref HEAD")?;
    writeln!(file, "}}")?;
    writeln!(file, "")?;
    writeln!(file, "function gitGraph() {{")?;
    writeln!(file, "    git log --graph --oneline --all")?;
    writeln!(file, "}}")?;
    writeln!(file, "")?;
    writeln!(file, "function gitDummyCommit() {{")?;
    writeln!(file, "    git commit --allow-empty -m ${{1}}")?;
    writeln!(file, "}}")?;
    writeln!(file, "")?;
    writeln!(file, "function gitDeleteRemote() {{")?;
    writeln!(file, "    git push -d origin ${{1}}")?;
    writeln!(file, "}}")?;
    writeln!(file, "")?;
    writeln!(file, "function gitDeleteLocal() {{")?;
    writeln!(file, "    git branch -d ${{1}}")?;
    writeln!(file, "}}")?;
    writeln!(file, "")?;
    writeln!(file, "function gitDeleteUntracked() {{")?;
    writeln!(file, "    git fetch -p && for branch in $(git for-each-ref --format '%(refname) %(upstream:track)' refs/heads | awk '$2 == \"[gone]\" {{sub(\"refs/heads/\", \"\", $1); print $1}}'); do git branch -D $branch; done")?;
    writeln!(file, "}}")?;
    writeln!(file, "")?;
    writeln!(file, "function migrateGitRepo() {{")?;
    writeln!(file, "    if [ -z $1 ]; then")?;
    writeln!(file, "        echo \"Please provide the new git repo URL\"")?;
    writeln!(file, "        return")?;
    writeln!(file, "    fi")?;
    writeln!(file, "")?;
    writeln!(
        file,
        "    for remote in `git branch -r | grep -v master `; do"
    )?;
    writeln!(file, "        git checkout --track $remote")?;
    writeln!(file, "    done")?;
    writeln!(file, "")?;
    writeln!(file, "    git remote rm origin")?;
    writeln!(file, "    git remote add origin $1")?;
    writeln!(file, "    git remote show origin")?;
    writeln!(file, "    git push origin '*:*'")?;
    writeln!(file, "}}")?;
    writeln!(file, "")?;
    writeln!(file, "if [ -f $HOME/.zshrc.custom ]; then")?;
    writeln!(file, "    source $HOME/.zshrc.custom")?;
    writeln!(file, "fi")?;
    writeln!(file, "")?;
    Ok(())
}

pub(crate) fn symlink(source: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    execute(&format!("ln -sfn {} {}", source, destination), true)
}
