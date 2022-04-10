use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use async_trait::async_trait;
use uuid::Uuid;

use crate::system::System;
use crate::{linux, system, unix};

pub(crate) struct Ubuntu {}

impl Ubuntu {
    fn add_apt_key(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.execute(&format!("apt-key adv --fetch-keys {}", url), true)?;
        Ok(())
    }

    fn add_apt_repo(&self, file_name: &str, urls: Vec<&str>) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(format!("/etc/apt/sources.list.d/{}.list", file_name))?;
        for url in urls {
            writeln!(file, "{}", url)?;
        }
        Ok(())
    }

    fn add_ppa(&self, ppa: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.execute(&format!("add-apt-repository -y ppa:{}", ppa), true)?;
        Ok(())
    }

    fn snap_install_application(
        &self,
        application: &str,
        classic: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if classic {
            self.execute(&format!("snap install --classic {}", application), true)?;
        } else {
            self.execute(&format!("snap install {}", application), true)?;
        }
        Ok(())
    }

    fn set_debconf(
        &self,
        installer: &str,
        conf: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let debconf_file = format!("{}.debconf", Uuid::new_v4());
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&debconf_file)?;
        writeln!(file, "{} {} select {}", installer, conf, value)?;
        writeln!(file, "{} {} seen {}", installer, conf, value)?;
        self.execute(&format!("debconf-set-selections {}", &debconf_file), true)?;
        fs::remove_file(debconf_file)?;
        Ok(())
    }
}

