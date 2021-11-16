use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Output;

use async_trait::async_trait;

use crate::{linux, system, unix};
use crate::system::System;

pub(crate) struct Arch {}

impl Arch {
    fn aur_install_application(&self, application: &str) -> Output {
        self.aur_install_applications(vec![application])
    }

    fn aur_install_applications(&self, applications: Vec<&str>) -> Output {
        self.execute(
            &format!("yay -S --noconfirm --needed {}", applications.join(" ")),
            false,
        )
    }

    fn enable_service(&self, service: &str) {
        self.execute(
            &format!("systemctl enable service {}", service),
            true,
        );
    }

    fn execute_path(&self, command: &str, super_user: bool, path: &str) -> Output {
        unix::execute_path(command, super_user, path)
    }
}

#[async_trait]
impl System for Arch {
    fn execute(&self, command: &str, super_user: bool) -> Output {
        unix::execute(command, super_user)
    }

    fn install_applications(&self, application: Vec<&str>) -> Output {
        self.execute(
            &format!("pacman -S --noconfirm --needed {}", application.join(" ")),
            true
        )
    }

    fn install_android_studio(&self) {
        self.aur_install_application("android-studio");
    }

    fn install_blender(&self) {
        self.install_application("blender");
    }

    fn install_bluetooth(&self) {
        self.install_applications(vec!["bluez", "bluez-utils"]);
        self.enable_service("bluetooth");
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec![
            "libdvdread",
            "libdvdcss",
            "libdvdnav",
            "libbluray",
            "libaacs",
        ]);
        system::setup_codecs().await?;
        let user_id = unix::get_user_id();
        let group_id = unix::get_group_id();
        unix::recursively_chown(
            &format!("{}/.config", system::get_home_dir()),
            &user_id,
            &group_id,
        )?;
        Ok(())
    }

    fn install_conemu(&self) {
        // no-op
    }

    fn install_cryptomator(&self) {
        self.aur_install_application("cryptomator");
    }

    fn install_curl(&self) {
        self.install_application("curl");
    }

    fn install_davinci_resolve(&self) -> Result<(), std::io::Error> {
        self.aur_install_application("davinci-resolve-studio");
        Ok(())
    }

    fn install_discord(&self) {
        self.install_application("discord");
    }

    fn install_docker(&self) -> Result<(), std::io::Error> {
        self.install_application("docker");
        linux::setup_docker(self);
        Ok(())
    }

    fn install_dropbox(&self) {
        self.install_applications(vec!["dropbox", "nautilus-dropbox"]);
    }

    async fn install_eclipse(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("eclipse-jee");
        if Path::new("/opt/eclipse").exists() {
            fs::create_dir_all("/opt/eclipse")?;
        }

        system::download_file("https://projectlombok.org/downloads/lombok.jar", "/opt/eclipse/lombok.jar").await?;

        let mut file = OpenOptions::new()
            .append(true)
            .open("/opt/eclipse/eclipse.ini")?;

        writeln!(file, "-javaagent:/opt/eclipse/lombok.jar")?;
        Ok(())
    }

    fn install_epic_games(&self) {
        // no-op
    }

    fn install_firefox(&self) {
        self.install_application("firefox");
    }

    fn install_firmware_updater(&self) {
        self.install_application("fwupd");
        self.enable_service("fwupd");
    }

    fn install_gog_galaxy(&self) {
        // no-op
    }

    async fn install_google_chrome(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("google-chrome");
        Ok(())
    }

    fn install_google_cloud_sdk(&self) -> Result<(), std::io::Error> {
        self.aur_install_application("google-cloud-sdk");
        Ok(())
    }

    fn install_google_drive(&self) {
        // no-op
    }

    fn install_git(&self) -> Result<(), std::io::Error> {
        self.install_application("git");
        system::setup_git_config(self)
    }

    fn install_gimp(&self) {
        self.install_application("gimp");
    }

    fn install_gpg(&self) {
        self.install_applications(vec!["seahorse", "seahorse-nautilus"]);
    }

    fn install_gradle(&self) {
        self.install_application("gradle");
    }

    fn install_graphic_card_tools(&self) {
        // if nvidia
        self.install_nvidia_tools();
        // else
    }

    fn install_graphic_card_laptop_tools(&self) {
        self.install_application("xf86-video-intel");
        self.install_nvidia_laptop_tools()
    }

    fn install_groovy(&self) {
        self.install_application("groovy");
    }

    fn install_handbrake(&self) {
        self.install_application("handbrake");
    }

    fn install_inkscape(&self) {
        self.install_application("inkscape");
    }

    fn install_insync(&self) {
        self.aur_install_application("insync");
    }

    fn install_intellij(&self) {
        self.aur_install_application("intellij-idea-ultimate-edition");
    }

    fn install_jdk(&self) -> Result<(), std::io::Error> {
        self.install_application("jdk-openjdk");
        unix::set_java_home(".zshrc.custom", "/usr/lib/jvm/default")?;
        unix::set_java_home(".bashrc.custom", "/usr/lib/jvm/default")?;
        Ok(())
    }

    fn install_keepassxc(&self) {
        self.install_application("keepassxc");
    }

    async fn install_kubectl(&self) -> Result<(), Box<dyn std::error::Error>>{
        self.install_application("kubectl");
        Ok(())
    }

    fn install_helm(&self) {
        self.install_application("helm");
    }

    fn install_latex(&self) {
        self.install_application("texlive-most");
    }

    fn install_lutris(&self) {
        self.install_application("lutris");
    }

    fn install_maven(&self) {
        self.install_application("maven");
    }

    fn install_makemkv(&self) {
        self.aur_install_applications(vec!["makemkv", "ccextractor"]);
    }

    // TODO: Duplicated in Ubuntu - move to Linux
    fn install_microcode(&self) -> Result<(), std::io::Error> {
        let file = File::open("/proc/cpuinfo")?;
        let buffer = BufReader::new(file);
        let cpu_name = buffer.lines().find_map(|line| {
            if line.is_ok() && line.as_ref().unwrap().starts_with("vendor_id") {
                let unwrapped_line = line.unwrap();
                return Some(unwrapped_line.split(":").next()?.to_string());
            }
            None
        });
        if cpu_name.is_none() {
            return Ok(());
        }
        if cpu_name.unwrap() == "GenuineIntel" {
            self.install_application("intel-ucode");
        } else {
            self.install_application("amd-ucode");
        }
        Ok(())
    }

    async fn install_minikube(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("minikube");
        Ok(())
    }

    fn install_mkvtoolnix(&self) {
        self.install_application("mkvtoolnix-gui");
    }

    fn install_nextcloud_client(&self) {
        self.install_application("nextcloud-client");
    }

    async fn install_nodejs(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("nvm");
        linux::setup_nodejs(self)?;
        Ok(())
    }

    async fn install_nordvpn(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("nordvpn-bin");
        self.enable_service("nordvpnd");
        Ok(())
    }

    fn install_nvidia_tools(&self) {
        self.install_applications(vec!["nvidia", "nvidia-utils", "lib32-nvidia-utils", "nvidia-settings", "vulkan-icd-loader", "lib32-vulkan-icd-loader"]);
    }

    fn install_nvidia_laptop_tools(&self) {
        self.install_application("nvidia-prime");
    }

    fn install_obs_studio(&self) {
        self.install_application("obs-studio");
    }

    fn install_onedrive(&self) {
        // no-op
    }

    fn install_origin(&self) {
        // no-op
    }

    fn install_powertop(&self) {
        self.install_application("powertop");
    }

    fn install_python(&self) {
        self.install_application("python");
    }

    async fn install_rust(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("rustup");
        self.execute("rustup default stable", true);
        Ok(())
    }

    fn install_slack(&self) {
        self.aur_install_application("slack-desktop");
    }

    fn install_spotify(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("spotify");
        Ok(())
    }

    fn install_steam(&self) {
        self.install_application("steam");
    }

    fn install_sweet_home_3d(&self) {
        self.install_application("sweethome3d");
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["base-devel", "ttf-dejavu"]);

        let original_pacman_file = File::open("/etc/pacman.conf")?;
        let original_lines = BufReader::new(original_pacman_file).lines();
        let mut enable_multilib = false;
        let new_lines = original_lines.map(|line| {
            let unwrapped_line = line.unwrap();
            return if unwrapped_line.starts_with("#[multilib]") {
                // Crude way to signify that we are under the multilib section
                enable_multilib = true;
                unwrapped_line.replacen("#", "", 1)
            } else if enable_multilib && unwrapped_line.starts_with("#Include = /etc/pacman.d/mirrorlist") {
                enable_multilib = false;
                unwrapped_line.replacen("#", "", 1)
            } else {
                unwrapped_line
            };
        }).collect::<Vec<String>>();

        let mut new_pacman_file = OpenOptions::new()
            .write(true)
            .open("/etc/pacman.conf")?;
        new_pacman_file.write_all(new_lines.join("\n").as_bytes())?;

        self.install_applications(vec!["wget"]);

        system::download_file("https://aur.archlinux.org/cgit/aur.git/snapshot/yay.tar.gz", "yay.tar.gz").await?;
        linux::untar_rename_root("yay.tar.gz", "yay")?;
        let user_id = unix::get_user_id();
        let group_id = unix::get_group_id();
        unix::recursively_chown("yay", &user_id, &group_id)?;
        self.execute_path("makepkg -si --noconfirm", false, &format!("{}/yay", std::env::current_dir().unwrap().into_os_string().into_string().unwrap()));
        Ok(())
    }

    fn install_telnet(&self) {
        self.install_application("inetutils");
    }

    async fn install_themes(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&format!("{}/.themes", system::get_home_dir()))?;
        let user_id = unix::get_user_id();
        let group_id = unix::get_group_id();
        unix::recursively_chown(&format!("{}/.themes", system::get_home_dir()), &user_id, &group_id)?;
        Ok(())
    }

    fn install_tlp(&self) {
        self.install_application("tlp");
        self.enable_service("tlp")
    }

    fn install_tmux(&self) -> Result<(), std::io::Error> {
        self.install_applications(vec!["tmux", "xclip"]);
        self.aur_install_application("tmux-bash-completion");
        linux::setup_tmux()
    }

    fn install_vim(&self) {
        self.install_application("vim");
    }

    fn install_vlc(&self) {
        self.install_application("vlc");
    }

    fn install_vm_tools(&self) {
        self.install_application("open-vm-tools");
    }

    fn install_vscode(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("code");
        Ok(())
    }

    async fn install_wifi(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::copy("/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin",
                 "/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin.bak")?;
        system::download_file(
            "https://github.com/kvalo/ath10k-firmware/raw/master/QCA6174/hw3.0/4.4.1.c3/firmware-6.bin_WLAN.RM.4.4.1.c3-00035",
            "/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin").await?;
        Ok(())
    }

    fn install_window_manager(&self) {
        self.install_applications(vec!["gnome", "libcanberra", "libappindicator-gtk3"]);
        self.aur_install_application("gnome-shell-extension-appindicator");
        self.enable_service("gdm");
        self.enable_service("NetworkManager");
    }

    fn install_wget(&self) {
        self.install_application("wget");
    }

    fn install_wine(&self) {
        self.install_application("wine");
    }

    fn install_xcode(&self) {
        // no-op
    }

    async fn install_zsh(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["zsh", "zsh-completions"]);
        unix::setup_zsh(self, None).await?;
        Ok(())
    }

    fn set_development_shortcuts(&self) {
        linux::gnome_development_shortcuts(self);
    }

    fn set_development_environment_settings(&self) -> Result<(), std::io::Error> {
        linux::set_development_environment_settings()
    }

    fn setup_power_saving_tweaks(&self) -> Result<(), std::io::Error> {
        linux::setup_power_saving_tweaks()?;
        Ok(())
    }

    fn update_os(&self) {
        self.update_os_repo();
        self.execute("pacman -Syu --noconfirm", true);
    }

    fn update_os_repo(&self) {
        self.execute("pacman -Sy", true);
    }
}
