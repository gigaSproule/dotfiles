use std::io::Error;
use std::process::Output;

use async_trait::async_trait;

use crate::system::System;
use crate::unix::Unix;

pub(crate) struct Mac {}

impl Default for Mac {
    fn default() -> Self {
        Mac {}
    }
}

#[async_trait]
impl System for Mac {
    fn execute(&self, command: &str, super_user: bool) -> Output {
        self.unix.execute(command, super_user)
    }

    fn install_applications(&self, applications: Vec<&str>) -> Output {
        todo!()
    }

    fn install_android_studio(&self) {
        todo!()
    }

    fn install_blender(&self) {
        todo!()
    }

    fn install_bluetooth(&self) {
        todo!()
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_conemu(&self) {
        todo!()
    }

    fn install_cryptomator(&self) {
        todo!()
    }

    fn install_curl(&self) {
        todo!()
    }

    fn install_davinci_resolve(&self) -> Result<(), Error> {
        todo!()
    }

    fn install_discord(&self) {
        todo!()
    }

    fn install_docker(&self) -> Result<(), Error> {
        todo!()
    }

    fn install_dropbox(&self) {
        todo!()
    }

    async fn install_eclipse(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_epic_games(&self) {
        todo!()
    }

    fn install_firefox(&self) {
        todo!()
    }

    fn install_firmware_updater(&self) {
        todo!()
    }

    fn install_gog_galaxy(&self) {
        todo!()
    }

    async fn install_google_chrome(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_google_cloud_sdk(&self) -> Result<(), std::io::Error> {
        todo!()
    }

    fn install_google_drive(&self) {
        todo!()
    }

    fn install_git(&self) -> Result<(), Error> {
        todo!()
    }

    fn install_gimp(&self) {
        todo!()
    }

    fn install_gpg(&self) {
        todo!()
    }

    fn install_gradle(&self) {
        todo!()
    }

    fn install_graphic_card_tools(&self) {
        todo!()
    }

    fn install_graphic_card_laptop_tools(&self) {
        todo!()
    }

    fn install_groovy(&self) {
        todo!()
    }

    fn install_handbrake(&self) {
        todo!()
    }

    fn install_inkscape(&self) {
        todo!()
    }

    fn install_insync(&self) {
        todo!()
    }

    fn install_intellij(&self) {
        todo!()
    }

    fn install_jdk(&self) -> Result<(), std::io::Error> {
        todo!()
    }

    fn install_keepassxc(&self) {
        todo!()
    }

    async fn install_kubectl(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_helm(&self) {
        todo!()
    }

    fn install_latex(&self) {
        todo!()
    }

    fn install_lutris(&self) {
        todo!()
    }

    fn install_maven(&self) {
        todo!()
    }

    fn install_makemkv(&self) {
        todo!()
    }

    fn install_microcode(&self) -> Result<(), std::io::Error> {
        todo!()
    }

    async fn install_minikube(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_mkvtoolnix(&self) {
        todo!()
    }

    fn install_nextcloud_client(&self) {
        todo!()
    }

    async fn install_nodejs(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn install_nordvpn(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_nvidia_tools(&self) {
        todo!()
    }

    fn install_nvidia_laptop_tools(&self) {
        todo!()
    }

    fn install_obs_studio(&self) {
        todo!()
    }

    fn install_onedrive(&self) {
        todo!()
    }

    fn install_origin(&self) {
        todo!()
    }

    fn install_powertop(&self) {
        todo!()
    }

    fn install_python(&self) {
        todo!()
    }

    fn install_rust(&self) {
        todo!()
    }

    fn install_slack(&self) {
        todo!()
    }

    fn install_spotify(&self) {
        todo!()
    }

    fn install_steam(&self) {
        todo!()
    }

    fn install_sweet_home_3d(&self) {
        todo!()
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_telnet(&self) {
        todo!()
    }

    async fn install_themes(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_tlp(&self) {
        todo!()
    }

    fn install_tmux(&self) {
        todo!()
    }

    fn install_vim(&self) {
        todo!()
    }

    fn install_vlc(&self) {
        todo!()
    }

    fn install_vm_tools(&self) {
        todo!()
    }

    fn install_vscode(&self) {
        todo!()
    }

    async fn install_wifi(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_window_manager(&self) {
        todo!()
    }

    fn install_wget(&self) {
        todo!()
    }

    fn install_wine(&self) {
        todo!()
    }

    fn install_xcode(&self) {
        todo!()
    }

    async fn install_zsh(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn set_development_shortcuts(&self) {
        todo!()
    }

    fn set_development_environment_settings(&self) {
        todo!()
    }

    fn setup_power_saving_tweaks(&self) {
        todo!()
    }

    fn update_os(&self) {
        todo!()
    }

    fn update_os_repo(&self) {
        todo!()
    }
}
