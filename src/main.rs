use std::env;

use crate::{config::parse, install::install};

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
    Box::new(mac::Mac::<'s>::new(config))
}

#[cfg(target_os = "windows")]
fn get_system<'s>(config: &'s config::Config) -> Box<dyn system::System + 's> {
    Box::new(windows::Windows::<'s>::new(config))
}

fn print_help() {
    println!("install [--browsers] [--development] [--docker] [--gaming] [--gcp] [--images] [--laptop] [--modelling] [--personal] [--recording] \
        [--ripping] [--video] [--video-editing] [--vm] [--vpn]");
}
