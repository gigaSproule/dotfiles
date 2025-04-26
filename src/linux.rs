use std::ffi::CStr;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use tar::Archive;

use crate::system::System;
use crate::system::{self, file_contains};
use crate::unix;

/// Returns the vendor ID of the CPU for the machine.
///
/// The possible values are `GenuineIntel` for Intel CPUs and `AuthenticAMD` for AMD CPUs.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use linux;
///
/// let cpu_name = linux::get_cpu_name();
/// ```
pub(crate) fn get_cpu_name() -> Option<String> {
    let file = File::open("/proc/cpuinfo");
    if let Ok(unwrapped_file) = file {
        let buffer = BufReader::new(unwrapped_file);
        let cpu_name = buffer.lines().find_map(|line| {
            if let Ok(unwrapped_line) = line {
                if unwrapped_line.starts_with("vendor_id") {
                    return Some(unwrapped_line.split(':').next()?.to_string());
                }
            }
            None
        });
        return cpu_name;
    }
    None
}

/// Returns the users home directory _without_ the trailing slash.
/// When using $HOME or other methods, on Linux, it returns `/root` rather than the actual user's
/// home directory, so have to utilise this approach
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use linux;
///
/// let home_dir = linux::get_home_dir();
/// ```
pub(crate) fn get_home_dir() -> String {
    unsafe {
        let passwd = libc::getpwuid(unix::get_user_id());
        if passwd.is_null() {
            panic!("User not found not found");
        }
        let pwd_dir = (*passwd).pw_dir;
        if pwd_dir.is_null() {
            panic!("User doesn't have a home directory");
        }
        CStr::from_ptr(pwd_dir).to_str().unwrap().to_string()
    }
}

