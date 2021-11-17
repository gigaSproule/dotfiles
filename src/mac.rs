use std::fs;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::Write;
use std::process::Output;

use async_trait::async_trait;

use crate::system::{self, System};
use crate::unix;

pub(crate) struct Mac {}

impl Default for Mac {
    fn default() -> Self {
        Mac {}
    }
}

impl Mac {
    fn app_store_install_application(&self, application_id: &str) -> Output {
        self.execute(&format!("mas install {}", application_id), true)
    }

    fn cask_install_application(&self, application: &str) -> Output {
        self.execute(&format!("brew install --cask {}", application), false)
    }
}

#[async_trait]
impl System for Mac {
    fn execute(&self, command: &str, super_user: bool) -> Output {
        unix::execute(command, super_user)
    }

    fn install_applications(&self, applications: Vec<&str>) -> Output {
        self.execute(&format!("brew install {}", applications.join(" ")), false)
    }

    fn install_android_studio(&self) {
        self.cask_install_application("android-studio");
    }

    fn install_blender(&self) {
        self.cask_install_application("blender");
    }

    fn install_bluetooth(&self) {
        // no-op
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn std::error::Error>> {
        system::setup_codecs().await
    }

    fn install_conemu(&self) {
        // no-op
    }

    fn install_cryptomator(&self) {
        self.cask_install_application("cryptomator");
    }

    fn install_curl(&self) {
        // no-op
    }

    fn install_davinci_resolve(&self) -> Result<(), Error> {
        open::that("https://www.blackmagicdesign.com/uk/products/davinciresolve/studio")
    }

    fn install_discord(&self) {
        self.cask_install_application("discord");
    }

    fn install_docker(&self) -> Result<(), Error> {
        self.cask_install_application("docker");
        Ok(())
    }

    fn install_dropbox(&self) {
        self.cask_install_application("dropbox");
    }

    async fn install_eclipse(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("eclipse-java");
        Ok(())
    }

    fn install_epic_games(&self) {
        // no-op
    }

    fn install_firefox(&self) {
        self.cask_install_application("firefox");
    }

    fn install_firmware_updater(&self) {
        // no-op
    }

    fn install_gog_galaxy(&self) {
        // no-op
    }

    async fn install_google_chrome(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("google-chrome");
        Ok(())
    }

    fn install_google_cloud_sdk(&self) -> Result<(), std::io::Error> {
        todo!()
    }

    fn install_google_drive(&self) {
        self.cask_install_application("google-drive");
    }

    fn install_git(&self) -> Result<(), Error> {
        self.install_application("git");
        system::setup_git_config(self)
    }

    fn install_gimp(&self) {
        self.cask_install_application("gimp");
    }

    fn install_gpg(&self) {
        self.cask_install_application("gpg-suite");
    }

    fn install_gradle(&self) {
        self.install_applications(vec!["gradle", "gradle-completion"]);
    }

    fn install_graphic_card_tools(&self) {
        // no-op
    }

    fn install_graphic_card_laptop_tools(&self) {
        // no-op
    }

    fn install_groovy(&self) {
        self.install_application("groovy");
    }

    fn install_handbrake(&self) {
        self.cask_install_application("handbrake");
    }

    fn install_inkscape(&self) {
        self.cask_install_application("inkscape");
    }

    fn install_insync(&self) {
        // no-op
    }

    fn install_intellij(&self) {
        self.cask_install_application("intellij-idea");
    }

    fn install_jdk(&self) -> Result<(), std::io::Error> {
        self.install_application("openjdk");
        // TODO: Replace `/opt/homebrew` with `$(brew --prefix)` (which needs to return correct value)
        unix::symlink(
            "/opt/homebrew/openjdk/libexec/openjdk.jdk",
            "/Library/Java/JavaVirtualMachines/openjdk.jdk",
        );
        unix::set_java_home(".zshrc.custom", "$(/usr/libexec/java_home)")?;
        Ok(())
    }

    fn install_keepassxc(&self) {
        self.cask_install_application("keepassxc");
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
        // no-op
    }

    fn install_maven(&self) {
        self.install_application("maven");
    }

    fn install_makemkv(&self) {
        todo!()
    }

    fn install_microcode(&self) -> Result<(), std::io::Error> {
        Ok(())
    }

    async fn install_minikube(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_mkvtoolnix(&self) {
        todo!()
    }

    fn install_nextcloud_client(&self) {
        self.cask_install_application("nextcloud");
    }

    async fn install_nodejs(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("nvm");
        let mut zshrc = OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{}/.zshrc.custom", system::get_home_dir()))?;
        writeln!(zshrc, "export NVM_DIR=\"$HOME/.nvm\"")?;
        writeln!(zshrc, "[ -s \"/opt/homebrew/opt/nvm/nvm.sh\" ] && . \"/opt/homebrew/opt/nvm/nvm.sh\"  # This loads nvm")?;
        writeln!(zshrc, "[ -s \"/opt/homebrew/opt/nvm/etc/bash_completion.d/nvm\" ] && . \"/opt/homebrew/opt/nvm/etc/bash_completion.d/nvm\"  # This loads nvm bash_completion")?;

        let mut bashrc = OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{}/.bashrc.custom", system::get_home_dir()))?;
        writeln!(bashrc, "export NVM_DIR=\"$HOME/.nvm\"")?;
        writeln!(bashrc, "[ -s \"/opt/homebrew/opt/nvm/nvm.sh\" ] && . \"/opt/homebrew/opt/nvm/nvm.sh\"  # This loads nvm")?;
        writeln!(bashrc, "[ -s \"/opt/homebrew/opt/nvm/etc/bash_completion.d/nvm\" ] && . \"/opt/homebrew/opt/nvm/etc/bash_completion.d/nvm\"  # This loads nvm bash_completion")?;

        self.execute("nvm install node --latest-npm", false);
        self.execute("npm install --global yarn", false);
        Ok(())
    }

    async fn install_nordvpn(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.app_store_install_application("1116599239");
        Ok(())
    }

    fn install_nvidia_tools(&self) {
        // no-op
    }

    fn install_nvidia_laptop_tools(&self) {
        // no-op
    }

    fn install_obs_studio(&self) {
        self.cask_install_application("obs");
    }

    fn install_onedrive(&self) {
        self.cask_install_application("onedrive");
    }

    fn install_origin(&self) {
        // no-op
    }

    fn install_powertop(&self) {
        // no-op
    }

    fn install_python(&self) {
        self.install_application("python");
    }

    async fn install_rust(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("rustup");
        self.execute("rustup-init -y", true);
        Ok(())
    }

    fn install_slack(&self) {
        self.app_store_install_application("803453959");
    }

    fn install_spotify(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("spotify");
        Ok(())
    }

    fn install_steam(&self) {
        self.cask_install_application("steam");
    }

    fn install_sweet_home_3d(&self) {
        self.cask_install_application("sweet-home3d");
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn std::error::Error>> {
        system::download_file(
            "https://raw.githubusercontent.com/Homebrew/install/master/install.sh",
            "brew-install",
        )
        .await?;
        self.execute("chmod +x brew-install", false);
        self.execute("./brew-install", false);
        fs::remove_file("brew-install")?;

        self.install_application("mas");
        self.cask_install_application("scroll-reverser");
        Ok(())
    }

    fn install_telnet(&self) {
        self.install_application("telnet");
    }

    async fn install_themes(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_tlp(&self) {
        // no-op
    }

    fn install_tmux(&self) -> Result<(), std::io::Error> {
        self.install_applications(vec!["tmux", "reattach-to-user-namespace"]);
        unix::setup_tmux()?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{}/.tmux.custom.conf", system::get_home_dir()))?;
        writeln!(file, "bind -T copy-mode-vi y send-keys -X copy-pipe-and-cancel 'reattach-to-user-namespace pbcopy'")?;
        Ok(())
    }

    fn install_vim(&self) {
        // no-op
    }

    fn install_vlc(&self) {
        self.cask_install_application("vlc");
    }

    fn install_vm_tools(&self) {
        todo!()
    }

    fn install_vscode(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("visual-studio-code");
        Ok(())
    }

    async fn install_wifi(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_window_manager(&self) {
        // no-op
    }

    fn install_wget(&self) {
        // no-op
    }

    fn install_wine(&self) {
        // no-op
    }

    fn install_xcode(&self) {
        self.app_store_install_application("497799835");
    }

    async fn install_zsh(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["zsh", "zsh-autosuggestions"]);
        unix::setup_zsh(self, Some("/usr/local/bin/zsh")).await?;
        Ok(())
    }

    fn set_development_shortcuts(&self) {
        // no-op
    }

    fn set_development_environment_settings(&self) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn setup_power_saving_tweaks(&self) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn update_os(&self) {
        self.update_os_repo();
        self.execute("brew -y upgrade", false);
    }

    fn update_os_repo(&self) {
        self.execute("brew update", false);
    }
}
