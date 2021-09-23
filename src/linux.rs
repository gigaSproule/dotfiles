use std::cmp::Ordering;
use std::fs::{File, OpenOptions};
use std::fs;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Output;

use async_trait::async_trait;
use flate2::read::GzDecoder;
use regex::Regex;
use tar::Archive;

use crate::arch::Arch;
use crate::system;
use crate::system::System;
use crate::unix;
use crate::unix::Unix;

struct Linux {
    distro: Box<dyn System>,
}

impl Default for Linux {
    fn default() -> Self {
        let distro = match whoami::distro() {
            distro if distro.starts_with("arch") => Arch {},
            _ => panic!("Unable to determine the distro"),
        };
        Linux {
            distro: Box::new(distro),
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
        self.distro.install_codecs()
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

    fn install_eclipse(&self) {
        self.distro.install_eclipse();
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

    fn install_google_chrome(&self) {
        self.distro.install_google_chrome();
    }

    fn install_google_cloud_sdk(&self) {
        self.distro.install_google_cloud_sdk();
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

    fn install_jdk(&self) {
        self.distro.install_jdk();
    }

    fn install_keepassxc(&self) {
        self.distro.install_keepassxc();
    }

    fn install_kubectl(&self) {
        self.distro.install_kubectl();
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

    fn install_microcode(&self) {
        self.distro.install_microcode();
    }

    fn install_minikube(&self) {
        self.distro.install_minikube();
    }

    fn install_mkvtoolnix(&self) {
        self.distro.install_mkvtoolnix();
    }

    fn install_nextcloud_client(&self) {
        self.distro.install_nextcloud_client();
    }

    fn install_nodejs(&self) -> Result<(), Error> {
        self.distro.install_nodejs()
    }

    fn install_nordvpn(&self) {
        self.distro.install_nordvpn();
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

    fn install_rust(&self) {
        self.distro.install_rust();
    }

    fn install_slack(&self) {
        self.distro.install_slack();
    }

    fn install_spotify(&self) {
        self.distro.install_spotify();
    }

    fn install_steam(&self) {
        self.distro.install_steam();
    }

    fn install_sweet_home_3d(&self) {
        self.distro.install_sweet_home_3d();
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.distro.install_system_extras()
    }

    fn install_telnet(&self) {
        self.distro.install_telnet();
    }

    fn install_themes(&self) {
        self.distro.install_themes();
    }

    fn install_tlp(&self) {
        self.distro.install_tlp();
    }

    fn install_tmux(&self) {
        self.distro.install_tmux();
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

    fn install_vscode(&self) {
        self.distro.install_vscode();
    }

    fn install_wifi(&self) {
        self.distro.install_wifi();
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

    fn install_zsh(&self) {
        self.distro.install_zsh();
    }

    fn set_development_shortcuts(&self) {
        self.distro.set_development_shortcuts();
    }

    fn set_development_environment_settings(&self) {
        self.distro.set_development_environment_settings();
    }

    fn setup_power_saving_tweaks(&self) {
        self.distro.setup_power_saving_tweaks();
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

pub(crate) fn set_development_environment_settings() {
    println!("Setting mmapfs limit for Elasticsearch");
    let mut file = OpenOptions::new()
        .append(true)
        .open("/etc/sysctl.conf")?;
    writeln!(file, "vm.max_map_count=262144");
}

pub(crate) fn setup_docker(system: &dyn System) {
    system.execute(
        format!("usermod -a -G docker {}", whoami::username()).as_str(),
        true,
    );

    let output: Vec<&str> = String::from_utf8(system.execute("git ls-remote https://github.com/docker/compose", true).stdout)?
        .split('\n')
        .collect();
    let mut versions: Vec<&str> = vec![];
    let pattern = Regex::new(r".*([0-9]+\\.[0-9]+\\.[0-9]+)$").unwrap();
    for line in output {
        if line.contains("refs/tags") {
            let matches = pattern.captures(line);
            if matches.is_some() {
                versions.push(matches.unwrap().get(1)?.as_str());
            }
        }
    }
    versions.sort_by(|version_a, version_b| {
        let version_a_split: Vec<&str> = version_a.split('.').collect();
        let version_b_split: Vec<&str> = version_b.split('.').collect();
        if version_a_split.get(0)? == version_b_split.get(0)? {
            if version_a_split.get(1)? == version_b_split.get(1)? {
                if version_a_split.get(2)? == version_b_split.get(2)? {
                    return Ordering::Equal;
                }
                if version_a_split.get(2)? > version_b_split.get(2)? {
                    return Ordering::Greater;
                }
                return Ordering::Less;
            }
            if version_a_split.get(1)? > version_b_split.get(1)? {
                return Ordering::Greater;
            }
            return Ordering::Less;
        }
        if version_a_split.get(0)? > version_b_split.get(0)? {
            return Ordering::Greater;
        }
        return Ordering::Less;
    });
    let docker_compose_version = versions.last()?;

    system::download_file(&format!("https://github.com/docker/compose/releases/download/{}/docker-compose-linux-{}", docker_compose_version, std::env::consts::ARCH), "/usr/local/bin/docker-compose")

    unix::recursively_chmod("/usr/local/bin/docker-compose", &0o755, &0o755);
}

pub(crate) fn setup_power_saving_tweaks() {
    let mut file = File::open("/sys/devices/virtual/dmi/id/product_name")?;
    let mut device_name = String::new();
    file.read_to_string(&mut device_name);

    if device_name == "XPS 15 9570" {
        let mem_sleep_file = OpenOptions::new()
            .append(true)
            .open("/sys/power/mem_sleep")?;
        writeln!(mem_sleep_file, "s2idle [deep]");

        let original_grub_file = File::open("/etc/default/grub")?;
        let buffer = BufReader::new(original_grub_file);
        let new_lines = buffer.lines().map(|line| {
            if line?.starts_with("GRUB_CMDLINE_LINUX_DEFAULT=") {
                let split_line = line.unwrap().split('=');
                let mut value = split_line[1].replace("\"", "");
                value += "mem_sleep_default = deep";
                format!("{}=\"{}\"", split_line[0], value)
            } else {
                line
            }
        });

        let mut new_grub_file = OpenOptions::new().append(true).open("/etc/default/grub")?;
        new_grub_file.write_all(new_lines.join("\n").as_bytes());
    }
}

pub(crate) fn setup_tmux(system: &dyn System) {
    unix::setup_tmux(system);
    let file = OpenOptions::new().append(true).open(format!("{}/.tmux.custom.conf", system::get_home_dir()));
    writeln!(file, "bind -T copy-mode-vi y send-keys -X copy-pipe-and-cancel 'xclip -in -selection clipboard'");
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
