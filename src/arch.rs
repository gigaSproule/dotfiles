use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::Output;

use async_trait::async_trait;

use crate::{linux, system, unix};
use crate::system::System;

impl Arch {
    fn aur_install_application(&self, application: &str) -> Output {
        self.aur_install_applications([application])
    }

    fn aur_install_applications(&self, applications: Vec<&str>) -> Output {
        self.execute(
            format!("yay -S --noconfirm --needed {}", applications.join(" ")),
            false,
        )
    }

    fn enable_service(&self, service: &str) {
        self.execute(
            format!("systemctl enable service {}", service).as_str(),
            true,
        )
    }

    fn setup_docker(&self) {
        self.execute(
            format!("usermod -a -G docker {}", whoami::username()).as_str(),
            true,
        );

        let output = self
            .execute("git ls-remote https://github.com/docker/compose", true)
            .stdout;
        output = output.split('\n');
        versions = [];
        pattern = re.compile(".*([0-9]+\\.[0-9]+\\.[0-9]+)$");
        // for line in output {
        //   if 'refs/tags' in line:
        //       match = pattern.match(line)
        //       if match is not None:
        //           versions.append(match.groups()[0])
        // docker_compose_version = sorted(versions, key=StrictVersion)[len(versions) - 1]
        //
        // urllib.request.urlretrieve('https://github.com/docker/compose/releases/download/%s/docker-compose-%s-%s' % (
        //                                                      docker_compose_version, platform.system(), platform.machine()), '/usr/local/bin/docker-compose')
        // self.recursively_chmod('/usr/local/bin/docker-compose', 0o755)
        //
        // if not os.path.exists('/etc/docker'):
        //     self.make_directory('/etc/docker')
        //
        // with open('/etc/docker/daemon.json', 'w') as f:
        //     f.write('{\n'
        //                                         '"dns": ["10.14.98.21", "10.14.98.22", "8.8.8.8"]\n'
        //                 '}')
    }
}

#[async_trait]
impl System for Arch {
    fn execute(&self, command: &str, super_user: bool) -> Output {
        unix::execute(command, super_user)
    }

    fn install_applications(&self, application: Vec<&str>) -> Output {
        self.execute(
            format!("pacman -S --noconfirm --needed {}", application.join(" ")),
            true,
        )
    }

    fn install_android_studio(&self) {
        self.aur_install_application("android-studio");
    }

