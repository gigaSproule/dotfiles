use std::env;

use crate::{config::parse, install::install};

#[cfg(all(not(test), target_os = "linux"))]
use whoami;

#[cfg(all(not(test), target_os = "linux"))]
use mockall::automock;
#[cfg(all(not(test), target_os = "linux"))]
#[automock()]
pub mod whoami {
    pub fn distro() -> String {
        "Ubuntu".to_string()
    }
}

#[cfg(target_os = "linux")]
mod arch;
mod config;
mod install;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod mac;
mod system;
#[cfg(target_os = "linux")]
mod ubuntu;
#[cfg(any(target_os = "linux", target_os = "macos"))]
mod unix;
#[cfg(target_os = "windows")]
mod windows;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() == 1 {
        print_help();
        return Ok(());
    }

    let config = parse(args);

    if config.help {
        print_help();
        return Ok(());
    }

    // let system = sys::new(&config);
    let system = get_system(&config);
    install(&config, &*system).await
}

#[cfg(target_os = "linux")]
fn get_system<'s>(config: &'s config::Config) -> Box<dyn system::System + 's> {
    let sudo_user = env::var("SUDO_USER");
    if sudo_user.is_err() {
        panic!("Need to run this with sudo.")
    }
    let distro_str = whoami::distro();
    match distro_str {
        distro if distro == "Arch Linux" => Box::new(arch::Arch::new(config)),
        distro if distro.starts_with("Ubuntu") => Box::new(ubuntu::Ubuntu::new(config)),
        _ => panic!("Unable to determine the distro {}.", distro_str),
    }
}

#[cfg(target_os = "macos")]
fn get_system<'s>(config: &'s config::Config) -> Box<dyn system::System + 's> {
    let sudo_user = env::var("SUDO_USER");
    if sudo_user.is_err() {
        panic!("Need to run this with sudo.")
    }
    Box::new(mac::Mac::<'s>::new(config))
}

#[cfg(target_os = "windows")]
fn get_system<'s>(config: &'s config::Config) -> Box<dyn system::System + 's> {
    if !is_elevated::is_elevated() {
        panic!("Need to run this with administrator privileges.")
    }
    Box::new(windows::Windows::<'s>::new(config))
}

fn print_help() {
    println!("install [--browsers] [--development] [--docker] [--gaming] [--gcp] [--images] [--laptop] [--modelling] [--personal] [--recording] \
        [--ripping] [--video] [--video-editing] [--vm] [--vpn]");
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    const CONFIG: config::Config = config::Config {
        browsers: false,
        development: false,
        docker: false,
        dry_run: false,
        gaming: false,
        gcp: false,
        gnome: false,
        images: false,
        kde: false,
        help: false,
        laptop: false,
        modelling: false,
        personal: false,
        recording: false,
        ripping: false,
        video: false,
        video_editing: false,
        vm: false,
        vpn: false,
    };

    #[test]
    #[serial]
    #[should_panic(expected = "Need to run this with sudo.")]
    #[cfg(target_os = "linux")]
    fn test_get_system_throws_error_if_sudo_user_not_set() {
        env::remove_var("SUDO_USER");
        get_system(&CONFIG);
    }

    #[test]
    #[serial]
    #[cfg(target_os = "linux")]
    fn test_get_system_returns_arch() {
        env::set_var("SUDO_USER", "username");
        let ctx = mock_whoami::distro_context();
        ctx.expect().returning(|| "Arch Linux".to_string());
        get_system(&CONFIG);
    }

    #[test]
    #[serial]
    #[cfg(target_os = "linux")]
    fn test_get_system_returns_ubuntu() {
        env::set_var("SUDO_USER", "username");
        let ctx = mock_whoami::distro_context();
        ctx.expect().returning(|| "Ubuntu 22.04 LTS".to_string());
        get_system(&CONFIG);
    }

    #[test]
    #[serial]
    #[should_panic(expected = "Need to run this with sudo.")]
    #[cfg(target_os = "macos")]
    fn test_get_system_throws_error_if_sudo_user_not_set() {
        env::remove_var("SUDO_USER");
        get_system(&CONFIG);
    }

    #[test]
    #[serial]
    #[cfg(target_os = "macos")]
    fn test_get_system_returns_mac() {
        env::set_var("SUDO_USER", "username");
        get_system(&CONFIG);
    }
}
