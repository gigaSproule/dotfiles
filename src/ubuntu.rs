use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Output;
use uuid::Uuid;

use async_trait::async_trait;

use crate::{linux, system, unix};
use crate::system::System;

pub(crate) struct Ubuntu {}

impl Ubuntu {
    fn add_apt_key(self, url: &str) {
        self.execute(&format!("apt-key adv --fetch-keys {}", url), true);
    }

    fn add_apt_repo(self, file_name: &str, urls: Vec<&str>) {
        let file = OpenOptions::new()
            .append(true)
            .open(format!("/etc/apt/sources.list.d/{}", file_name));
        for url in urls {
            writeln!(file, "{}", url)
        }
    }

    fn add_ppa(&self, ppa: &str) {
        self.execute(&format!("add-apt-repository -y ppa:{}", ppa), true);
    }

    fn snap_install_application(&self, application: &str, classic: bool) -> Output {
        if classic {
            self.execute(&format!("snap install --classic {}", application), true)
        } else {
            self.execute(&format!("snap install {}", application), true)
        }
    }

    fn set_debconf(&self, installer: &str, conf: &str, value: &str) {
        let debconf_file = format!("{}.debconf", Uuid::new_v4());
        let file = OpenOptions::new().append(true).open(&debconf_file);
        writeln!(file, "{} {} select {}", installer, conf, value);
        writeln!(file, "{} {} seen {}", installer, conf, value);
        self.execute(&format!("debconf-set-selections {}", &debconf_file));
        fs::remove_file(debconf_file);
    }
}

#[async_trait]
impl System for Ubuntu {
    fn execute(&self, command: &str, super_user: bool) -> Output {
        unix::execute(command, super_user)
    }

    fn install_applications(&self, application: Vec<&str>) -> Output {
        self.execute(
            format!("apt-get install -y {}", application.join(" ")),
            true,
        )
    }

    fn install_android_studio(&self) {
        self.add_ppa("maarten-fonville/android-studio");
        self.update_os_repo();
        self.install_application("android-studio");
    }

    fn install_blender(&self) {
        self.install_application("blender")
    }

