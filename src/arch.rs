use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use async_trait::async_trait;

use crate::system::System;
use crate::{linux, system, unix};

pub(crate) struct Arch {}

static JAVA_HOME: &str = "/usr/lib/jvm/default";

impl Arch {
    fn aur_install_application(
        &self,
        application: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.aur_install_applications(vec![application])
    }

    fn aur_install_applications(
        &self,
        applications: Vec<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.execute(
            &format!("yay -S --noconfirm --needed {}", applications.join(" ")),
            false,
        )
    }

    fn enable_service(&self, service: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.execute(&format!("systemctl enable service {}", service), true)
    }
}

#[async_trait]
impl System for Arch {
    fn execute(
        &self,
        command: &str,
        super_user: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        unix::execute(command, super_user)
    }

    fn get_home_dir(&self) -> String {
        linux::get_home_dir(self)
    }

    fn install_applications(
        &self,
        application: Vec<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.execute(
            &format!("pacman -S --noconfirm --needed {}", application.join(" ")),
            true,
        )
    }

    fn install_android_studio(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("android-studio")?;
        Ok(())
    }

    fn install_bash(&self) -> Result<(), Box<dyn std::error::Error>> {
        unix::setup_bash(self)?;
        Ok(())
    }

    fn install_blender(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("blender")?;
        Ok(())
    }

