use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Output;

use async_trait::async_trait;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::arch::Arch;
use crate::system;
use crate::system::System;
use crate::ubuntu::Ubuntu;
use crate::unix;

pub(crate) struct Linux {
    distro: Box<dyn System>,
}

impl Default for Linux {
    fn default() -> Self {
        let sudo_user = std::env::var("SUDO_USER");
        if sudo_user.is_err() {
            panic!("Need to run this with sudo.")
        }
        let distro_str = whoami::distro();
        let distro: Box<dyn System> = match distro_str {
            distro if distro == "Arch Linux" => Box::new(Arch {}),
            distro if distro.starts_with("Ubuntu") => Box::new(Ubuntu {}),
            _ => panic!("Unable to determine the distro {}.", distro_str),
        };
        Linux {
            distro,
        }
    }
}

#[async_trait]
impl System for Linux {
    fn execute(&self, command: &str, super_user: bool) -> Output {
        self.distro.execute(command, super_user)
    }

    fn install_applications(&self, applications: Vec<&str>) -> Output {
        self.distro.install_applications(applications)
    }

    fn install_android_studio(&self) {
        self.distro.install_android_studio();
    }

    fn install_blender(&self) {
        self.distro.install_blender();
    }

    fn install_bluetooth(&self) {
        self.distro.install_bluetooth();
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let install_codecs = self.distro.install_codecs();
        install_codecs.await
    }

    fn install_conemu(&self) {
        self.distro.install_conemu();
    }

    fn install_cryptomator(&self) {
        self.distro.install_cryptomator();
    }

    fn install_curl(&self) {
        self.distro.install_curl();
    }

    fn install_davinci_resolve(&self) -> Result<(), Error> {
        self.distro.install_davinci_resolve()
    }

    fn install_discord(&self) {
        self.distro.install_discord();
    }

    fn install_docker(&self) -> Result<(), Error> {
        self.distro.install_docker()
    }

    fn install_dropbox(&self) {
        self.distro.install_dropbox();
    }

    async fn install_eclipse(&self) -> Result<(), Box<dyn std::error::Error>> {
        let install_eclipse = self.distro.install_eclipse();
        install_eclipse.await
    }

    fn install_epic_games(&self) {
        self.distro.install_epic_games();
    }

    fn install_firefox(&self) {
        self.distro.install_firefox();
    }

    fn install_firmware_updater(&self) {
        self.distro.install_firmware_updater();
    }

    fn install_gog_galaxy(&self) {
        self.distro.install_gog_galaxy();
    }

    async fn install_google_chrome(&self) -> Result<(), Box<dyn std::error::Error>> {
        let install_google_chrome = self.distro.install_google_chrome();
        install_google_chrome.await
    }

    fn install_google_cloud_sdk(&self) -> Result<(), std::io::Error> {
        self.distro.install_google_cloud_sdk()
    }

    fn install_google_drive(&self) {
        self.distro.install_google_drive();
    }

    fn install_git(&self) -> Result<(), Error> {
        self.distro.install_git()
    }

    fn install_gimp(&self) {
        self.distro.install_gimp();
    }

    fn install_gpg(&self) {
        self.distro.install_gpg();
    }

    fn install_gradle(&self) {
        self.distro.install_gradle();
    }

    fn install_graphic_card_tools(&self) {
        self.distro.install_graphic_card_tools();
    }

    fn install_graphic_card_laptop_tools(&self) {
        self.distro.install_graphic_card_laptop_tools();
    }

    fn install_groovy(&self) {
        self.distro.install_groovy();
    }

    fn install_handbrake(&self) {
        self.distro.install_handbrake();
    }

    fn install_inkscape(&self) {
        self.distro.install_inkscape();
    }

    fn install_insync(&self) {
        self.distro.install_insync();
    }

    fn install_intellij(&self) {
        self.distro.install_intellij();
    }

    fn install_jdk(&self) -> Result<(), std::io::Error> {
        self.distro.install_jdk()
    }

    fn install_keepassxc(&self) {
        self.distro.install_keepassxc();
    }

    async fn install_kubectl(&self) -> Result<(), Box<dyn std::error::Error>> {
        let install_kubectl = self.distro.install_kubectl();
        install_kubectl.await
    }

    fn install_helm(&self) {
        self.distro.install_helm();
    }

    fn install_latex(&self) {
        self.distro.install_latex();
    }

    fn install_lutris(&self) {
        self.distro.install_lutris();
    }

    fn install_maven(&self) {
        self.distro.install_maven();
    }

    fn install_makemkv(&self) {
        self.distro.install_makemkv();
    }

    fn install_microcode(&self) -> Result<(), std::io::Error> {
        self.distro.install_microcode()
    }

    async fn install_minikube(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_minikube().await
    }

    fn install_mkvtoolnix(&self) {
        self.distro.install_mkvtoolnix();
    }

    fn install_nextcloud_client(&self) {
        self.distro.install_nextcloud_client();
    }