    fn install_bluetooth(&self) {
        self.install_applications(vec!["bluez", "bluez-utils"]);
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec![
            "libdvd-pkg",
            "libaacs0",
            "libbluray-bdj",
            "libbluray1",
        ])?;
        system::setup_codecs()?;
        let user_id = unix::get_user_id();
        let group_id = unix::get_group_id();
        unix::recursively_chown(
            format!("{}/.config", system::get_home_dir()).as_str(),
            user_id,
            group_id,
        )?;
        Ok(())
    }

    fn install_conemu(&self) {
        // no-op
    }

    fn install_cryptomator(&self) {
        self.install_application("cryptomator");
    }

    fn install_curl(&self) {
        self.install_application("curl");
    }

    fn install_davinci_resolve(&self) -> Result<(), std::io::Error> {
        self.install_application("davinci-resolve-studio")
    }

    fn install_discord(&self) {
        self.snap_install_application("discord", false);
    }

    fn install_docker(&self) -> Result<(), std::io::Error> {
        self.install_application("docker")?;
        self.setup_docker()?;
        Ok(())
    }

    fn install_dropbox(&self) {
        self.install_application("nautilus-dropbox")
    }

    fn install_eclipse(&self) -> Result<(), std::io::Error> {
        self.snap_install_application("eclipse", true);
        if Path::new("/opt/eclipse").exists() {
            fs::create_dir_all("/opt/eclipse");
        }

        self.download_file(
            "https://projectlombok.org/downloads/lombok.jar",
            "/opt/eclipse/lombok.jar",
        );

        let mut file = OpenOptions::new()
            .append(true)
            .open("/opt/eclipse/eclipse.ini")?;

        writeln!(file, "-javaagent:/opt/eclipse/lombok.jar")?
    }

    fn install_epic_games(&self) {
        // no-op
    }

    fn install_firefox(&self) {
        self.install_application("firefox");
    }

    fn install_firmware_updater(&self) {
        self.install_application("fwupd");
    }

    fn install_gog_galaxy(&self) {
        // no-op
    }

    fn install_google_chrome(&self) {
        system::download_file(
            "https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb",
            "google-chrome.deb",
        );
        self.execute("dpkg -i google-chrome-stable_current_amd64.deb");
        fs::remove_file("google-chrome.deb");
    }

    fn install_google_cloud_sdk(&self) {
        self.add_apt_key("https://packages.cloud.google.com/apt/doc/apt-key.gpg");
        self.add_apt_repo(
            "google-cloud-sdk",
            vec!["deb https://packages.cloud.google.com/apt cloud-sdk main"],
        );
        self.install_application("google-cloud-sdk");
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
        self.install_application("seahorse-nautilus");
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
        self.install_application("insync-nautilus");
    }

    fn install_intellij(&self) {
        self.snap_install_application("intellij-idea-ultimate", true);
    }

    fn install_jdk(&self) {
        self.install_applications(vec!["openjdk-16-jdk"]);
        unix::set_java_home(
            ".zshrc.custom",
            &format!("/usr/lib/jvm/java-16-openjdk-{}", std::env::consts::ARCH),
        );
        unix::set_java_home(
            ".bashrc.custom",
            &format!("/usr/lib/jvm/java-16-openjdk-{}", std::env::consts::ARCH),
        );
    }

    fn install_keepassxc(&self) {
        self.add_ppa("phoerious/keepassxc");
        self.install_application("keepassxc");
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
        unix::recursively_chmod("/usr/local/bin/kubectl", &0o644, &0o644);
        Ok(())
    }

    fn install_helm(&self) {
        self.execute("curl -L https://git.io/get_helm.sh | bash");
    }

    fn install_latex(&self) {
        self.install_application("texlive-extra-utils");
    }

    fn install_lutris(&self) {
        self.add_ppa("lutris-team/lutris");
        self.update_os_repo();
        self.install_application("lutris");
    }

    fn install_maven(&self) {
        self.install_application("maven");
    }

    fn install_makemkv(&self) {
        self.add_ppa("heyarje/makemkv-beta");
        self.update_os_repo();
        self.install_applications(vec!["makemkv-bin", "makemkv-oss"]);
        self.install_application("ccextractor");
    }

    fn install_microcode(&self) -> Result<(), std::io::Error>{
        let file = File::open("/proc/cpuinfo")?;
        let buffer = BufReader::new(file);
        let cpu_name = buffer.lines().find_map(|line| {
            if line.is_ok() && line.unwrap().starts_with("vendor_id") {
                line.unwrap().split(":").next()?
            }
            None
        });
        if cpu_name.is_none() {
            return Ok(());
        }
        if cpu_name.unwrap() == "GenuineIntel" {
            self.install_application("intel-microcode");
        } else {
            self.install_application("amd-microcode");
        }
        Ok(())
    }

    fn install_minikube(&self) {
        system::download_file("https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64", "/usr/local/bin/minikube");
        self.execute("chmod +x /usr/local/bin/minikube");
    }

    fn install_mkvtoolnix(&self) {
        self.install_application("mkvtoolnix-gui");
    }

    fn install_nextcloud_client(&self) {
        self.install_application("nextcloud-desktop");
    }

    async fn install_nodejs(&self) -> Result<(), std::io::Error> {
        system::download_file("https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh", "nvm-install.sh").await?;
        unix::recursively_chmod("nvm-install.sh", &0o644, &0o644);
        self.execute("./nvm-install.sh", false);
        fs::remove_file("nvm-install.sh");
        unix::setup_nodejs(&self)
    }

    async fn install_nordvpn(&self) -> Result<(), std::io::Error>{
        system::download_file("https://repo.nordvpn.com/deb/nordvpn/debian/pool/main/nordvpn-release_1.0.0_all.deb", "nordvpn.deb").await?;
        self.install_application("./nordvpn.deb");
        self.update_os_repo();
        self.install_application("nordvpn");
        Ok(())
    }

    fn install_nvidia_tools(&self) {
        self.add_ppa("graphics-drivers/ppa");
        self.update_os_repo();
        self.execute("ubuntu-drivers autoinstall", true);
    }

    fn install_nvidia_laptop_tools(&self) {
        self.install_application("nvidia-prime");
    }

    fn install_obs_studio(&self) {
        self.add_ppa("obsproject/obs-studio");
        self.update_os_repo();
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
        self.install_application("python3");
    }

    fn install_rust(&self) {
        system::download_file("https://sh.rustup.rs", "rustup-install");
        unix::recursively_chmod("rustup-install", &0o644, &0o644);
        self.execute("./rustup-install -y", false);
        fs::remove_file("rustup-install");
        unix::add_to_path(".zshrc.custom", &format!("{}/.cargo/bin", system::get_home_dir()));
        unix::add_to_path(".bashrc.custom", &format!("{}/.cargo/bin", system::get_home_dir()));
        self.execute("rustup default stable");
    }

    fn install_slack(&self) {
        self.snap_install_application("slack", true);
    }

    fn install_spotify(&self) {
        self.add_apt_key("https://download.spotify.com/debian/pubkey.gpg");
        self.add_apt_repo("spotify", ["deb http://repository.spotify.com stable non-free"]);
        self.update_os_repo();
        self.install_application("spotify_client");
    }

    fn install_steam(&self) {
        self.install_application("steam-installer");
    }

    fn install_sweet_home_3d(&self) {
        self.install_application("sweethome3d");
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.set_debconf("ttf-mscorefonts-installer", "msttcorefonts/accepted-mscorefonts-eula", "true");
        self.install_applications(vec!["ubuntu-restricted-extras", "chrome-gnome-shell", "gnome-tweaks"]);
        Ok(())
    }

    fn install_telnet(&self) {
        self.install_application("telnet");
    }

    fn install_themes(&self) {
        fs::create_dir_all(&format!("{}/.themes", self.get_home_dir()));
        self.execute("git clone https://github.com/Roboron3042/Cyberpunk-Neon.git");
        linux::untar_rename_root("Cyberpunk-Neon/gtk/Materia-Cyberpunk-Neon.tar.gz", "Materia-Cyberpunk-Neon");
        fs::copy("Materia-Cyberpunk-Neon", format!("{}/.themes", system::get_home_dir()));
        fs::remove_file("Cyberpunk-Neon");

        self.add_ppa("snwh/ppa");
        self.update_os_repo();
        self.install_application("paper-icon-theme");

        system::download_file("https://raw.githubusercontent.com/gusbemacbe/suru-plus/master/install.sh", "suru-plus-install.sh");
        unix::recursively_chmod("suru-plus-install.sh", &0o644, &0o644);
        self.execute("./suru-plus-install.sh");

        let user_id = unix::get_user_id();
        let group_id = unix::get_group_id();
        unix::recursively_chown(
            &format!("{}/.themes", self.get_home_dir()),
            &user_id,
            &group_id,
        );
    }

    fn install_tlp(&self) {
        self.install_application("tlp");
    }

    fn install_tmux(&self) {
        self.install_applications(vec!["tmux", "xclip"]);
        linux::setup_tmux(&self);
    }

    fn install_vim(&self) {
        self.install_application("vim");
    }

    fn install_vlc(&self) {
        self.install_application("vlc");
    }

    fn install_vm_tools(&self) {
        self.install_applications(vec!["open-vm-tools", "open-vm-tools-desktop"]);
    }

    fn install_vscode(&self) {
        self.add_apt_key("https://packages.microsoft.com/keys/microsoft.asc");
        self.add_apt_repo("vscode", ["deb [arch=amd64] https://packages.microsoft.com/repos/vscode stable main"]);
        self.update_os_repo();
        self.install_application("code");
    }

    fn install_wifi(&self) {
        // no-op
    }

    fn install_window_manager(&self) {
        // no-op
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

    fn install_zsh(&self) {
        self.install_application("zsh");
        unix::setup_zsh(&self, None);
    }

    fn set_development_shortcuts(&self) {
        linux::gnome_development_shortcuts(&self);
    }

    fn set_development_environment_settings(&self) {
        linux::set_development_environment_settings();
    }

    fn setup_power_saving_tweaks(&self) {
        linux::setup_power_saving_tweaks();
    }

    fn update_os(&self) {
        self.update_os_repo();
        self.execute("apt-get dist-upgrade", true);
    }

    fn update_os_repo(&self) {
        self.execute("apt-get update", true);
    }
}