#[async_trait]
impl System for Ubuntu {
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
            &format!("apt-get install -y {}", application.join(" ")),
            true,
        )
    }

    fn install_android_studio(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_ppa("maarten-fonville/android-studio")?;
        self.update_os_repo()?;
        self.install_application("android-studio")?;
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
        Ok(())
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec![
            "libdvd-pkg",
            "libaacs0",
            "libbluray-bdj",
            "libbluray1",
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
        self.add_ppa("sebastian-stenzel/cryptomator")?;
        self.update_os_repo()?;
        self.install_application("cryptomator")?;
        Ok(())
    }

    fn install_curl(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("curl")?;
        Ok(())
    }

    fn install_davinci_resolve(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("davinci-resolve-studio")?;
        linux::setup_davinci_resolve(self)?;
        Ok(())
    }

    fn install_discord(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.snap_install_application("discord", false)?;
        Ok(())
    }

    fn install_docker(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("docker")?;
        linux::setup_docker(self)?;
        Ok(())
    }

    fn install_dropbox(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("nautilus-dropbox")?;
        Ok(())
    }

    async fn install_eclipse(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.snap_install_application("eclipse", true)?;
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
        Ok(())
    }

    fn install_gog_galaxy(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn install_google_chrome(&self) -> Result<(), Box<dyn std::error::Error>> {
        system::download_file(
            "https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb",
            "google-chrome.deb",
        )
        .await?;
        self.execute("dpkg -i google-chrome-stable_current_amd64.deb", true)?;
        self.install_application("chrome-gnome-shell")?;
        fs::remove_file("google-chrome.deb")?;
        Ok(())
    }

    fn install_google_cloud_sdk(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_apt_key("https://packages.cloud.google.com/apt/doc/apt-key.gpg")?;
        self.add_apt_repo(
            "google-cloud-sdk",
            vec!["deb https://packages.cloud.google.com/apt cloud-sdk main"],
        )?;
        self.install_application("google-cloud-sdk")?;
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
        self.install_application("seahorse-nautilus")?;
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
        self.install_application("insync-nautilus")?;
        Ok(())
    }

    fn install_intellij(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.snap_install_application("intellij-idea-ultimate", true)?;
        Ok(())
    }

    fn install_jdk(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["openjdk-16-jdk"])?;
        unix::set_java_home(
            self,
            ".zshrc",
            &format!("/usr/lib/jvm/java-16-openjdk-{}", std::env::consts::ARCH),
        )?;
        unix::set_java_home(
            self,
            ".bashrc",
            &format!("/usr/lib/jvm/java-16-openjdk-{}", std::env::consts::ARCH),
        )?;
        Ok(())
    }

    fn install_keepassxc(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_ppa("phoerious/keepassxc")?;
        self.update_os_repo()?;
        self.install_application("keepassxc")?;
        Ok(())
    }

    async fn install_kubectl(&self) -> Result<(), Box<dyn std::error::Error>> {
        let kubectl_version =
            reqwest::get("https://storage.googleapis.com/kubernetes-release/release/stable.txt")
                .await?
                .text()
                .await?
                .replace("\n", "");
        system::download_file(
            &format!("https://storage.googleapis.com/kubernetes-release/release/{}/bin/linux/amd64/kubectl", kubectl_version), "/usr/local/bin/kubectl").await?;
        unix::recursively_chmod("/usr/local/bin/kubectl", &0o755, &0o755)?;
        Ok(())
    }

    fn install_helm(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.execute("curl -L https://git.io/get_helm.sh | bash", true)?;
        Ok(())
    }

    fn install_latex(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("texlive-extra-utils")?;
        Ok(())
    }

    fn install_lutris(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_ppa("lutris-team/lutris")?;
        self.update_os_repo()?;
        self.install_application("lutris")?;
        Ok(())
    }

    fn install_maven(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("maven")?;
        Ok(())
    }

    fn install_makemkv(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_ppa("heyarje/makemkv-beta")?;
        self.update_os_repo()?;
        self.install_applications(vec!["makemkv-bin", "makemkv-oss"])?;
        self.install_application("ccextractor")?;
        Ok(())
    }

    // TODO: Duplicated in Arch - move to Linux
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
            self.install_application("intel-microcode")?;
        } else {
            self.install_application("amd-microcode")?;
        }
        Ok(())
    }

    async fn install_minikube(&self) -> Result<(), Box<dyn std::error::Error>> {
        system::download_file(
            "https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64",
            "/usr/local/bin/minikube",
        )
        .await?;
        unix::recursively_chmod("/usr/local/bin/minikube", &0o755, &0o755)?;
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
        self.install_application("nextcloud-desktop")?;
        Ok(())
    }

    async fn install_nodejs(&self) -> Result<(), Box<dyn std::error::Error>> {
        system::download_file(
            "https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh",
            "nvm-install.sh",
        )
        .await?;
        unix::recursively_chmod("nvm-install.sh", &0o755, &0o755)?;
        self.execute("./nvm-install.sh", false)?;
        fs::remove_file("nvm-install.sh")?;
        linux::setup_nodejs(self)?;
        Ok(())
    }

    async fn install_nordvpn(&self) -> Result<(), Box<dyn std::error::Error>> {
        system::download_file(
            "https://repo.nordvpn.com/deb/nordvpn/debian/pool/main/nordvpn-release_1.0.0_all.deb",
            "nordvpn.deb",
        )
        .await?;
        self.install_application("./nordvpn.deb")?;
        self.update_os_repo()?;
        self.install_application("nordvpn")?;
        Ok(())
    }

    fn install_nvidia_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_ppa("graphics-drivers/ppa")?;
        self.update_os_repo()?;
        self.install_application("ubuntu-drivers-common")?;
        self.execute("ubuntu-drivers autoinstall", true)?;
        Ok(())
    }

    fn install_nvidia_laptop_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("nvidia-prime")?;
        Ok(())
    }

    fn install_obs_studio(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_ppa("obsproject/obs-studio")?;
        self.update_os_repo()?;
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
        self.install_application("python3")?;
        Ok(())
    }

    async fn install_rust(&self) -> Result<(), Box<dyn std::error::Error>> {
        system::download_file("https://sh.rustup.rs", "rustup-install").await?;
        unix::recursively_chmod("rustup-install", &0o755, &0o755)?;
        self.execute("./rustup-install -y", false)?;
        fs::remove_file("rustup-install")?;
        unix::add_to_path(
            self,
            ".zshrc",
            &format!("{}/.cargo/bin", self.get_home_dir()),
        )?;
        unix::add_to_path(
            self,
            ".bashrc",
            &format!("{}/.cargo/bin", self.get_home_dir()),
        )?;
        self.execute("rustup default stable", true)?;
        Ok(())
    }

    fn install_slack(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.snap_install_application("slack", true)?;
        Ok(())
    }

    fn install_spotify(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_apt_key("https://download.spotify.com/debian/pubkey.gpg")?;
        self.add_apt_repo(
            "spotify",
            vec!["deb http://repository.spotify.com stable non-free"],
        )?;
        self.update_os_repo()?;
        self.install_application("spotify_client")?;
        Ok(())
    }

    fn install_steam(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("steam-installer")?;
        Ok(())
    }

    fn install_sweet_home_3d(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("sweethome3d")?;
        Ok(())
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.set_debconf(
            "ttf-mscorefonts-installer",
            "msttcorefonts/accepted-mscorefonts-eula",
            "true",
        )?;
        self.install_applications(vec![
            "ubuntu-restricted-extras",
            "gnome-tweaks",
            "snapd",
            "software-properties-common",
        ])?;
        linux::setup_nas(self)?;
        Ok(())
    }

    async fn install_themes(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&format!("{}/.themes", self.get_home_dir()))?;
        self.execute(
            "git clone https://github.com/Roboron3042/Cyberpunk-Neon.git",
            false,
        )?;
        linux::untar_rename_root(
            "Cyberpunk-Neon/gtk/Materia-Cyberpunk-Neon.tar.gz",
            "Materia-Cyberpunk-Neon",
        )?;
        fs::copy(
            "Materia-Cyberpunk-Neon",
            format!("{}/.themes", self.get_home_dir()),
        )?;
        fs::remove_file("Cyberpunk-Neon")?;

        self.add_ppa("snwh/ppa")?;
        self.update_os_repo()?;
        self.install_application("paper-icon-theme")?;

        system::download_file(
            "https://raw.githubusercontent.com/gusbemacbe/suru-plus/master/install.sh",
            "suru-plus-install.sh",
        )
        .await?;
        unix::recursively_chmod("suru-plus-install.sh", &0o755, &0o755)?;
        self.execute("./suru-plus-install.sh", true)?;

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
        Ok(())
    }

    fn install_tmux(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["tmux", "xclip"])?;
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
        self.install_applications(vec!["open-vm-tools", "open-vm-tools-desktop"])?;
        Ok(())
    }

    fn install_vscode(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_apt_key("https://packages.microsoft.com/keys/microsoft.asc")?;
        self.add_apt_repo(
            "vscode",
            vec!["deb [arch=amd64] https://packages.microsoft.com/repos/code stable main"],
        )?;
        self.update_os_repo()?;
        self.install_application("code")?;
        Ok(())
    }

    async fn install_wifi(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_window_manager(&self) -> Result<(), Box<dyn std::error::Error>> {
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
        self.install_application("zsh")?;
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
        self.execute("apt-get dist-upgrade -y", true)?;
        Ok(())
    }

    fn update_os_repo(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.execute("apt-get update", true)?;
        Ok(())
    }
}
