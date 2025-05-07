use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use async_trait::async_trait;

use crate::config::Config;
use crate::system::System;
use crate::{linux, system, unix};

pub(crate) struct Arch<'s> {
    config: &'s Config,
}

static JAVA_HOME: &str = "/usr/lib/jvm/default";

impl<'s> Arch<'s> {
    pub(crate) fn new(config: &'s Config) -> Self {
        Arch { config }
    }

    fn aur_install_application(&self, application: &str) -> Result<String, Box<dyn Error>> {
        self.aur_install_applications(vec![application])
    }

    fn aur_install_applications(&self, applications: Vec<&str>) -> Result<String, Box<dyn Error>> {
        self.execute(
            &format!("yay -S --noconfirm --needed {}", applications.join(" ")),
            false,
        )
    }

    fn enable_service(&self, service: &str) -> Result<String, Box<dyn Error>> {
        self.execute(&format!("systemctl enable {}", service), true)
    }

    fn install_hunspell(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("hunspell")? {
            self.install_application("hunspell")?;
        }
        if !self.is_installed("hunspell-en_gb")? {
            self.install_application("hunspell-en_gb")?;
        }
        Ok(())
    }

    fn is_installed(&self, app: &str) -> Result<bool, Box<dyn Error>> {
        let output = unix::execute(&format!("pacman -Qi {}", app), false, false, false);
        if !output?.ends_with("was not found") {
            return Ok(true);
        }
        Ok(false)
    }
}

#[async_trait]
impl<'s> System for Arch<'s> {
    fn execute(&self, command: &str, super_user: bool) -> Result<String, Box<dyn Error>> {
        unix::execute(command, super_user, true, self.config.dry_run)
    }

    fn get_home_dir(&self) -> String {
        linux::get_home_dir()
    }

    fn install_applications(&self, application: Vec<&str>) -> Result<String, Box<dyn Error>> {
        self.execute(
            &format!("pacman -S --noconfirm --needed {}", application.join(" ")),
            true,
        )
    }