pub(crate) fn gnome_development_shortcuts(
    system: &dyn System,
) -> Result<(), Box<dyn std::error::Error>> {
    system.execute(
        r#"gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-up "[]""#,
        false,
    )?;
    system.execute(
        r#"gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-down "[]""#,
        false,
    )?;
    system.execute(
        r#"gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-left "[]""#,
        false,
    )?;
    system.execute(
        r#"gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-right "[]""#,
        false,
    )?;
    system.execute(
        r#"gsettings set org.gnome.desktop.wm.keybindings begin-move "[]""#,
        false,
    )?;
    system.execute(r#"gsettings set org.gnome.shell.extensions.screenshot-window-sizer cycle-screenshot-sizes "[]""#, false)?;
    Ok(())
}

pub(crate) fn set_development_environment_settings() -> Result<(), std::io::Error> {
    println!("Setting mmapfs limit for Elasticsearch");
    system::add_to_file("/etc/sysctl.conf", "vm.max_map_count=262144")?;
    Ok(())
}

pub(crate) fn setup_davinci_resolve(system: &dyn System) -> Result<(), std::io::Error> {
    println!("Setting up DaVinci Resolve helper scripts");

    let convert_audio = format!("{}/bin/convert_audio", system.get_home_dir());
    let mut convert_audio_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&convert_audio)?;

    write!(
        convert_audio_file,
        r##"#!/usr/bin/env bash
set -e
shopt -s extglob nullglob
directory=$1
backup_dir="$directory/original"
extensions="${{@:2}}"
extensions="${{extensions:-m4a aac}}"
echo $extensions
if [ ! -d "$backup_dir" ]; then
    echo "Creating $backup_dir directory."
    mkdir "$backup_dir"
fi

for ext in $extensions; do
    for audio in "$directory"/*.$ext; do
        ffmpeg -i "$audio" -f flac "converted.flac"
        filename=$(basename \"$audio\")
        noext="${{filename%.$ext}}"
        echo $noext
        mv "$audio" "$backup_dir"
        mv "converted.flac" "$directory/${{noext// /_}}.flac"
    done
done
"##
    )?;
    unix::recursively_chmod(&convert_audio, &0o755, &0o755)?;

    let convert_video = format!("{}/bin/convert_videos", system.get_home_dir());
    let mut convert_video_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&convert_video)?;

    write!(
        convert_video_file,
        r##"#!/usr/bin/env bash

set -e
video=$1
codec=${{2:-pcm_s16le}}
container=${{3:-mov}}
directory="$(basename "$(dirname "$video")")"
backup_dir="$directory/original"

if [ ! -d "$backup_dir" ]; then
    echo "Creating $backup_dir directory."
    mkdir "$backup_dir"
fi

ffmpeg -i "$video" -acodec "$codec" -vcodec copy "converted.$container"
filename=$(basename "$video")
extension="${{filename##*.}}"
noext="${{filename%.$extension}}"
echo $noext
mv "$video" "$backup_dir"
mv "converted.$container" "$directory/${{noext// /_}}.$container"
"##
    )?;
    unix::recursively_chmod(&convert_video, &0o755, &0o755)?;
    let convert_videos = format!("{}/bin/convert_videos", system.get_home_dir());

    let mut convert_videos_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&convert_videos)?;

    write!(
        convert_videos_file,
        r##"#!/usr/bin/env bash

set -e
shopt -s extglob nullglob
directory=${{1:-.}}
backup_dir="$directory/original"
extensions="${{@:2}}"
extensions="${{extensions:-mp4 MP4}}"
echo $extensions

for ext in $extensions; do
    for video in "$directory"/*.$ext; do
        convert_video "$video" pcm_s16le mov
    done
done
"##
    )?;
    unix::recursively_chmod(&convert_videos, &0o755, &0o755)?;
    Ok(())
}

pub(crate) fn setup_docker(system: &dyn System) -> Result<(), Box<dyn std::error::Error>> {
    system.execute(
        format!("usermod -a -G docker {}", unix::get_username()).as_str(),
        true,
    )?;
    Ok(())
}

pub(crate) fn setup_nas(system: &impl System) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating NAS group");
    system.execute("groupadd nas", true)?;
    system.execute(&format!("usermod -a -G nas {}", unix::get_username()), true)?;

    println!("Setting up NAS scripts");
    let smb_credentials = format!("{}/.smbcredentials", system.get_home_dir());
    if !Path::new(&smb_credentials).exists() {
        let mut smb_credentials_file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(smb_credentials)?;

        writeln!(smb_credentials_file, "username=")?;
        writeln!(smb_credentials_file, "password=")?;
        writeln!(smb_credentials_file)?;
    }

    let user_id = unix::get_user_id();
    let group_id = unix::get_group_id_by_name("name");

    let benjamin_mount = "/mnt/benjamin";
    if !Path::new(benjamin_mount).exists() {
        fs::create_dir_all(benjamin_mount)?;
        unix::recursively_chown(benjamin_mount, &user_id, &group_id)?;
    }
    let music_mount = "/mnt/music";
    if !Path::new(music_mount).exists() {
        fs::create_dir_all(music_mount)?;
        unix::recursively_chown(music_mount, &user_id, &group_id)?;
    }
    let photo_mount = "/mnt/photo";
    if !Path::new(photo_mount).exists() {
        fs::create_dir_all(photo_mount)?;
        unix::recursively_chown(photo_mount, &user_id, &group_id)?;
    }
    let shared_mount = "/mnt/shared";
    if !Path::new(shared_mount).exists() {
        fs::create_dir_all(shared_mount)?;
        unix::recursively_chown(shared_mount, &user_id, &group_id)?;
    }
    let videos_mount = "/mnt/videos";
    if !Path::new(videos_mount).exists() {
        fs::create_dir_all(videos_mount)?;
        unix::recursively_chown(videos_mount, &user_id, &group_id)?;
    }

    let mount_nas = format!("{}/bin/mount-nas", system.get_home_dir());
    let mut mount_nas_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&mount_nas)?;

    writeln!(mount_nas_file, "#!/usr/bin/env bash")?;
    writeln!(mount_nas_file, "sudo mount -t cifs -o rw,uid=$(id -u),gid=$(id -g),credentials=/home/benjamin/.smbcredentials,vers=3.0 //192.168.1.225/homes/benjamin {}", benjamin_mount)?;
    writeln!(mount_nas_file, "sudo mount -t cifs -o rw,uid=$(id -u),gid=$(id -g),credentials=/home/benjamin/.smbcredentials,vers=3.0 //192.168.1.225/music {}", music_mount)?;
    writeln!(mount_nas_file, "sudo mount -t cifs -o rw,uid=$(id -u),gid=$(id -g),credentials=/home/benjamin/.smbcredentials,vers=3.0 //192.168.1.225/photo {}", photo_mount)?;
    writeln!(mount_nas_file, "sudo mount -t cifs -o rw,uid=$(id -u),gid=$(id -g),credentials=/home/benjamin/.smbcredentials,vers=3.0 //192.168.1.225/shared {}", shared_mount)?;
    writeln!(mount_nas_file, "sudo mount -t cifs -o rw,uid=$(id -u),gid=$(id -g),credentials=/home/benjamin/.smbcredentials,vers=3.0 //192.168.1.225/video {}", videos_mount)?;
    writeln!(mount_nas_file)?;
    unix::recursively_chmod(&mount_nas, &0o755, &0o755)?;

    let unmount_nas = format!("{}/bin/unmount-nas", system.get_home_dir());
    let mut unmount_nas_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&unmount_nas)?;

    writeln!(unmount_nas_file, "#!/usr/bin/env bash")?;
    writeln!(unmount_nas_file, "sudo umount {}", benjamin_mount)?;
    writeln!(unmount_nas_file, "sudo umount {}", music_mount)?;
    writeln!(unmount_nas_file, "sudo umount {}", photo_mount)?;
    writeln!(unmount_nas_file, "sudo umount {}", shared_mount)?;
    writeln!(unmount_nas_file, "sudo umount {}", videos_mount)?;
    writeln!(unmount_nas_file)?;
    unix::recursively_chmod(&unmount_nas, &0o755, &0o755)?;

    Ok(())
}

pub(crate) fn setup_nodejs(system: &dyn System) -> Result<(), Box<dyn std::error::Error>> {
    let nvm_content = "export NVM_DIR=\"$([ -z \"${{XDG_CONFIG_HOME-}}\" ] && printf %s \"${{HOME}}/.nvm\" || printf %s \"${{XDG_CONFIG_HOME}}/nvm\")\"\n\
    [ -s \"$NVM_DIR/nvm.sh\" ] && \\. \"$NVM_DIR/nvm.sh\" # This loads nvm";
    system::add_to_file(&format!("{}/.zshrc", system.get_home_dir()), nvm_content)?;
    system::add_to_file(&format!("{}/.bashrc", system.get_home_dir()), nvm_content)?;

    let zsh_nvm_dir = "autoload -U add-zsh-hook\n\
        load-nvmrc() {\n\
            local node_version=\"$(nvm version)\"\n\
            local nvmrc_path=\"$(nvm_find_nvmrc)\"\n\
            if [ -n \"$nvmrc_path\" ]; then\n\
                local nvmrc_node_version=$(nvm version \"$(cat \"${nvmrc_path}\")\")\n\
                if [ \"$nvmrc_node_version\" = \"N/A\" ]; then\n\
                    nvm install\n\
                elif [ \"$nvmrc_node_version\" != \"$node_version\" ]; then\n\
                    nvm use\n\
                fi\n\
            elif [ \"$node_version\" != \"$(nvm version default)\" ]; then\n\
                echo \"Reverting to nvm default version\"\n\
                nvm use default\n\
            fi\n\
        }\n\
        add-zsh-hook chpwd load-nvmrc\n\
        load-nvmrc";
    system::add_to_file(&format!("{}/.zshrc", system.get_home_dir()), zsh_nvm_dir)?;

    let bash_nvm_dir = "cdnvm() {\n\
            command cd \"$@\";\n\
            nvm_path=$(nvm_find_up .nvmrc | tr -d '\n')\n\
            # If there are no .nvmrc file, use the default nvm version\n\
            if [[ ! $nvm_path = *[^[:space:]]* ]]; then\n\
                declare default_version;\n\
                default_version=$(nvm version default);\n\
                # If there is no default version, set it to `node`\n\
                # This will use the latest version on your machine\n\
                if [[ $default_version == \"N/A\" ]]; then\n\
                    nvm alias default node;\n\
                    default_version=$(nvm version default);\n\
                fi\n\
                # If the current version is not the default version, set it to use the default version\n\
                if [[ $(nvm current) != \"$default_version\" ]]; then\n\
                    nvm use default;\n\
                fi\n\
            elif [[ -s $nvm_path/.nvmrc && -r $nvm_path/.nvmrc ]]; then\n\
                declare nvm_version\n\
                nvm_version=$(<\"$nvm_path\"/.nvmrc)\n\
                declare locally_resolved_nvm_version\n\
                # `nvm ls` will check all locally-available versions\n\
                # If there are multiple matching versions, take the latest one\n\
                # Remove the `->` and `*` characters and spaces\n\
                # `locally_resolved_nvm_version` will be `N/A` if no local versions are found\n\
                locally_resolved_nvm_version=$(nvm ls --no-colors \"$nvm_version\" | tail -1 | tr -d '\\->*' | tr -d '[:space:]')\n\
                # If it is not already installed, install it\n\
                # `nvm install` will implicitly use the newly-installed version\n\
                if [[ \"$locally_resolved_nvm_version\" == \"N/A\" ]]; then\n\
                    nvm install \"$nvm_version\";\n\
                elif [[ $(nvm current) != \"$locally_resolved_nvm_version\" ]]; then\n\
                    nvm use \"$nvm_version\";\n\
                fi\n\
            fi\n\
        }\n\
        alias cd='cdnvm'\n\
        cd \"$PWD\"";
    system::add_to_file(&format!("{}/.bashrc", system.get_home_dir()), bash_nvm_dir)?;

    system.execute("nvm install node --latest-npm", false)?;
    system.execute("npm install --global yarn", false)?;
    Ok(())
}

pub(crate) fn setup_power_saving_tweaks() -> Result<(), std::io::Error> {
    let mut file = File::open("/sys/devices/virtual/dmi/id/product_name")?;
    let mut device_name = String::new();
    file.read_to_string(&mut device_name)?;

    if device_name == "XPS 15 9570" {
        let mem_sleep = "/sys/power/mem_sleep";
        if !file_contains(mem_sleep, "s2idle [deep]") {
            let mut mem_sleep_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(mem_sleep)?;
            writeln!(mem_sleep_file, "s2idle [deep]")?;
        }

        let original_grub_file = File::open("/etc/default/grub")?;
        let buffer = BufReader::new(original_grub_file);
        let new_lines = buffer
            .lines()
            .map(|line| {
                let unwrapped_line = line.unwrap();
                if unwrapped_line.starts_with("GRUB_CMDLINE_LINUX_DEFAULT=")
                    && !unwrapped_line.contains("mem_sleep_default = deep")
                {
                    let mut split_line = unwrapped_line.split('=');
                    split_line.next();
                    let unwrapped_next_split = split_line.next().unwrap();
                    let mut value = unwrapped_next_split.replace('\"', "");
                    value += "mem_sleep_default = deep";
                    format!("GRUB_CMDLINE_LINUX_DEFAULT=\"{}\"", value)
                } else {
                    unwrapped_line
                }
            })
            .collect::<Vec<String>>();

        let mut new_grub_file = OpenOptions::new().append(true).open("/etc/default/grub")?;
        new_grub_file.write_all(new_lines.join("\n").as_bytes())?;
    }
    Ok(())
}

pub(crate) fn setup_tmux(system: &impl System) -> Result<(), std::io::Error> {
    unix::setup_tmux(system)?;
    system::add_to_file(
        &format!("{}/.tmux.conf", system.get_home_dir()),
        "bind -T copy-mode-vi y send-keys -X copy-pipe-and-cancel 'xclip -in -selection clipboard'",
    )?;
    Ok(())
}

/// Extracts the contents of the tar file and renames the root directory.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use linux;
///
/// linux::untar_rename_root("/path/to/tar", "/path/to/dest");
/// ```
pub(crate) fn untar_rename_root(src: &str, dest: &str) -> Result<(), std::io::Error> {
    let file = File::open(src)?;
    let mut archive = Archive::new(GzDecoder::new(file));

    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf, std::io::Error> {
            let stripped_path: PathBuf = entry.path()?.iter().skip(1).collect();
            let path = Path::new(dest).join(stripped_path);
            entry.unpack(&path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));
    Ok(())
}

pub(crate) fn gtk_theme(system: &impl System) -> Result<(), std::io::Error> {
    let theme = "@import \"colors.css\";\n\
\n\
window.ssd headerbar.titlebar {\n\
  padding-top: 2px;\n\
  padding-bottom: 2px;\n\
  min-height: 0;\n\
}\n\
\n\
window.ssd headerbar.titlebar button.titlebutton {\n\
  padding-top: 2px;\n\
  padding-bottom: 2px;\n\
  min-height: 0;\n\
}\n\
\n\
/* shrink headebars */\n\
headerbar {\n\
  min-height: 38px;\n\
  /* same as childrens vertical margins for nicer proportions */\n\
  padding-left: 2px;\n\
  padding-right: 2px;\n\
}\n\
\n\
headerbar entry,\n\
headerbar spinbutton,\n\
headerbar button,\n\
headerbar separator {\n\
  /* same as headerbar side padding for nicer proportions */\n\
  margin-top: 2px;\n\
  margin-bottom: 2px;\n\
}\n\
\n\
/* shrink ssd titlebars */\n\
.default-decoration {\n\
  /* let the entry and button drive the titlebar size */\n\
  min-height: 0;\n\
  padding: 2px;\n\
}\n\
\n\
.default-decoration .titlebutton {\n\
  /* tweak these two props to reduce button size */\n\
  min-height: 26px;\n\
  min-width: 26px;\n\
}\n";
    system::add_to_file(
        &format!("{}/.config/gtk-3.0/gtk.css", system.get_home_dir()),
        theme,
    )?;
    Ok(())
}
