use std::env;

#[cfg(target_os = "linux")]
use linux::Linux as sys;
#[cfg(target_os = "macos")]
use mac::Mac as sys;
#[cfg(target_os = "windows")]
use windows::Windows as sys;

use crate::system::System;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
mod arch;
#[cfg(target_os = "linux")]
mod ubuntu;
#[cfg(target_os = "macos")]
mod mac;
mod system;
#[cfg(any(target_os = "linux", target_os = "macos"))]
mod unix;
#[cfg(target_os = "windows")]
mod windows;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_help();
        return Ok(());
    }

    if args.contains(&"--help".to_string()) {
        print_help();
        return Ok(());
    }

    let browsers = args.contains(&"--browsers".to_string());
    let development = args.contains(&"--development".to_string());
    let docker = args.contains(&"--docker".to_string());
    let gaming = args.contains(&"--gaming".to_string());
    let gcp = args.contains(&"--gcp".to_string());
    let images = args.contains(&"--images".to_string());
    let laptop = args.contains(&"--laptop".to_string());
    let modelling = args.contains(&"--modelling".to_string());
    let personal = args.contains(&"--personal".to_string());
    let recording = args.contains(&"--recording".to_string());
    let ripping = args.contains(&"--ripping".to_string());
    let video = args.contains(&"--video".to_string());
    let video_editing = args.contains(&"--video_editing".to_string());
    let vm = args.contains(&"--vm".to_string());
    let vpn = args.contains(&"--vpn".to_string());

    let system = sys {};

    system.setup_user_bin()?;

    println!("Installing Distro Specific Extras");
    system.install_system_extras().await?;
    system.update_os();

    println!("Installing Window Manager");
    system.install_window_manager();
    println!("Installing Graphic Card Tools");
    system.install_graphic_card_tools();

    println!("Installing Cryptomator");
    system.install_cryptomator();
    println!("Installing ConEmu");
    system.install_conemu();
    println!("Installing Curl");
    system.install_curl();
    println!("Installing KeepassXC");
    system.install_keepassxc();
    println!("Installing tmux");
    system.install_tmux();
    println!("Installing Vim");
    system.install_vim();
    println!("Installing Wget");
    system.install_wget();
    println!("Installing ZSH");
    system.install_zsh();

    if browsers {
        println!("Installing Firefox");
        system.install_firefox();
        println!("Installing Google Chrome");
        system.install_google_chrome();
    }

    if development {
        println!("Installing Android Studio");
        // system.install_android_studio();
        println!("Installing Eclipse");
        // system.install_eclipse();
        println!("Installing Gradle");
        system.install_gradle();
        println!("Installing Git");
        system.install_git()?;
        println!("Installing Groovy");
        system.install_groovy();
        println!("Installing IntelliJ");
        system.install_intellij();
        println!("Installing Java");
        system.install_jdk();
        println!("Installing Maven");
        system.install_maven();
        println!("Installing NodeJS");
        system.install_nodejs()?;
        println!("Installing Python");
        system.install_python();
        println!("Installing Rust");
        system.install_rust();
        println!("Installing Slack");
        system.install_slack();
        println!("Installing VSCode");
        system.install_vscode();
        println!("Installing Xcode");
        system.install_xcode();
        println!("Setting development specific shortcuts");
        system.set_development_shortcuts();
        println!("Setting development environment settings");
        system.set_development_environment_settings();
    }

    if docker {
        println!("Installing Docker");
        system.install_docker()?;
        println!("Installing Kubectl");
        system.install_kubectl();
        println!("Installing Helm");
        system.install_helm();
        println!("Installing Minikube");
        // system.install_minikube();
    }

    if gaming {
        println!("Installing Discord");
        system.install_discord();
        println!("Installing Epic Games");
        system.install_epic_games();
        println!("Installing GOG Galaxy");
        system.install_gog_galaxy();
        println!("Installing Lutris");
        system.install_lutris();
        println!("Installing Origin");
        system.install_origin();
        println!("Installing Steam");
        system.install_steam();
        println!("Installing Wine");
        system.install_wine();
    }

    if gcp {
        println!("Installing Google Cloud SDK");
        system.install_google_cloud_sdk();
    }

    if images {
        println!("Installing Gimp");
        system.install_gimp();
        println!("Installing Inkscape");
        system.install_inkscape();
    }

    if laptop {
        println!("Installing Bluetooth");
        system.install_bluetooth();
        println!("Installing FWUPD");
        system.install_firmware_updater();
        println!("Installing Graphic Card Tools");
        system.install_graphic_card_tools();
        println!("Installing Graphics Card Tools for Laptop");
        system.install_graphic_card_laptop_tools();
        println!("Installing Microcode");
        system.install_microcode();
        println!("Installing Powertop");
        system.install_powertop();
        println!("Installing TLP");
        system.install_tlp();
        println!("Install WiFi");
        system.install_wifi();
        println!("Setup power saving tweaks");
        system.setup_power_saving_tweaks();
    }

    if modelling {
        println!("Installing Blender");
        system.install_blender();
    }

    if personal {
        println!("Installing Dropbox");
        system.install_dropbox();
        println!("Installing Google Drive");
        system.install_google_drive();
        println!("Installing GPG");
        system.install_gpg();
        println!("Installing Insync");
        system.install_insync();
        println!("Installing LaTeX");
        system.install_latex();
        println!("Installing Nextcloud Client");
        system.install_nextcloud_client();
        println!("Installing OneDrive");
        system.install_onedrive();
        println!("Installing Spotify");
        system.install_spotify();
        println!("Installing SweetHome3D");
        system.install_sweet_home_3d();
        println!("Installing themes");
        system.install_themes();
    }

    if recording {
        println!("Installing OBS Studio");
        system.install_obs_studio();
    }

    if ripping {
        println!("Installing Handbrake");
        system.install_handbrake();
        println!("Installing MakeMKV");
        system.install_makemkv();
        println!("Installing MKVToolNix");
        system.install_mkvtoolnix();
    }

    if video {
        println!("Installing Codecs");
        system.install_codecs().await?;
        println!("Installing VLC");
        system.install_vlc();
    }

    if video_editing {
        println!("Installing DaVinci Resolve");
        system.install_davinci_resolve()?;
    }

    if vm {
        println!("Installing VM Tools");
        system.install_vm_tools();
    }

    if vpn {
        println!("Installing NordVPN");
        system.install_nordvpn();
    }

    Ok(())
}

fn print_help() {
    println!("install.py [--browsers] [--development] [--docker] [--gaming] [--gcp] [--images] [--laptop] [--modelling] [--personal] [--recording] \
        [--ripping] [--video] [--video_editing] [--vm] [--vpn]");
}
