use crate::{config::parse, install::install};
use log::{debug, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::Config;
#[cfg(all(test, any(target_os = "linux", target_os = "windows")))]
use mockall::automock;
use std::env;
#[cfg(all(test, any(target_os = "linux", target_os = "windows")))]
use std::sync::Mutex;
#[cfg(all(test, target_os = "linux"))]
static DISTRO_VALUE: Mutex<&str> = Mutex::new("Ubuntu");
#[cfg(all(test, target_os = "linux"))]
#[automock()]
pub mod whoami {
    use crate::DISTRO_VALUE;

    pub fn distro() -> Result<String, whoami::Error> {
        Ok(DISTRO_VALUE
            .lock()
            .expect("Failed to lock the DISTRO_VALUE mutex.")
            .to_string())
    }
}
#[cfg(all(test, target_os = "windows"))]
static IS_ELEVATED_VALUE: Mutex<bool> = Mutex::new(true);
#[cfg(all(test, target_os = "windows"))]
#[automock()]
pub mod is_elevated {
    use crate::IS_ELEVATED_VALUE;

    pub fn is_elevated() -> bool {
        IS_ELEVATED_VALUE
            .lock()
            .expect("Failed to lock the IS_ELEVATED_VALUE mutex.")
            .clone()
    }
}
#[cfg(target_os = "linux")]
mod arch;
mod config;
mod error;
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

    if args.len() == 1 {
        print_help();
        return Ok(());
    }

    let config = parse(args);

    configure_logging(&config);

    if config.help {
        print_help();
        return Ok(());
    }

    let system = get_system(&config);
    debug!("System: {:?}", system);
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
        Ok(distro) if distro == "Arch Linux" => Box::new(arch::Arch::new(config)),
        Ok(distro) if distro.starts_with("Ubuntu") => Box::new(ubuntu::Ubuntu::new(config)),
        Ok(distro) => panic!("Unable to determine the distro {distro}."),
        Err(msg) => panic!("Unable to determine the distro {msg}."),
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
    println!(
        "install [--browsers] [--development] [--docker] [--gaming] [--gcp] [--images] \
        [--laptop] [--modelling] [--personal] [--printer] [--recording] [--ripping] [--video] \
        [--video-editing] [--vm] [--vpn] [--debug]"
    );
}

fn configure_logging(config: &config::Config) {
    let stdout = ConsoleAppender::builder().build();

    let level = if config.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let log_config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("dotfiles", level))
        .build(Root::builder().appender("stdout").build(LevelFilter::Error))
        .unwrap();

    log4rs::init_config(log_config).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    const CONFIG: config::Config = config::Config {
        browsers: false,
        cli_only: false,
        debug: false,
        development: false,
        docker: false,
        dry_run: false,
        gaming: false,
        gcp: false,
        gnome: false,
        images: false,
        infrastructure: false,
        kde: false,
        help: false,
        laptop: false,
        modelling: false,
        personal: false,
        printer: false,
        recording: false,
        ripping: false,
        video: false,
        video_editing: false,
        vm: false,
        vpn: false,
        wsl: false,
    };

    #[test]
    #[serial]
    #[should_panic(expected = "Need to run this with sudo.")]
    #[cfg(target_os = "linux")]
    fn test_get_system_throws_error_if_sudo_user_not_set_linux() {
        env::remove_var("SUDO_USER");
        get_system(&CONFIG);
    }

    #[test]
    #[serial]
    #[should_panic(expected = "Unable to determine the distro Unknown Linux.")]
    #[cfg(target_os = "linux")]
    fn test_get_system_throws_error_if_distro_not_supported() {
        env::set_var("SUDO_USER", "username");
        set_distro_value("Unknown Linux");
        get_system(&CONFIG);
    }

    #[test]
    #[serial]
    #[cfg(target_os = "linux")]
    fn test_get_system_returns_arch() {
        env::set_var("SUDO_USER", "username");
        set_distro_value("Arch Linux");
        get_system(&CONFIG);
    }

    #[test]
    #[serial]
    #[cfg(target_os = "linux")]
    fn test_get_system_returns_ubuntu() {
        env::set_var("SUDO_USER", "username");
        set_distro_value("Ubuntu 22.04 LTS");
        get_system(&CONFIG);
    }

    #[cfg(target_os = "linux")]
    fn set_distro_value(distro_value: &'static str) {
        let mut mtx = DISTRO_VALUE
            .lock()
            .expect("Failed to lock the DISTRO_VALUE mutex.");
        *mtx = distro_value;
    }

    #[test]
    #[serial]
    #[should_panic(expected = "Need to run this with sudo.")]
    #[cfg(target_os = "macos")]
    fn test_get_system_throws_error_if_sudo_user_not_set_mac() {
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

    #[test]
    #[serial]
    #[should_panic(expected = "Need to run this with administrator privileges.")]
    #[cfg(target_os = "windows")]
    fn test_get_system_throws_error_if_is_not_elevated_windows() {
        set_is_elevated_value(false);
        get_system(&CONFIG);
    }

    #[test]
    #[serial]
    #[cfg(target_os = "windows")]
    fn test_get_system_returns_windows() {
        set_is_elevated_value(true);
        get_system(&CONFIG);
    }

    #[cfg(target_os = "windows")]
    fn set_is_elevated_value(is_elevated_value: bool) {
        let mut mtx = IS_ELEVATED_VALUE
            .lock()
            .expect("Failed to lock the IS_ELEVATED_VALUE mutex.");
        *mtx = is_elevated_value;
    }
}
