use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::arch::Arch;
use crate::system::file_contains;
use crate::system::System;
use crate::ubuntu::Ubuntu;
use crate::unix;
use crate::unix::get_username;

pub(crate) struct Linux {
    distro: Box<dyn System>,
}

impl Default for Linux {
    fn default() -> Self {
        let sudo_user = env::var("SUDO_USER");
        if sudo_user.is_err() {
            panic!("Need to run this with sudo.")
        }
        let distro_str = whoami::distro();
        let distro: Box<dyn System> = match distro_str {
            distro if distro == "Arch Linux" => Box::new(Arch {}),
            distro if distro.starts_with("Ubuntu") => Box::new(Ubuntu {}),
            _ => panic!("Unable to determine the distro {}.", distro_str),
        };
        Linux { distro }
    }
}

#[async_trait]
impl System for Linux {
    fn execute(
        &self,
        command: &str,
        super_user: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.distro.execute(command, super_user)
    }

    fn get_home_dir(&self) -> String {
        self.distro.get_home_dir()
    }

    fn install_applications(
        &self,
        applications: Vec<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.distro.install_applications(applications)
    }

    fn install_android_studio(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_android_studio()
    }

    fn install_bash(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_bash()
    }

    fn install_blender(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_blender()
    }

    fn install_bluetooth(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_bluetooth()
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_codecs().await
    }

    fn install_conemu(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_conemu()
    }

    fn install_cryptomator(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_cryptomator()
    }

    fn install_curl(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_curl()
    }

    fn install_davinci_resolve(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_davinci_resolve()
    }

    fn install_discord(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_discord()
    }

    fn install_docker(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_docker()
    }

    fn install_dropbox(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_dropbox()
    }

    async fn install_eclipse(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_eclipse().await
    }

    fn install_epic_games(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_epic_games()
    }

    fn install_firefox(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_firefox()
    }

    fn install_firmware_updater(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_firmware_updater()
    }

    fn install_gog_galaxy(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_gog_galaxy()
    }

    async fn install_google_chrome(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_google_chrome().await
    }

    fn install_google_cloud_sdk(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_google_cloud_sdk()
    }

    fn install_google_drive(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_google_drive()
    }

    fn install_git(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_git()
    }

    fn install_gimp(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_gimp()
    }

    fn install_gpg(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_gpg()
    }

    fn install_gradle(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_gradle()
    }

    fn install_graphic_card_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_graphic_card_tools()
    }

    fn install_graphic_card_laptop_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_graphic_card_laptop_tools()
    }

    fn install_groovy(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_groovy()
    }

    fn install_handbrake(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_handbrake()
    }

    fn install_inkscape(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_inkscape()
    }

    fn install_insync(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_insync()
    }

    fn install_intellij(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_intellij()
    }

    fn install_jdk(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_jdk()
    }

    fn install_keepassxc(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_keepassxc()
    }

    async fn install_kubectl(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_kubectl().await
    }

    fn install_helm(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_helm()
    }

    fn install_latex(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_latex()
    }

    fn install_lutris(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_lutris()
    }

    fn install_maven(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_maven()
    }

    fn install_makemkv(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_makemkv()
    }

    fn install_microcode(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_microcode()
    }

    async fn install_minikube(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_minikube().await
    }

    fn install_mkvtoolnix(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_mkvtoolnix()
    }

    fn install_nextcloud_client(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_nextcloud_client()
    }

    async fn install_nodejs(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_nodejs().await
    }

    async fn install_nordvpn(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_nordvpn().await
    }

    fn install_nvidia_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_nvidia_tools()
    }

    fn install_nvidia_laptop_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_nvidia_laptop_tools()
    }

    fn install_obs_studio(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_obs_studio()
    }

    fn install_onedrive(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_onedrive()
    }

    fn install_origin(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_origin()
    }

    fn install_powertop(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_powertop()
    }

    fn install_python(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_python()
    }

    async fn install_rust(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_rust().await
    }

    fn install_slack(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_slack()
    }

    fn install_spotify(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_spotify()
    }

    fn install_steam(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_steam()
    }

    fn install_sweet_home_3d(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_sweet_home_3d()
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_system_extras().await
    }

    fn install_telnet(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_telnet()
    }

    async fn install_themes(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_themes().await
    }

    fn install_tlp(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_tlp()
    }

    fn install_tmux(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_tmux()
    }

    fn install_vim(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_vim()
    }

    fn install_vlc(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_vlc()
    }

    fn install_vm_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_vm_tools()
    }

    fn install_vscode(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_vscode()
    }

    async fn install_wifi(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_wifi().await
    }

    fn install_window_manager(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_window_manager()
    }

    fn install_wget(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_wget()
    }

    fn install_wine(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_wine()
    }

    fn install_xcode(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_xcode()
    }

    async fn install_zsh(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_zsh().await
    }

    fn set_development_shortcuts(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.set_development_shortcuts()
    }

    fn set_development_environment_settings(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.set_development_environment_settings()
    }

    fn setup_power_saving_tweaks(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.setup_power_saving_tweaks()
    }

    fn update_os(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.update_os()
    }

    fn update_os_repo(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.update_os_repo()
    }
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
/// linux::get_home_dir();
/// ```
pub(crate) fn get_home_dir(system: &impl System) -> String {
    let passwd_entry = system.execute(&format!("getent passwd {}", unix::get_username()), true).unwrap();
    passwd_entry.split(":").nth(5).unwrap().to_string()
}

