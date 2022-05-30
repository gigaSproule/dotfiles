use std::env;

#[cfg(target_os = "linux")]
use linux::Linux as sys;
#[cfg(target_os = "macos")]
use mac::Mac as sys;
#[cfg(target_os = "windows")]
use windows::Windows as sys;

use crate::system::System;
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

    let system = sys::new(&config);
    install(config, &system).await
}

fn print_help() {
    println!("install [--browsers] [--development] [--docker] [--gaming] [--gcp] [--images] [--laptop] [--modelling] [--personal] [--recording] \
        [--ripping] [--video] [--video-editing] [--vm] [--vpn]");
}