    fn install_affinity_suite(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_android_studio(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("android-studio")? {
            self.aur_install_application("android-studio")?;
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
            self.aur_install_application("authy")?;
        }
        Ok(())
    }

    fn install_bambu_studio(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("bambustudio-bin")? {
            self.aur_install_application("bambustudio-bin")?;
        }
        Ok(())
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
        if !self.is_installed("pulseaudio-bluetooth")? {
            self.install_application("pulseaudio-bluetooth")?;
        }
        self.enable_service("bluetooth")?;
        Ok(())
    }

    fn install_calibre(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("calibre")? {
            self.install_application("calibre")?;
        }
        Ok(())
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("libdvdread")? {
            self.install_application("libdvdread")?;
        }
        if !self.is_installed("libdvdcss")? {
            self.install_application("libdvdcss")?;
        }
        if !self.is_installed("libdvdnav")? {
            self.install_application("libdvdnav")?;
        }
        if !self.is_installed("libbluray")? {
            self.install_application("libbluray")?;
        }
        if !self.is_installed("libaacs")? {
            self.install_application("libaacs")?;
        }
        if !self.is_installed("x264")? {
            self.install_application("x264")?;
        }
        if !self.is_installed("x265")? {
            self.install_application("x265")?;
        }
        if !self.is_installed("xvidcore")? {
            self.install_application("xvidcore")?;
        }
        if !self.is_installed("libmpeg2")? {
            self.install_application("libmpeg2")?;
        }
        if !self.is_installed("svt-av1")? {
            self.install_application("svt-av1")?;
        }
        if !self.is_installed("libvpx")? {
            self.install_application("libvpx")?;
        }
        if !self.is_installed("libtheora")? {
            self.install_application("libtheora")?;
        }
        if !self.is_installed("gst-plugins-ugly")? {
            self.install_application("gst-plugins-ugly")?;
        }
        if !self.is_installed("gst-libav")? {
            self.install_application("gst-libav")?;
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
        // Required as a dependency for cryptomator
        self.install_jdk()?;
        if !self.is_installed("cryptomator")? {
            self.aur_install_application("cryptomator")?;
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
            self.aur_install_application("davinci-resolve-studio")?;
        }
        linux::setup_davinci_resolve(self)?;
        Ok(())
    }

    fn install_discord(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("discord")? {
            self.install_application("discord")?;
        }
        Ok(())
    }

    fn install_disk_usage_analyser(&self) -> Result<(), Box<dyn Error>> {
        if self.config.gnome && !self.is_installed("baobab")? {
            self.install_application("baobab")?;
        }
        if self.config.kde && !self.is_installed("filelight")? {
            self.install_application("filelight")?;
        }
        Ok(())
    }

    fn install_development_extras(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("pkgconf")? {
            self.install_application("pkgconf")?;
        }
        Ok(())
    }

    fn install_docker(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("docker")? {
            self.install_application("docker")?;
        }
        if !self.is_installed("docker-compose")? {
            self.install_application("docker-compose")?;
        }
        linux::setup_docker(self)?;
        Ok(())
    }

    async fn install_eclipse(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("eclipse-jee")? {
            self.aur_install_application("eclipse-jee")?;
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
        if !self.is_installed("exercism-bin")? {
            self.install_application("exercism-bin")?;
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
        self.enable_service("fwupd")?;
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
        if !self.is_installed("godot-mono")? {
            self.aur_install_application("godot-mono")?;
        }
        Ok(())
    }

    fn install_gog_galaxy(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn install_google_chrome(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("google-chrome")? {
            self.aur_install_application("google-chrome")?;
            println!("To enable screen sharing, you will need to enable `enable-webrtc-pipewire-catpturer` chrome://flags/#enable-webrtc-pipewire-capturer")
        }
        Ok(())
    }

    fn install_google_cloud_sdk(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("google-cloud-sdk")? {
            self.aur_install_application("google-cloud-sdk")?;
        }
        Ok(())
    }

    fn install_google_drive(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_gpg(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("seahorse")? {
            self.install_application("seahorse")?;
        }
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
        if !self.is_installed("insync")? {
            self.aur_install_application("insync")?;
        }
        if !self.is_installed("insync-emblem-icons")? {
            self.aur_install_application("insync-emblem-icons")?;
        }
        if self.config.gnome && !self.is_installed("insync-nautilus")? {
            self.aur_install_application("insync-nautilus")?;
        }
        if self.config.kde && !self.is_installed("insync-dolphin")? {
            self.aur_install_application("insync-dolphin")?;
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
        if !self.is_installed("intellij-idea-ultimate-edition")? {
            self.aur_install_application("intellij-idea-ultimate-edition")?;
        }
        Ok(())
    }

    fn install_jdk(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("jdk-openjdk")? {
            self.install_application("jdk-openjdk")?;
        }
        unix::set_java_home(self, ".zshrc", JAVA_HOME)?;
        unix::set_java_home(self, ".bashrc", JAVA_HOME)?;
        unix::add_to_path(self, ".zshrc", "$JAVA_HOME/bin")?;
        unix::add_to_path(self, ".bashrc", "$JAVA_HOME/bin")?;
        Ok(())
    }

    fn install_keepassxc(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("keepassxc")? {
            self.install_application("keepassxc")?;
        }
        Ok(())
    }

    async fn install_kubectl(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("kubectl")? {
            self.install_application("kubectl")?;
        }
        Ok(())
    }

    async fn install_helm(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("helm")? {
            self.install_application("helm")?;
        }
        Ok(())
    }

    fn install_latex(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("texlive-most")? {
            self.install_application("texlive-most")?;
        }
        if !self.is_installed("perl-yaml-tiny")? {
            self.install_application("perl-yaml-tiny")?;
        }
        if !self.is_installed("perl-file-homedir")? {
            self.install_application("perl-file-homedir")?;
        }
        self.install_hunspell()?;
        Ok(())
    }

    fn install_office(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("libreoffice-fresh")? {
            self.install_application("libreoffice-fresh")?;
        }
        if !self.is_installed("hyphen")? {
            self.install_application("hyphen")?;
        }
        if !self.is_installed("hyphen-en")? {
            self.install_application("hyphen-en")?;
        }
        self.install_hunspell()?;
        Ok(())
    }

    fn install_openscad(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("openscad-git")? {
            self.aur_install_application("openscad-git")?;
        }
        Ok(())
    }

    fn install_lutris(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("lutris")? {
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
        if !self.is_installed("makemkv")? {
            self.aur_install_application("makemkv")?;
        }
        if !self.is_installed("ccextractor")? {
            self.aur_install_application("ccextractor")?;
        }
        Ok(())
    }

    fn install_microcode(&self) -> Result<(), Box<dyn Error>> {
        let cpu_name = linux::get_cpu_name();

        match cpu_name.as_deref() {
            Some("GeniuneIntel") => {
                if !self.is_installed("intel-ucode")? {
                    self.install_application("intel-ucode")?;
                }
            }
            Some("AuthenticAMD") => {
                if !self.is_installed("amd-ucode")? {
                    self.install_application("amd-ucode")?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn install_microsoft_edge(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("microsoft-edge-stable-bin")? {
            self.aur_install_application("microsoft-edge-stable-bin")?;
        }
        Ok(())
    }

    async fn install_minikube(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("minikube")? {
            self.install_application("minikube")?;
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
        if !self.is_installed("nextcloud-client")? {
            self.install_application("nextcloud-client")?;
        }
        Ok(())
    }

    async fn install_nodejs(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("nvm")? {
            self.aur_install_application("nvm")?;
        }
        linux::setup_nodejs(self)?;
        Ok(())
    }

    async fn install_nordvpn(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("nordvpn-bin")? {
            self.aur_install_application("nordvpn-bin")?;
        }
        if self.config.kde {
            open::that("https://store.kde.org/p/1689651")?;
        }
        self.enable_service("nordvpnd")?;
        Ok(())
    }

    fn install_nvidia_tools(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("nvidia")? {
            self.install_application("nvidia")?;
        }
        if !self.is_installed("nvidia-utils")? {
            self.install_application("nvidia-utils")?;
        }
        if !self.is_installed("lib32-nvidia-utils")? {
            self.install_application("lib32-nvidia-utils")?;
        }
        if !self.is_installed("nvidia-settings")? {
            self.install_application("nvidia-settings")?;
        }
        if !self.is_installed("vulkan-icd-loader")? {
            self.install_application("vulkan-icd-loader")?;
        }
        if !self.is_installed("lib32-vulkan-icd-loader")? {
            self.install_application("lib32-vulkan-icd-loader")?;
        }
        if !self.is_installed("opencl-nvidia")? {
            self.install_application("opencl-nvidia")?;
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
            self.install_application("obs-studio")?;
        }
        if !self.is_installed("qt6-wayland")? {
            self.install_application("qt6-wayland")?;
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
        if !self.is_installed("python")? {
            self.install_application("python")?;
        }
        Ok(())
    }

    fn install_retroarch(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("retroarch")? {
            self.install_application("retroarch")?;
        }
        if !self.is_installed("libretro-gambatte")? {
            self.install_application("libretro-gambatte")?;
        }
        if !self.is_installed("libretro-mgba")? {
            self.install_application("libretro-mgba")?;
        }
        if !self.is_installed("libretro-beetle-psx-hw")? {
            self.install_application("libretro-beetle-psx-hw")?;
        }
        if !self.is_installed("libretro-desmume")? {
            self.install_application("libretro-desmume")?;
        }
        if !self.is_installed("libretro-yabause")? {
            self.install_application("libretro-yabause")?;
        }
        if !self.is_installed("libretro-mupen64plus-next")? {
            self.install_application("libretro-mupen64plus-next")?;
        }
        if !self.is_installed("libretro-snes9x")? {
            self.install_application("libretro-snes9x")?;
        }
        if !self.is_installed("libretro-ppsspp")? {
            self.install_application("libretro-ppsspp")?;
        }
        if !self.is_installed("libretro-genesis-plus-gx")? {
            self.install_application("libretro-genesis-plus-gx")?;
        }
        Ok(())
    }

    async fn install_rust(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("rustup")? {
            self.install_application("rustup")?;
            self.execute("rustup default stable", true)?;
        }
        Ok(())
    }

    fn install_rust_rover(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("rustrover")? {
            self.aur_install_application("rustrover")?;
        }
        Ok(())
    }

    fn install_slack(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("slack-desktop")? {
            self.aur_install_application("slack-desktop")?;
        }
        Ok(())
    }

    fn install_spotify(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("spotify")? {
            self.aur_install_application("spotify")?;
        }
        Ok(())
    }

    fn install_steam(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("steam")? {
            self.install_application("steam")?;
        }
        Ok(())
    }

    fn install_strawberry_music_player(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("strawberry")? {
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
            .open(sweet_home_3d_desktop)?;

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
        if !self.is_installed("networkmanager")? {
            self.install_application("networkmanager")?;
        }
        if !self.is_installed("base-devl")? {
            self.install_application("base-devel")?;
        }

        let original_pacman_file = File::open("/etc/pacman.conf")?;
        let original_lines = BufReader::new(original_pacman_file).lines();
        let mut enable_multilib = false;
        let mut new_lines = original_lines
            .map(|line| {
                let unwrapped_line = line.unwrap();
                if unwrapped_line.starts_with("#[multilib]") {
                    // Crude way to signify that we are under the multilib section
                    enable_multilib = true;
                    unwrapped_line.replacen('#', "", 1)
                } else if enable_multilib
                    && unwrapped_line.starts_with("#Include = /etc/pacman.d/mirrorlist")
                {
                    enable_multilib = false;
                    unwrapped_line.replacen('#', "", 1)
                } else {
                    unwrapped_line
                }
            })
            .collect::<Vec<String>>();

        if !new_lines.contains(&"[multilib]".to_string()) {
            new_lines.push("[multilib]".to_string());
            new_lines.push("Include = /etc/pacman.d/mirrorlist".to_string());
        }

        let mut new_pacman_file = OpenOptions::new().write(true).open("/etc/pacman.conf")?;
        new_pacman_file.write_all(new_lines.join("\n").as_bytes())?;

        self.update_os_repo()?;

        if !self.is_installed("yay")? {
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
                true,
                self.config.dry_run,
            )?;
        }
        if !self.is_installed("wget")? {
            self.install_application("wget")?;
        }
        if !self.is_installed("ttf-dejavu")? {
            self.install_application("ttf-dejavu")?;
        }
        if !self.is_installed("alsa-utils")? {
            self.install_application("alsa-utils")?;
        }
        if !self.is_installed("pipewire")? {
            self.install_application("pipewire")?;
        }
        if !self.is_installed("lib32-pipewire")? {
            self.install_application("lib32-pipewire")?;
        }
        if !self.is_installed("man-db")? {
            self.install_application("man-db")?;
        }
        if !self.is_installed("pipewire-alsa")? {
            self.install_application("pipewire-alsa")?;
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
            self.install_application("terraform")?;
        }
        Ok(())
    }

    async fn install_themes(&self) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(format!("{}/.themes", self.get_home_dir()))?;
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
        self.enable_service("tlp")?;
        Ok(())
    }

    fn install_tmux(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("tmux")? {
            self.install_application("tmux")?;
        }
        if !self.is_installed("xclip")? {
            self.install_application("xclip")?;
        }
        if !self.is_installed("tmux-bash-completion")? {
            self.aur_install_application("tmux-bash-completion")?;
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
        Ok(())
    }

    fn install_vscode(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("code")? {
            self.install_application("code")?;
        }
        self.install_hunspell()?;
        let dictionary_config = &format!("{}/Code/Dictionaries", self.get_home_dir());
        let dictionaries_path = Path::new(dictionary_config);
        if !dictionaries_path.exists() {
            fs::create_dir_all(dictionaries_path)?;
        }
        unix::symlink(self, "/usr/share/hunspell/*", dictionary_config)?;
        let user_id = unix::get_user_id();
        let group_id = unix::get_group_id();
        unix::recursively_chown(dictionary_config, &user_id, &group_id)?;
        Ok(())
    }

    async fn install_wifi(&self) -> Result<(), Box<dyn Error>> {
        fs::copy(
            "/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin",
            "/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin.bak",
        )?;
        system::download_file(
            "https://github.com/kvalo/ath10k-firmware/raw/master/QCA6174/hw3.0/4.4.1.c3/firmware-6.bin_WLAN.RM.4.4.1.c3-00035",
            "/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin",
        ).await?;
        Ok(())
    }

    fn install_window_manager(&self) -> Result<(), Box<dyn Error>> {
        if self.config.gnome {
            if !self.is_installed("gnome")? {
                self.install_application("gnome")?;
            }
            if !self.is_installed("gnome-tweaks")? {
                self.install_application("gnome-tweaks")?;
            }
            if !self.is_installed("xdg-desktop-portal-gnome")? {
                self.install_application("xdg-desktop-portal-gnome ")?;
            }
            if !self.is_installed("libcanbera")? {
                self.install_application("libcanberra")?;
            }
            if !self.is_installed("libappindicator-gtk3")? {
                self.install_application("libappindicator-gtk3")?;
            }
            if !self.is_installed("gnome-browser-connector")? {
                self.aur_install_application("gnome-browser-connector")?;
            }
            if !self.is_installed("gnome-shell-extension-appindicator")? {
                self.aur_install_application("gnome-shell-extension-appindicator")?;
            }
            if !self.is_installed("gnome-shell-extension-hidetopbar-git")? {
                self.aur_install_application("gnome-shell-extension-hidetopbar-git")?;
            }
            if !self.is_installed("gnome-shell-extension-sound-output-device-chooser")? {
                self.aur_install_application("gnome-shell-extension-sound-output-device-chooser")?;
            }
            if !self.is_installed("gnome-shell-extension-nordvpn-connect-git")? {
                self.aur_install_application("gnome-shell-extension-nordvpn-connect-git")?;
            }
            self.enable_service("gdm")?;
            open::that("https://extensions.gnome.org/extension/3960/transparent-top-bar-adjustable-transparency/")?;
        }
        if self.config.kde {
            if !self.is_installed("plasma")? {
                self.install_application("plasma")?;
            }
            if !self.is_installed("plasma-wayland-session")? {
                self.install_application("plasma-wayland-session")?;
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
            if !self.is_installed("sddm-kcm")? {
                self.install_application("sddm-kcm")?;
            }
            if !self.is_installed("xdg-desktop-portal-kde")? {
                self.install_application("xdg-desktop-portal-kde")?;
            }
            if !self.is_installed("plasma5-runners-nordvpn")? {
                self.aur_install_application("plasma5-runners-nordvpn")?;
            }
            self.enable_service("sddm")?;
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
        if !self.is_installed("greenlight-bin")? {
            self.aur_install_application("greenlight-bin")?;
        }
        Ok(())
    }

    fn install_xcode(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn install_zsh(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("zsh")? {
            self.install_application("zsh")?;
        }
        if !self.is_installed("zsh-completion")? {
            self.install_application("zsh-completions")?;
        }
        unix::setup_zsh(self, None).await?;
        Ok(())
    }

    fn set_development_shortcuts(&self) -> Result<(), Box<dyn Error>> {
        linux::gnome_development_shortcuts(self)?;
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
        self.execute("pacman -Syu --noconfirm", true)?;
        Ok(())
    }

    fn update_os_repo(&self) -> Result<(), Box<dyn Error>> {
        self.execute("pacman -Sy", true)?;
        Ok(())
    }
}