    async fn install_nodejs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let install_nodejs = self.distro.install_nodejs();
        install_nodejs.await
    }

    async fn install_nordvpn(&self) -> Result<(), Box<dyn std::error::Error>> {
        let install_nordvpn = self.distro.install_nordvpn();
        install_nordvpn.await
    }

    fn install_nvidia_tools(&self) {
        self.distro.install_nvidia_tools();
    }

    fn install_nvidia_laptop_tools(&self) {
        self.distro.install_nvidia_laptop_tools();
    }

    fn install_obs_studio(&self) {
        self.distro.install_obs_studio();
    }

    fn install_onedrive(&self) {
        self.distro.install_onedrive();
    }

    fn install_origin(&self) {
        self.distro.install_origin();
    }

    fn install_powertop(&self) {
        self.distro.install_powertop();
    }

    fn install_python(&self) {
        self.distro.install_python();
    }

    async fn install_rust(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_rust().await
    }

    fn install_slack(&self) {
        self.distro.install_slack();
    }

    fn install_spotify(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_spotify()
    }

    fn install_steam(&self) {
        self.distro.install_steam();
    }

    fn install_sweet_home_3d(&self) {
        self.distro.install_sweet_home_3d();
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn std::error::Error>> {
        let install_system_extras = self.distro.install_system_extras();
        install_system_extras.await
    }

    fn install_telnet(&self) {
        self.distro.install_telnet();
    }

    async fn install_themes(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_themes().await
    }

    fn install_tlp(&self) {
        self.distro.install_tlp();
    }

    fn install_tmux(&self) -> Result<(), std::io::Error> {
        self.distro.install_tmux()
    }

    fn install_vim(&self) {
        self.distro.install_vim();
    }

    fn install_vlc(&self) {
        self.distro.install_vlc();
    }

    fn install_vm_tools(&self) {
        self.distro.install_vm_tools();
    }

    fn install_vscode(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_vscode()
    }

    async fn install_wifi(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_wifi().await
    }

    fn install_window_manager(&self) {
        self.distro.install_window_manager();
    }

    fn install_wget(&self) {
        self.distro.install_wget();
    }

    fn install_wine(&self) {
        self.distro.install_wine();
    }

    fn install_xcode(&self) {
        self.distro.install_xcode();
    }

    async fn install_zsh(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_zsh().await?;
        Ok(())
    }

    fn set_development_shortcuts(&self) {
        self.distro.set_development_shortcuts();
    }

    fn set_development_environment_settings(&self) -> Result<(), std::io::Error> {
        self.distro.set_development_environment_settings()
    }

    fn setup_power_saving_tweaks(&self) -> Result<(), std::io::Error> {
        self.distro.setup_power_saving_tweaks()
    }

    fn update_os(&self) {
        self.distro.update_os();
    }

    fn update_os_repo(&self) {
        self.distro.update_os_repo();
    }
}

pub(crate) fn gnome_development_shortcuts(system: &dyn System) {
    system.execute("gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-up []", false);
    system.execute("gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-down []", false);
    system.execute("gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-left []", false);
    system.execute("gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-right []", false);
    system.execute("gsettings set org.gnome.desktop.wm.keybindings begin-move []", false);
    system.execute("gsettings set org.gnome.shell.extensions.screenshot-window-sizer cycle-screenshot-sizes []", false);
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

pub(crate) fn setup_docker(system: &dyn System) {
    system.execute(
        format!("usermod -a -G docker {}", unix::get_username()).as_str(),
        true,
    );
}

pub(crate) fn setup_power_saving_tweaks() -> Result<(), std::io::Error> {
    let mut file = File::open("/sys/devices/virtual/dmi/id/product_name")?;
    let mut device_name = String::new();
    file.read_to_string(&mut device_name)?;

    if device_name == "XPS 15 9570" {
        let mut mem_sleep_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("/sys/power/mem_sleep")?;
        writeln!(mem_sleep_file, "s2idle [deep]")?;

        let original_grub_file = File::open("/etc/default/grub")?;
        let buffer = BufReader::new(original_grub_file);
        let new_lines = buffer.lines().map(|line| {
            if line.as_ref().unwrap().starts_with("GRUB_CMDLINE_LINUX_DEFAULT=") {
                let unwrapped_line = line.unwrap();
                let mut split_line = unwrapped_line.split('=');
                split_line.next();
                let unwrapped_next_split = split_line.next().unwrap();
                let mut value = unwrapped_next_split.replace("\"", "");
                value += "mem_sleep_default = deep";
                format!("{}=\"{}\"", "GRUB_CMDLINE_LINUX_DEFAULT", value)
            } else {
                line.unwrap()
            }
        }).collect::<Vec<String>>();

        let mut new_grub_file = OpenOptions::new().append(true).open("/etc/default/grub")?;
        new_grub_file.write_all(new_lines.join("\n").as_bytes())?;
    }
    Ok(())
}

pub(crate) fn setup_tmux() -> Result<(), std::io::Error> {
    unix::setup_tmux()?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{}/.tmux.custom.conf", system::get_home_dir()))?;
    writeln!(file, "bind -T copy-mode-vi y send-keys -X copy-pipe-and-cancel 'xclip -in -selection clipboard'")?;
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