    fn install_blender(&self) {
        self.install_application("blender")
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
        ])?;
        system::setup_codecs()?;
        let user_id = system.get_user_id();
        let group_id = system.get_group_id();
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
        self.aur_install_application("cryptomator");
    }

    fn install_curl(&self) {
        self.install_application("curl");
    }

    fn install_davinci_resolve(&self) -> Result<(), std::io::Error> {
        self.aur_install_application("davinci-resolve-studio")
    }

    fn install_discord(&self) {
        self.install_application("discord");
    }

    fn install_docker(&self) -> Result<(), std::io::Error> {
        self.install_application("docker")?;
        self.setup_docker()?;
        Ok(())
    }

    fn install_dropbox(&self) {
        self.install_applications(vec!["dropbox", "nautilus-dropbox"])
    }

    fn install_eclipse(&self) {
        self.aur_install_application("eclipse-jee");
        if Path::new("/opt/eclipse").exists() {
            fs::create_dir_all("/opt/eclipse");
        }

        self.download_file("https://projectlombok.org/downloads/lombok.jar", "/opt/eclipse/lombok.jar");

        let mut file = OpenOptions::new()
            .append(true)
            .open("/opt/eclipse/eclipse.ini")?;

        writeln!(file, "-javaagent:/opt/eclipse/lombok.jar")?;
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

    fn install_google_chrome(&self) {
        self.aur_install_application("google-chrome");
    }

    fn install_google_cloud_sdk(&self) {
        self.aur_install_application("google-cloud-sdk");
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

    fn install_jdk(&self) {
        self.install_application("jdk-openjdk");
        unix::set_java_home(".zshrc.custom", "/usr/lib/jvm/default");
        unix::set_java_home(".bashrc.custom", "/usr/lib/jvm/default");
    }

    fn install_keepassxc(&self) {
        self.install_application("keepassxc");
    }

    fn install_kubectl(&self) {
        self.install_application("kubectl");
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

    fn install_microcode(&self) {
        let file = File::open("/proc/cpuinfo")?;
        let buffer = BufReader::new(file);
        let cpu_name = buffer.lines().find_map(|line| {
            if line.is_ok() && line.unwrap().starts_with("vendor_id") {
                line.unwrap().split(":").next()?
            }
            None
        });
        if cpu_name.is_none() {
            return;
        }
        if cpu_name.unwrap() == "GenuineIntel" {
            self.install_application("intel-ucode");
        } else {
            self.install_application("amd-ucode");
        }
    }

    fn install_minikube(&self) {
        self.install_application("minikube");
    }

    fn install_mkvtoolnix(&self) {
        self.install_application("mkvtoolnix-gui");
    }

    fn install_nextcloud_client(&self) {
        self.install_application("nextcloud-client");
    }

    fn install_nodejs(&self) -> Result<(), std::io::Error> {
        self.aur_install_application("nvm");
        unix::setup_nodejs(&self)
    }

    fn install_nordvpn(&self) {
        self.aur_install_application("nordvpn-bin");
        self.enable_service("nordvpnd");
    }

    fn install_nvidia_tools(&self) {
        self.install_applications(vec!["nvidia", "nvidia-utils", "lib32-nvidia-utils", "nvidia-settings", "vulkan-icd-loader", "lib32-vulkan-icd-loader"])
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

    fn install_rust(&self) {
        self.install_application("rustup");
        self.execute("rustup default stable");
    }

    fn install_slack(&self) {
        self.aur_install_application("slack-desktop");
    }

    fn install_spotify(&self) {
        self.aur_install_application("spotify");
    }

    fn install_steam(&self) {
        self.install_application("steam");
    }

    fn install_sweet_home_3d(&self) {
        self.install_application("sweethome3d");
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["'base-devel'," "ttf-dejavu"]);

        let original_pacman_file = File::open("/etc/pacman.conf")?;
        let original_lines = BufReader::new(original_pacman_file).lines();
        let mut enable_multilib = false;
        let new_lines = original_lines.map(|line| {
            return if line?.startswith("#[multilib]") {
                // Crude way to signify that we are under the multilib section
                enable_multilib = true;
                line?.replacen("#", "", 1)
            } else if enable_multilib && line?.startswith("#Include = /etc/pacman.d/mirrorlist") {
                enable_multilib = false;
                line?.replacen("#", "", 1)
            } else {
                line?
            };
        }).collect::<Vec<String>>();

        let mut new_pacman_file = OpenOptions::new()
            .write(true)
            .open("/etc/pacman.conf")?;
        new_pacman_file.write_all(new_lines.join("\n").as_bytes());

        self.install_applications(vec!["yay", "wget"])
    }

    fn install_telnet(&self) {
        self.install_application("inetutils");
    }

    fn install_themes(&self) {
        fs::create_dir_all(&format!("{}/.themes", self.get_home_dir()));
        self.install_specific_themes();
        let user_id = unix::get_user_id();
        let group_id = unix::get_group_id();
        unix::recursively_chown(&format!("{}/.themes", self.get_home_dir()), &user_id, &group_id);
    }

    fn install_tlp(&self) {
        self.install_application("tlp");
        self.enable_service("tlp")
    }

    fn install_tmux(&self) {
        self.install_applications(vec!["tmux", "xclip"]);
        self.aur_install_application("tmux-bash-completion");
        linux::setup_tmux(&self);
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

    fn install_vscode(&self) {
        self.install_application("code");
    }

    fn install_wifi(&self) {
        fs::copy("/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin",
                 "/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin.bak");
        system::download_file(
            "https://github.com/kvalo/ath10k-firmware/raw/master/QCA6174/hw3.0/4.4.1.c3/firmware-6.bin_WLAN.RM.4.4.1.c3-00035",
            "/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin");
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

    fn install_zsh(&self) {
        self.install_applications(vec!["zsh", "zsh-completions"]);
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
        self.execute("pacman -Syu --noconfirm");
    }

    fn update_os_repo(&self) {
        self.execute("pacman -Sy");
    }
}