pub(crate) fn gnome_development_shortcuts(
    system: &dyn System,
) -> Result<(), Box<dyn std::error::Error>> {
    system.execute(
        "gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-up []",
        false,
    )?;
    system.execute(
        "gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-down []",
        false,
    )?;
    system.execute(
        "gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-left []",
        false,
    )?;
    system.execute(
        "gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-right []",
        false,
    )?;
    system.execute(
        "gsettings set org.gnome.desktop.wm.keybindings begin-move []",
        false,
    )?;
    system.execute("gsettings set org.gnome.shell.extensions.screenshot-window-sizer cycle-screenshot-sizes []", false)?;
    Ok(())
}

pub(crate) fn set_development_environment_settings() -> Result<(), std::io::Error> {
    println!("Setting mmapfs limit for Elasticsearch");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/etc/sysctl.conf")?;
    writeln!(file, "vm.max_map_count=262144")?;
    Ok(())
}

pub(crate) fn setup_docker(system: &dyn System) -> Result<(), Box<dyn std::error::Error>> {
    system.execute(
        format!("usermod -a -G docker {}", unix::get_username()).as_str(),
        true,
    )?;
    Ok(())
}

pub(crate) fn setup_nas(system: &impl System) -> Result<(), std::io::Error> {
    println!("Setting up NAS scripts");
    let smb_credentials = format!("{}/.smbcredentials", system.get_home_dir());
    if !Path::new(&smb_credentials).exists() {
        let mut smb_credentials_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(smb_credentials)?;

        writeln!(smb_credentials_file, "username=")?;
        writeln!(smb_credentials_file, "password=")?;
        writeln!(smb_credentials_file, "")?;
    }

    let user_id = unix::get_user_id();
    let group_id = unix::get_group_id();

    let benjamin_mount = "/mnt/benjamin";
    if !Path::new(benjamin_mount).exists() {
        fs::create_dir_all(benjamin_mount)?;
        unix::recursively_chown(benjamin_mount, &user_id, &group_id)?;
    }
    let shared_mount = "/mnt/shared";
    if !Path::new(shared_mount).exists() {
        fs::create_dir_all(shared_mount)?;
        unix::recursively_chown(shared_mount, &user_id, &group_id)?;
    }

    let mount_nas = format!("{}/bin/mount-nas.sh", system.get_home_dir());
    let mut mount_nas_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&mount_nas)?;

    writeln!(mount_nas_file, "#!/bin/bash")?;
    writeln!(mount_nas_file, "sudo mount -t cifs -o rw,uid=$(id -u),gid=$(id -g),credentials=/home/benjamin/.smbcredentials,vers=1.0 //192.168.1.200/benjamin {}", benjamin_mount)?;
    writeln!(mount_nas_file, "sudo mount -t cifs -o rw,uid=$(id -u),gid=$(id -g),credentials=/home/benjamin/.smbcredentials,vers=1.0 //192.168.1.200/shared {}", shared_mount)?;
    writeln!(mount_nas_file, "")?;
    unix::recursively_chmod(&mount_nas, &0o755, &0o755)?;

    let unmount_nas = format!("{}/bin/unmount-nas.sh", system.get_home_dir());
    let mut unmount_nas_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&unmount_nas)?;

    writeln!(unmount_nas_file, "#!/bin/bash")?;
    writeln!(unmount_nas_file, "sudo umount {}", benjamin_mount)?;
    writeln!(unmount_nas_file, "sudo umount {}", shared_mount)?;
    writeln!(unmount_nas_file, "")?;
    unix::recursively_chmod(&unmount_nas, &0o755, &0o755)?;

    Ok(())
}

pub(crate) fn setup_nodejs(system: &dyn System) -> Result<(), Box<dyn std::error::Error>> {
    let mut zshrc = OpenOptions::new()
        .append(true)
        .open(format!("{}/.zshrc", system.get_home_dir()))?;
    writeln!(zshrc, "export NVM_DIR=\"$([ -z \"${{XDG_CONFIG_HOME-}}\" ] && printf %s \"${{HOME}}/.nvm\" || printf %s \"${{XDG_CONFIG_HOME}}/nvm\")\"")?;
    writeln!(
        zshrc,
        "[ -s \"$NVM_DIR/nvm.sh\" ] && \\. \"$NVM_DIR/nvm.sh\" # This loads nvm"
    )?;

    let mut bashrc = OpenOptions::new()
        .append(true)
        .open(format!("{}/.bashrc", system.get_home_dir()))?;
    writeln!(bashrc, "export NVM_DIR=\"$([ -z \"${{XDG_CONFIG_HOME-}}\" ] && printf %s \"${{HOME}}/.nvm\" || printf %s \"${{XDG_CONFIG_HOME}}/nvm\")\"")?;
    writeln!(
        bashrc,
        "[ -s \"$NVM_DIR/nvm.sh\" ] && \\. \"$NVM_DIR/nvm.sh\" # This loads nvm"
    )?;

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
                if unwrapped_line.starts_with("GRUB_CMDLINE_LINUX_DEFAULT=") && !unwrapped_line.contains("mem_sleep_default = deep") {
                    let mut split_line = unwrapped_line.split("=");
                    split_line.next();
                    let unwrapped_next_split = split_line.next().unwrap();
                    let mut value = unwrapped_next_split.replace("\"", "");
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
    let mut file = OpenOptions::new()
        .append(true)
        .open(format!("{}/.tmux.conf", system.get_home_dir()))?;
    writeln!(
        file,
        "bind -T copy-mode-vi y send-keys -X copy-pipe-and-cancel 'xclip -in -selection clipboard'"
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