    fn install_bluetooth(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["bluez", "bluez-utils"])?;
        self.enable_service("bluetooth")?;
        Ok(())
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec![
            "libdvdread",
            "libdvdcss",
            "libdvdnav",
            "libbluray",
            "libaacs",
            "x264",
            "x265",
            "xvidcore",
            "libmpeg2",
            "svt-av1",
            "libvpx",
            "libtheora",
            "gst-plugins-ugly",
            "gst-libav",
        ])?;
        system::setup_codecs(self).await?;
        let user_id = unix::get_user_id();
        let group_id = unix::get_group_id();
        unix::recursively_chown(
            &format!("{}/.config", self.get_home_dir()),
            &user_id,
            &group_id,
        )?;
        Ok(())
    }

    fn install_conemu(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_cryptomator(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Required as a dependency for cryptomator
        self.install_jdk()?;
        self.aur_install_application("cryptomator")?;
        Ok(())
    }

    fn install_curl(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("curl")?;
        Ok(())
    }

    fn install_davinci_resolve(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("davinci-resolve-studio")?;
        linux::setup_davinci_resolve(self)?;
        Ok(())
    }

    fn install_discord(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("discord")?;
        Ok(())
    }

    fn install_docker(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("docker")?;
        linux::setup_docker(self)?;
        Ok(())
    }

    fn install_dropbox(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["dropbox", "nautilus-dropbox"])?;
        Ok(())
    }

    async fn install_eclipse(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("eclipse-jee")?;
        if Path::new("/opt/eclipse").exists() {
            fs::create_dir_all("/opt/eclipse")?;
        }

        system::download_file(
            "https://projectlombok.org/downloads/lombok.jar",
            "/opt/eclipse/lombok.jar",
        )
        .await?;

        let mut file = OpenOptions::new()
            .append(true)
            .open("/opt/eclipse/eclipse.ini")?;

        writeln!(file, "-javaagent:/opt/eclipse/lombok.jar")?;
        Ok(())
    }

    fn install_epic_games(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_firefox(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("firefox")?;
        Ok(())
    }

    fn install_firmware_updater(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("fwupd")?;
        self.enable_service("fwupd")?;
        Ok(())
    }

    fn install_gog_galaxy(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn install_google_chrome(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("google-chrome")?;
        Ok(())
    }

    fn install_google_cloud_sdk(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("google-cloud-sdk")?;
        Ok(())
    }

    fn install_google_drive(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_git(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("git")?;
        system::setup_git_config(self)?;
        Ok(())
    }

    fn install_gimp(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("gimp")?;
        Ok(())
    }

    fn install_gpg(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["seahorse", "seahorse-nautilus"])?;
        Ok(())
    }

    fn install_gradle(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("gradle")?;
        Ok(())
    }

    fn install_graphic_card_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        // if nvidia
        self.install_nvidia_tools()?;
        // else
        Ok(())
    }

    fn install_graphic_card_laptop_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("xf86-video-intel")?;
        self.install_nvidia_laptop_tools()?;
        Ok(())
    }

    fn install_groovy(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("groovy")?;
        Ok(())
    }

    fn install_handbrake(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("handbrake")?;
        Ok(())
    }

    fn install_inkscape(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("inkscape")?;
        Ok(())
    }

    fn install_insync(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("insync")?;
        Ok(())
    }

    fn install_intellij(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("intellij-idea-ultimate-edition")?;
        Ok(())
    }

    fn install_jdk(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("jdk-openjdk")?;
        unix::set_java_home(self, ".zshrc", JAVA_HOME)?;
        unix::set_java_home(self, ".bashrc", JAVA_HOME)?;
        unix::add_to_path(self, ".zshrc", "$JAVA_HOME/bin")?;
        unix::add_to_path(self, ".bashrc", "$JAVA_HOME/bin")?;
        Ok(())
    }

    fn install_keepassxc(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("keepassxc")?;
        Ok(())
    }

    async fn install_kubectl(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("kubectl")?;
        Ok(())
    }

    fn install_helm(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("helm")?;
        Ok(())
    }

    fn install_latex(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("texlive-most")?;
        Ok(())
    }

    fn install_lutris(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("lutris")?;
        Ok(())
    }

    fn install_maven(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("maven")?;
        Ok(())
    }

    fn install_makemkv(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_applications(vec!["makemkv", "ccextractor"])?;
        Ok(())
    }

    // TODO: Duplicated in Ubuntu - move to Linux
    fn install_microcode(&self) -> Result<(), Box<dyn std::error::Error>> {
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
            self.install_application("intel-ucode")?;
        } else {
            self.install_application("amd-ucode")?;
        }
        Ok(())
    }

    async fn install_minikube(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("minikube")?;
        Ok(())
    }

    fn install_mkvtoolnix(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("mkvtoolnix-gui")?;
        Ok(())
    }

    fn install_networking_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["inetutils", "nmap"])?;
        Ok(())
    }

    fn install_nextcloud_client(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("nextcloud-client")?;
        Ok(())
    }

    async fn install_nodejs(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("nvm")?;
        linux::setup_nodejs(self)?;
        Ok(())
    }

    async fn install_nordvpn(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("nordvpn-bin")?;
        self.enable_service("nordvpnd")?;
        Ok(())
    }

    fn install_nvidia_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec![
            "nvidia",
            "nvidia-utils",
            "lib32-nvidia-utils",
            "nvidia-settings",
            "vulkan-icd-loader",
            "lib32-vulkan-icd-loader",
            "opencl-nvidia",
        ])?;
        Ok(())
    }

    fn install_nvidia_laptop_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("nvidia-prime")?;
        Ok(())
    }

    fn install_obs_studio(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("obs-studio")?;
        Ok(())
    }

    fn install_onedrive(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_origin(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_powertop(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("powertop")?;
        Ok(())
    }

    fn install_python(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("python")?;
        Ok(())
    }

    async fn install_rust(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("rustup")?;
        self.execute("rustup default stable", true)?;
        Ok(())
    }

    fn install_slack(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("slack-desktop")?;
        Ok(())
    }

    fn install_spotify(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.aur_install_application("spotify")?;
        Ok(())
    }

    fn install_steam(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("steam")?;
        Ok(())
    }

    fn install_sweet_home_3d(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("sweethome3d")?;
        Ok(())
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["base-devel", "ttf-dejavu"])?;

        let original_pacman_file = File::open("/etc/pacman.conf")?;
        let original_lines = BufReader::new(original_pacman_file).lines();
        let mut enable_multilib = false;
        let mut new_lines = original_lines
            .map(|line| {
                let unwrapped_line = line.unwrap();
                return if unwrapped_line.starts_with("#[multilib]") {
                    // Crude way to signify that we are under the multilib section
                    enable_multilib = true;
                    unwrapped_line.replacen("#", "", 1)
                } else if enable_multilib
                    && unwrapped_line.starts_with("#Include = /etc/pacman.d/mirrorlist")
                {
                    enable_multilib = false;
                    unwrapped_line.replacen("#", "", 1)
                } else {
                    unwrapped_line
                };
            })
            .collect::<Vec<String>>();

        if !new_lines.contains(&"[multilib]".to_string()) {
            new_lines.push("[multilib]".to_string());
            new_lines.push("Include = /etc/pacman.d/mirrorlist".to_string());
        }

        let mut new_pacman_file = OpenOptions::new().write(true).open("/etc/pacman.conf")?;
        new_pacman_file.write_all(new_lines.join("\n").as_bytes())?;

        self.update_os_repo()?;
        self.install_applications(vec!["wget"])?;

        system::download_file(
            "https://aur.archlinux.org/cgit/aur.git/snapshot/yay.tar.gz",
            "yay.tar.gz",
        )
        .await?;
        linux::untar_rename_root("yay.tar.gz", "yay")?;
        let user_id = unix::get_user_id();
        let group_id = unix::get_group_id();
        unix::recursively_chown("yay", &user_id, &group_id)?;
        unix::execute_path(
            "makepkg -si --noconfirm",
            false,
            &format!(
                "{}/yay",
                std::env::current_dir()
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap()
            ),
        )?;
        linux::setup_nas(self)?;
        Ok(())
    }

    async fn install_themes(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&format!("{}/.themes", self.get_home_dir()))?;
        let user_id = unix::get_user_id();
        let group_id = unix::get_group_id();
        unix::recursively_chown(
            &format!("{}/.themes", self.get_home_dir()),
            &user_id,
            &group_id,
        )?;
        Ok(())
    }

    fn install_tlp(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("tlp")?;
        self.enable_service("tlp")?;
        Ok(())
    }

    fn install_tmux(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["tmux", "xclip"])?;
        self.aur_install_application("tmux-bash-completion")?;
        linux::setup_tmux(self)?;
        Ok(())
    }

    fn install_vim(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("vim")?;
        Ok(())
    }

    fn install_vlc(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("vlc")?;
        Ok(())
    }

    fn install_vm_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("open-vm-tools")?;
        Ok(())
    }

    fn install_vscode(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("code")?;
        Ok(())
    }

    async fn install_wifi(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::copy(
            "/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin",
            "/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin.bak",
        )?;
        system::download_file(
            "https://github.com/kvalo/ath10k-firmware/raw/master/QCA6174/hw3.0/4.4.1.c3/firmware-6.bin_WLAN.RM.4.4.1.c3-00035",
            "/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin").await?;
        Ok(())
    }

    fn install_window_manager(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Gnome
        self.install_applications(vec!["gnome", "libcanberra", "libappindicator-gtk3"])?;
        self.aur_install_application("gnome-shell-extension-appindicator")?;
        self.enable_service("gdm")?;
        self.enable_service("NetworkManager")?;
        // KDE/Plasma
        self.install_applications(vec![
            "ark",
            "baloo",
            "dolphin",
            "dolphin-plugins",
            "ffmpegthumbnailer",
            "ffmpegthumbs",
            "gwenview",
            "konsole",
            "ktorrent",
            "latte-dock",
            "okular",
            "plasma",
            "plasma-wayland-session",
            "sddm",
            "sddm-kcm",
        ])?;
        self.enable_service("sddm")?;
        Ok(())
    }

    fn install_wget(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("wget")?;
        Ok(())
    }

    fn install_wine(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("wine")?;
        Ok(())
    }

    fn install_xcode(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn install_zsh(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["zsh", "zsh-completions"])?;
        unix::setup_zsh(self, None).await?;
        Ok(())
    }

    fn set_development_shortcuts(&self) -> Result<(), Box<dyn std::error::Error>> {
        linux::gnome_development_shortcuts(self)?;
        Ok(())
    }

    fn set_development_environment_settings(&self) -> Result<(), Box<dyn std::error::Error>> {
        linux::set_development_environment_settings()?;
        Ok(())
    }

    fn setup_power_saving_tweaks(&self) -> Result<(), Box<dyn std::error::Error>> {
        linux::setup_power_saving_tweaks()?;
        Ok(())
    }

    fn update_os(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.update_os_repo()?;
        self.execute("pacman -Syu --noconfirm", true)?;
        Ok(())
    }

    fn update_os_repo(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.execute("pacman -Sy", true)?;
        Ok(())
    }
}
