use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use async_trait::async_trait;

use uuid::Uuid;

use crate::config::Config;
use crate::system::System;
use crate::{linux, system, unix};

pub(crate) struct Ubuntu<'s> {
    config: &'s Config,
}

impl<'s> Ubuntu<'s> {
    pub(crate) fn new(config: &'s Config) -> Self {
        Ubuntu { config }
    }

    fn add_apt_key(&self, url: &str) -> Result<(), Box<dyn Error>> {
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

    fn add_ppa(&self, ppa: &str) -> Result<(), Box<dyn Error>> {
        self.execute(&format!("add-apt-repository -y ppa:{}", ppa), true)?;
        Ok(())
    }

    fn enable_service(&self, service: &str) -> Result<String, Box<dyn Error>> {
        self.execute(&format!("systemctl enable service {}", service), true)
    }

    fn install_hunspell(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("hunspell")? {
            self.install_application("hunspell")?;
        }
        if !self.is_installed("hunspell-en-gb")? {
            self.install_application("hunspell-en-gb")?;
        }
        Ok(())
    }

    fn is_installed(&self, app: &str) -> Result<bool, Box<dyn Error>> {
        let dpkg_output = unix::execute(&format!("dpkg -l {}", app), true, false, false)?;
        if !dpkg_output.starts_with("dpkg-query: no packages found matching") {
            return Ok(true);
        }
        let which_output = unix::execute(&format!("which {}", app), true, false, false)?;
        if !which_output.ends_with("not found") {
            return Ok(true);
        }
        let snap_output = unix::execute(&format!("snap list | grep {}", app), false, false, false)?;
        if !snap_output.is_empty() {
            return Ok(true);
        }
        Ok(false)
    }

    fn snap_install_application(
        &self,
        application: &str,
        classic: bool,
    ) -> Result<(), Box<dyn Error>> {
        if classic {
            self.execute(&format!("snap install --classic {}", application), true)?;
        } else {
            self.execute(&format!("snap install {}", application), true)?;
        }
        Ok(())
    }

    fn set_debconf(&self, installer: &str, conf: &str, value: &str) -> Result<(), Box<dyn Error>> {
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
impl<'s> System for Ubuntu<'s> {
    fn execute(&self, command: &str, super_user: bool) -> Result<String, Box<dyn Error>> {
        unix::execute(command, super_user, true, self.config.dry_run)
    }

    fn get_home_dir(&self) -> String {
        linux::get_home_dir()
    }

    fn install_applications(&self, application: Vec<&str>) -> Result<String, Box<dyn Error>> {
        self.execute(
            &format!("apt-get install -y {}", application.join(" ")),
            true,
        )
    }

    fn install_affinity_suite(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_android_studio(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("android-studio")? {
            self.add_ppa("maarten-fonville/android-studio")?;
            self.update_os_repo()?;
            self.install_application("android-studio")?;
        }
        Ok(())
    }

    fn install_archiver(&self) -> Result<(), Box<dyn Error>> {
        if self.config.gnome && !self.is_installed("file-roller")? {
            self.install_application("file-roller")?;
        }
        if self.config.kde && !self.is_installed("ark")? {
            self.install_application("ark")?;
        }
        Ok(())
    }

    fn install_audacity(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("audacity")? {
            self.install_application("audacity")?;
        }

        if !self.is_installed("ffmpeg")? {
            self.install_application("ffmpeg")?;
        }

        if !self.is_installed("lame")? {
            self.install_application("lame")?;
        }
        Ok(())
    }

    fn install_authy(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("authy")? {
            self.snap_install_application("authy", false)?;
        }
        Ok(())
    }

    fn install_bambu_studio(&self) -> Result<(), Box<dyn Error>> {
        todo!("Implement this");
    }

    fn install_bash(&self) -> Result<(), Box<dyn Error>> {
        unix::setup_bash(self)?;
        Ok(())
    }

    fn install_blender(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("blender")? {
            self.install_application("blender")?;
        }
        Ok(())
    }

    fn install_bluetooth(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("bluez")? {
            self.install_application("bluez")?;
        }
        if !self.is_installed("bluez-utils")? {
            self.install_application("bluez-utils")?;
        }
        if !self.is_installed("pulseaudio-module-bluetooth")? {
            self.install_application("pulseaudio-module-bluetooth")?;
        }
        Ok(())
    }

    fn install_calibre(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("calibre")? {
            self.install_application("calibre")?;
        }
        Ok(())
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("libdvd-pkg")? {
            self.install_application("libdvd-pkg")?;
        }
        if !self.is_installed("libaacs0")? {
            self.install_application("libaacs0")?;
        }
        if !self.is_installed("libbluray-bdj")? {
            self.install_application("libbluray-bdj")?;
        }
        if !self.is_installed("libbluray1")? {
            self.install_application("libbluray1")?;
        }
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

    fn install_cplusplus(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("gcc")? {
            self.install_application("gcc")?;
        }
        if !self.is_installed("make")? {
            self.install_application("make")?;
        }
        if !self.is_installed("cmake")? {
            self.install_application("cmake")?;
        }
        Ok(())
    }

    async fn install_cryptomator(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("cryptomator")? {
            self.add_ppa("sebastian-stenzel/cryptomator")?;
            self.update_os_repo()?;
            self.install_application("cryptomator")?;
        }
        Ok(())
    }

    fn install_curl(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("curl")? {
            self.install_application("curl")?;
        }
        Ok(())
    }

    fn install_davinci_resolve(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("davinci-resolve-studio")? {
            self.install_application("davinci-resolve-studio")?;
            linux::setup_davinci_resolve(self)?;
        }
        Ok(())
    }

    fn install_discord(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("discord")? {
            self.snap_install_application("discord", false)?;
        }
        Ok(())
    }

    fn install_disk_usage_analyser(&self) -> Result<(), Box<dyn Error>> {
        if self.config.gnome && !self.is_installed("baobab")? {
            self.install_application("baobab ")?;
        }
        if self.config.kde && !self.is_installed("filelight")? {
            self.install_application("filelight")?;
        }
        Ok(())
    }

    fn install_development_extras(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("build-essential")? {
            self.install_application("build-essential")?;
        }
        if !self.is_installed("libssl-dev")? {
            self.install_application("libssl-dev")?;
        }
        if !self.is_installed("pkg-config")? {
            self.install_application("pkg-config")?;
        }
        Ok(())
    }

    fn install_docker(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("docker")? {
            self.install_application("docker")?;
        }
        linux::setup_docker(self)?;
        Ok(())
    }

    async fn install_eclipse(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("eclipse")? {
            self.snap_install_application("eclipse", true)?;
        }
        if Path::new("/opt/eclipse").exists() {
            fs::create_dir_all("/opt/eclipse")?;
        }

        system::download_file(
            "https://projectlombok.org/downloads/lombok.jar",
            "/opt/eclipse/lombok.jar",
        )
        .await?;

        system::add_to_file(
            "/opt/eclipse/eclipse.ini",
            "-javaagent:/opt/eclipse/lombok.jar",
        )?;
        Ok(())
    }

    fn install_epic_games(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_exact_audio_copy(&self) -> Result<(), Box<dyn Error>> {
        // Ensure Wine is installed
        // Ensure dotnet20 and dotnet40 in Wine
        // Install EAC into Wine
        Ok(())
    }

    async fn install_exercism(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("exercism")? {
            system::download_file("https://github.com/exercism/cli/releases/download/v3.1.0/exercism-3.1.0-linux-x86_64.tar.gz", "exercism.tar.gz").await?;
            let exercism_path = format!("{}/bin/exercism", self.get_home_dir());
            linux::untar_rename_root("exercism.tar.gz", &exercism_path)?;
            let user_id = unix::get_user_id();
            let group_id = unix::get_group_id();
            unix::recursively_chown("exercism", &user_id, &group_id)?;
            let exercism_bin_path = format!("{}/exercism", exercism_path);
            unix::recursively_chmod(&exercism_bin_path, &0o755, &0o755)?;
            unix::add_to_path(self, ".zshrc", &exercism_bin_path)?;
            unix::add_to_path(self, ".bashrc", &exercism_bin_path)?;
            fs::remove_file("exercism.tar.gz")?;
        }
        Ok(())
    }

    fn install_firefox(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("firefox")? {
            self.install_application("firefox")?;
            system::add_to_file(
                &format!("{}/.config/environment.d/envvars.conf", self.get_home_dir()),
                "MOZ_ENABLE_WAYLAND=1",
            )?;
        }
        Ok(())
    }

    fn install_firmware_updater(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("fwupd")? {
            self.install_application("fwupd")?;
        }
        Ok(())
    }

    fn install_git(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("git")? {
            self.install_application("git")?;
        }
        system::setup_git_config(self)?;
        Ok(())
    }

    fn install_gimp(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("gimp")? {
            self.install_application("gimp")?;
        }
        Ok(())
    }

    async fn install_godot(&self) -> Result<(), Box<dyn Error>> {
        if !Path::new("/opt/godot-mono").exists() {
            system::download_file(
                "https://github.com/godotengine/godot/releases/download/4.2.2-stable/Godot_v4.2.2-stable_mono_linux_x86_64.zip",
                "godot-mono.zip",
            )
            .await?;
            system::extract_zip(
                Path::new("godot-mono.zip"),
                Path::new("/opt/godot-mono"),
                true,
            )?;
        }
        Ok(())
    }

    fn install_gog_galaxy(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn install_google_chrome(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("google-chrome-stable")? {
            system::download_file(
                "https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb",
                "google-chrome.deb",
            )
            .await?;
            self.execute("dpkg -i google-chrome.deb", true)?;
            fs::remove_file("google-chrome.deb")?;
            println!("To enable screen sharing, you will need to enable `enable-webrtc-pipewire-catpturer` chrome://flags/#enable-webrtc-pipewire-capturer")
        }
        Ok(())
    }

    fn install_google_cloud_sdk(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("google-cloud-sdk")? {
            self.add_apt_key("https://packages.cloud.google.com/apt/doc/apt-key.gpg")?;
            self.add_apt_repo(
                "google-cloud-sdk",
                vec!["deb https://packages.cloud.google.com/apt cloud-sdk main"],
            )?;
            self.install_application("google-cloud-sdk")?;
        }
        Ok(())
    }

    fn install_google_drive(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_gpg(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("seahorse-nautilus")? {
            self.install_application("seahorse-nautilus")?;
        }
        Ok(())
    }

    fn install_gradle(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("gradle")? {
            self.install_application("gradle")?;
        }
        Ok(())
    }

    fn install_gramps(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("gramps")? {
            self.install_application("gramps")?;
        }
        Ok(())
    }

    fn install_groovy(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("groovy")? {
            self.install_application("groovy")?;
        }
        Ok(())
    }

    fn install_handbrake(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("handbrake")? {
            self.install_application("handbrake")?;
        }
        Ok(())
    }

    fn install_inkscape(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("inkscape")? {
            self.install_application("inkscape")?;
        }
        Ok(())
    }

    fn install_insync(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("insync-nautilus")? {
            self.install_application("insync-nautilus")?;
        }
        Ok(())
    }

    fn install_intel_gpu_laptop_tools(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("xf86-video-intel")? {
            self.install_application("xf86-video-intel")?;
        }
        Ok(())
    }

    fn install_intellij(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("intellij-idea-ultimate")? {
            self.snap_install_application("intellij-idea-ultimate", true)?;
        }
        Ok(())
    }

    fn install_jdk(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("openjdk-16-jdk")? {
            self.install_applications(vec!["openjdk-16-jdk"])?;
        }
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

    fn install_keepassxc(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("keepassxc")? {
            self.add_ppa("phoerious/keepassxc")?;
            self.update_os_repo()?;
            self.install_application("keepassxc")?;
        }
        Ok(())
    }

    async fn install_kubectl(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("kubectl")? {
            let kubectl_version = reqwest::get(
                "https://storage.googleapis.com/kubernetes-release/release/stable.txt",
            )
            .await?
            .text()
            .await?
            .replace('\n', "");
            system::download_file(
                &format!("https://storage.googleapis.com/kubernetes-release/release/{}/bin/linux/amd64/kubectl", kubectl_version), "/usr/local/bin/kubectl").await?;
            unix::recursively_chmod("/usr/local/bin/kubectl", &0o755, &0o755)?;
        }
        Ok(())
    }

    async fn install_helm(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("helm")? {
            system::download_file("https://git.io/get_helm.sh", "get_helm.sh").await?;
            unix::recursively_chmod("get_helm.sh", &0o755, &0o755)?;
            self.execute("./get_helm.sh", true)?;
        }
        Ok(())
    }

    fn install_latex(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("texlive-extra-utils")? {
            self.install_application("texlive-extra-utils")?;
        }
        self.install_hunspell()?;
        Ok(())
    }

    fn install_office(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("libreoffice")? {
            self.install_application("libreoffice")?;
        }
        if !self.is_installed("hyphen-en-gb")? {
            self.install_application("hyphen-en-gb")?;
        }
        self.install_hunspell()?;
        Ok(())
    }

    fn install_lutris(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("lutris")? {
            self.add_ppa("lutris-team/lutris")?;
            self.update_os_repo()?;
            self.install_application("lutris")?;
        }
        Ok(())
    }

    fn install_maven(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("maven")? {
            self.install_application("maven")?;
        }
        Ok(())
    }

    fn install_makemkv(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("makemkv-bin")? {
            self.add_ppa("heyarje/makemkv-beta")?;
            self.update_os_repo()?;
            self.install_application("makemkv-bin")?;
        }
        if !self.is_installed("makemkv-oss")? {
            self.install_application("makemkv-oss")?;
        }
        if !self.is_installed("ccextractor")? {
            self.install_application("ccextractor")?;
        }
        Ok(())
    }

    fn install_microcode(&self) -> Result<(), Box<dyn Error>> {
        let cpu_name = linux::get_cpu_name();

        match cpu_name.as_deref() {
            Some("GeniuneIntel") => {
                if !self.is_installed("intel-microcode")? {
                    self.install_application("intel-microcode")?;
                }
            }
            Some("AuthenticAMD") => {
                if !self.is_installed("amd-microcode")? {
                    self.install_application("amd-microcode")?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn install_microsoft_edge(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("microsoft-edge-stable")? {
            self.add_apt_key("https://packages.microsoft.com/keys/microsoft.asc")?;
            self.add_apt_repo(
                "microsoft-edge",
                vec!["deb [arch=amd64] https://packages.microsoft.com/repos/edge stable main"],
            )?;
            self.update_os_repo()?;
            self.install_application("microsoft-edge-stable")?;
        }
        Ok(())
    }

    async fn install_minikube(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("minikube")? {
            system::download_file(
                "https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64",
                "/usr/local/bin/minikube",
            )
            .await?;
            unix::recursively_chmod("/usr/local/bin/minikube", &0o755, &0o755)?;
        }
        Ok(())
    }

    fn install_mkvtoolnix(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("mkvtoolnix-gui")? {
            self.install_application("mkvtoolnix-gui")?;
        }
        Ok(())
    }

    fn install_networking_tools(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("inetutils")? {
            self.install_application("inetutils")?;
        }
        if !self.is_installed("nmap")? {
            self.install_application("nmap")?;
        }
        Ok(())
    }

    fn install_nextcloud_client(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("nextcloud-desktop")? {
            self.install_application("nextcloud-desktop")?;
        }
        Ok(())
    }

    async fn install_nodejs(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("nvm")? {
            system::download_file(
                "https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh",
                "nvm-install.sh",
            )
            .await?;
            unix::recursively_chmod("nvm-install.sh", &0o755, &0o755)?;
            self.execute("./nvm-install.sh", false)?;
            fs::remove_file("nvm-install.sh")?;
        }
        linux::setup_nodejs(self)?;
        Ok(())
    }

    async fn install_nordvpn(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("nordvpn")? {
            system::download_file(
                "https://repo.nordvpn.com/deb/nordvpn/debian/pool/main/nordvpn-release_1.0.0_all.deb",
                "nordvpn.deb",
            ).await?;
            self.install_application("./nordvpn.deb")?;
            self.update_os_repo()?;
            self.install_application("nordvpn")?;
        }
        if self.config.kde {
            open::that("https://store.kde.org/p/1689651")?;
        }
        Ok(())
    }

    fn install_nvidia_tools(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("ubuntu-drivers-common")? {
            self.add_ppa("graphics-drivers/ppa")?;
            self.update_os_repo()?;
            self.install_application("ubuntu-drivers-common")?;
            self.execute("ubuntu-drivers autoinstall", true)?;
        }
        Ok(())
    }

    fn install_nvidia_laptop_tools(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("nvidia-prime")? {
            self.install_application("nvidia-prime")?;
        }
        Ok(())
    }

    fn install_obs_studio(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("obs-studio")? {
            self.add_ppa("obsproject/obs-studio")?;
            self.update_os_repo()?;
            self.install_application("obs-studio")?;
        }
        Ok(())
    }

    fn install_onedrive(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_origin(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_powertop(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("powertop")? {
            self.install_application("powertop")?;
        }
        Ok(())
    }

    fn install_python(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("python3")? {
            self.install_application("python3")?;
        }
        Ok(())
    }

    fn install_retroarch(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn install_rust(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("rustup")? {
            system::download_file("https://sh.rustup.rs", "rustup-install").await?;
            unix::recursively_chmod("rustup-install", &0o755, &0o755)?;
            self.execute("./rustup-install -y", false)?;
            fs::remove_file("rustup-install")?;
        }
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

    fn install_rust_rover(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("rustrover")? {
            self.snap_install_application("rustrover", true)?;
        }
        Ok(())
    }

    fn install_slack(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("slack")? {
            self.snap_install_application("slack", true)?;
        }
        Ok(())
    }

    fn install_spotify(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("spotify_client")? {
            self.add_apt_key("https://download.spotify.com/debian/pubkey.gpg")?;
            self.add_apt_repo(
                "spotify",
                vec!["deb http://repository.spotify.com stable non-free"],
            )?;
            self.update_os_repo()?;
            self.install_application("spotify_client")?;
        }
        Ok(())
    }

    fn install_steam(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("steam-installer")? {
            self.install_application("steam-installer")?;
        }
        Ok(())
    }

    fn install_strawberry_music_player(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("strawberry")? {
            self.add_ppa("jonaski/strawberry")?;
            self.update_os_repo()?;
            self.install_application("strawberry")?;
        }
        Ok(())
    }

    fn install_sweet_home_3d(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("sweethome3d")? {
            self.install_application("sweethome3d")?;
        }

        let sweet_home_3d_desktop = "/usr/share/applictaions/sweethome3d.desktop";
        let mut sweet_home_3d_desktop_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&sweet_home_3d_desktop)?;

        let content = "[Desktop Entry]\n\
            Version=1.0\n\
            Type=Application\n\
            Name=Sweet Home 3D\n\
            Comment=An interior design application\n\
            TryExec=sweethome3d\n\
            Exec=env JAVA_HOME=/usr/lib/jvm/java-11-openjdk sweethome3d\n\
            Icon=sweethome3d\n\
            Categories=Office;Java;\n\
            StartupWMClass=com-eteks-sweethome3d-SweetHome3D\n\
            MimeType=application/x-sweethome3d\n";
        write!(sweet_home_3d_desktop_file, "{}", content)?;

        Ok(())
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn Error>> {
        self.set_debconf(
            "ttf-mscorefonts-installer",
            "msttcorefonts/accepted-mscorefonts-eula",
            "true",
        )?;
        if !self.is_installed("network-manager")? {
            self.install_application("network-manager")?;
        }
        if !self.is_installed("ubuntu-restricted-extras")? {
            self.install_application("ubuntu-restricted-extras")?;
        }
        if !self.is_installed("snapd")? {
            self.install_application("snapd")?;
        }
        if !self.is_installed("software-properties-common")? {
            self.install_application("software-properties-common")?;
        }
        if !self.is_installed("alsa-base")? {
            self.install_application("alsa-base")?;
        }
        if !self.is_installed("man-db")? {
            self.install_application("man-db")?;
        }
        if !self.is_installed("pipewire")? {
            self.install_application("pipewire")?;
        }
        if !self.is_installed("pipewire-pulse")? {
            self.install_application("pipewire-pulse")?;
        }
        if !self.is_installed("wireplumber")? {
            self.install_application("wireplumber")?;
        }
        linux::setup_nas(self)?;
        Ok(())
    }

    fn install_terraform(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("terraform")? {
            self.add_apt_key("https://apt.releases.hashicorp.com/gpg")?;
            self.add_apt_repo(
                "terraform",
                vec!["https://apt.releases.hashicorp.com jammy"],
            )?;
            self.install_application("terraform")?;
        }
        Ok(())
    }

    async fn install_themes(&self) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(&format!("{}/.themes", self.get_home_dir()))?;
        let user_id = unix::get_user_id();
        let group_id = unix::get_group_id();
        unix::recursively_chown(
            &format!("{}/.themes", self.get_home_dir()),
            &user_id,
            &group_id,
        )?;
        if self.config.gnome {
            linux::gtk_theme(self)?;
        }
        Ok(())
    }

    fn install_tlp(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("tlp")? {
            self.install_application("tlp")?;
        }
        Ok(())
    }

    fn install_tmux(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("tmux")? {
            self.install_application("tmux")?;
        }
        if !self.is_installed("xclip")? {
            self.install_application("xclip")?;
        }
        linux::setup_tmux(self)?;
        Ok(())
    }

    fn install_vim(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("vim")? {
            self.install_application("vim")?;
        }
        Ok(())
    }

    async fn install_vlc(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("vlc")? {
            self.install_application("vlc")?;
        }
        Ok(())
    }

    fn install_vm_tools(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("open-vm-tools")? {
            self.install_application("open-vm-tools")?;
        }
        if !self.is_installed("open-vm-tools-desktop")? {
            self.install_application("open-vm-tools-desktop")?;
        }
        Ok(())
    }

    fn install_vscode(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("code")? {
            self.add_apt_key("https://packages.microsoft.com/keys/microsoft.asc")?;
            self.add_apt_repo(
                "vscode",
                vec!["deb [arch=amd64] https://packages.microsoft.com/repos/code stable main"],
            )?;
            self.update_os_repo()?;
            self.install_application("code")?;
        }
        self.install_hunspell()?;
        Ok(())
    }

    async fn install_wifi(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_window_manager(&self) -> Result<(), Box<dyn Error>> {
        if self.config.gnome {
            if !self.is_installed("ubuntu-desktop-minimal")? {
                self.install_application("ubuntu-desktop-minimal")?;
            }
            if !self.is_installed("network-manager-gnome")? {
                self.install_application("network-manager-gnome")?;
            }
            if !self.is_installed("gnome-tweaks")? {
                self.install_application("gnome-tweaks")?;
            }
            if !self.is_installed("xdg-desktop-portal-gnome")? {
                self.install_application("xdg-desktop-portal-gnome ")?;
            }
            if !self.is_installed("libcanberra0")? {
                self.install_application("libcanberra0")?;
            }
            if !self.is_installed("libappindicator")? {
                self.install_application("libappindicator")?;
            }
            if !self.is_installed("gnome-shell-extension-appindicator")? {
                self.install_application("gnome-shell-extension-appindicator")?;
            }
            if !self.is_installed("chrome-gnome-shell")? {
                self.install_application("chrome-gnome-shell")?;
            }
            open::that("https://extensions.gnome.org/extension/545/hide-top-bar/")?;
            open::that("https://extensions.gnome.org/extension/1595/nordvpn-connect/")?;
            open::that("https://extensions.gnome.org/extension/3960/transparent-top-bar-adjustable-transparency/")?;
            self.execute("dpkg-reconfigure gdm3", true)?;
        }
        if self.config.kde {
            if !self.is_installed("kde-plasma-desktop")? {
                self.install_application("kde-plasma-desktop")?;
            }
            if !self.is_installed("baloo")? {
                self.install_application("baloo")?;
            }
            if !self.is_installed("dolphin")? {
                self.install_application("dolphin")?;
            }
            if !self.is_installed("dolphin-plugins")? {
                self.install_application("dolphin-plugins")?;
            }
            if !self.is_installed("phonon-qt5-vlc")? {
                self.install_application("phonon-qt5-vlc")?;
            }
            if !self.is_installed("ffmpegthumbnailer")? {
                self.install_application("ffmpegthumbnailer")?;
            }
            if !self.is_installed("ffmpegthumbs")? {
                self.install_application("ffmpegthumbs")?;
            }
            if !self.is_installed("gwenview")? {
                self.install_application("gwenview")?;
            }
            if !self.is_installed("kdegraphics-thumbnailers")? {
                self.install_application("kdegraphics-thumbnailers")?;
            }
            if !self.is_installed("konsole")? {
                self.install_application("konsole")?;
            }
            if !self.is_installed("ktorrent")? {
                self.install_application("ktorrent")?;
            }
            if !self.is_installed("latte-dock")? {
                self.install_application("latte-dock")?;
            }
            if !self.is_installed("okular")? {
                self.install_application("okular")?;
            }
            if !self.is_installed("sddm")? {
                self.install_application("sddm")?;
            }
            if !self.is_installed("kde-config-sddm")? {
                self.install_application("kde-config-sddm")?;
            }
            if !self.is_installed("xdg-desktop-portal-kde")? {
                self.install_application("xdg-desktop-portal-kde")?;
            }
            // TODO: Implement install steps
            open::that("https://github.com/alex1701c/NordVPNKrunner")?;
            self.execute("dpkg-reconfigure sddm", true)?;
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(format!(
                    "{}/.config/plasma-workspace/env/gtk.sh",
                    self.get_home_dir()
                ))?;
            writeln!(file, "export GTK_USE_PORTAL=1")?;
        }
        self.enable_service("NetworkManager")?;
        Ok(())
    }

    fn install_wget(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("wget")? {
            self.install_application("wget")?;
        }
        Ok(())
    }

    fn install_whatsapp(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_wine(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("wine")? {
            self.install_application("wine")?;
        }
        Ok(())
    }

    async fn install_xbox_streaming(&self) -> Result<(), Box<dyn Error>> {
        system::download_file(
            "https://github.com/unknownskl/greenlight/releases/download/v2.3.2/greenlight_2.3.2_amd64.deb",
            "greenlight.deb",
        ).await?;
        self.execute("dpkg -i greenlight.deb", true)?;
        fs::remove_file("greenlight.deb")?;
        Ok(())
    }

    fn install_xcode(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn install_zsh(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("zsh")? {
            self.install_application("zsh")?;
        }
        unix::setup_zsh(self, None).await?;
        Ok(())
    }

    fn set_development_shortcuts(&self) -> Result<(), Box<dyn Error>> {
        if self.config.gnome {
            linux::gnome_development_shortcuts(self)?;
        }
        Ok(())
    }

    fn set_development_environment_settings(&self) -> Result<(), Box<dyn Error>> {
        linux::set_development_environment_settings()?;
        Ok(())
    }

    fn setup_power_saving_tweaks(&self) -> Result<(), Box<dyn Error>> {
        linux::setup_power_saving_tweaks()?;
        Ok(())
    }

    fn setup_user_bin(&self) -> Result<(), Box<dyn Error>> {
        unix::setup_user_bin(self)?;
        Ok(())
    }

    fn update_os(&self) -> Result<(), Box<dyn Error>> {
        self.update_os_repo()?;
        self.execute("apt-get dist-upgrade -y", true)?;
        Ok(())
    }

    fn update_os_repo(&self) -> Result<(), Box<dyn Error>> {
        self.execute("apt-get update", true)?;
        Ok(())
    }
}
