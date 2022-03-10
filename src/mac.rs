use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

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
    fn app_store_install_application(
        &self,
        application_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.execute(&format!("mas install {}", application_id), true)
    }

    fn cask_install_application(
        &self,
        application: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.execute(&format!("brew install --cask {}", application), false)
    }

    fn get_brew_prefix(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.execute("brew --prefix", false)
    }
}

#[async_trait]
impl System for Mac {
    fn execute(
        &self,
        command: &str,
        super_user: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        unix::execute(command, super_user)
    }

    fn get_home_dir(&self) -> String {
        system::get_home_dir()
    }

    fn install_applications(
        &self,
        applications: Vec<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.execute(&format!("brew install {}", applications.join(" ")), false)
    }

    fn install_android_studio(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("android-studio")?;
        Ok(())
    }

    fn install_blender(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("blender")?;
        Ok(())
    }

    fn install_bluetooth(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn std::error::Error>> {
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

    fn install_conemu(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_cryptomator(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("cryptomator")?;
        Ok(())
    }

    fn install_curl(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_davinci_resolve(&self) -> Result<(), Box<dyn std::error::Error>> {
        open::that("https://www.blackmagicdesign.com/uk/products/davinciresolve/studio")?;
        Ok(())
    }

    fn install_discord(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("discord")?;
        Ok(())
    }

    fn install_docker(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("docker")?;
        Ok(())
    }

    fn install_dropbox(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("dropbox")?;
        Ok(())
    }

    async fn install_eclipse(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("eclipse-java")?;
        Ok(())
    }

    fn install_epic_games(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_firefox(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("firefox")?;
        Ok(())
    }

    fn install_firmware_updater(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_gog_galaxy(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn install_google_chrome(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("google-chrome")?;
        Ok(())
    }

    fn install_google_cloud_sdk(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_google_drive(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("google-drive")?;
        Ok(())
    }

    fn install_git(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("git")?;
        system::setup_git_config(self)?;
        Ok(())
    }

    fn install_gimp(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("gimp")?;
        Ok(())
    }

    fn install_gpg(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("gpg-suite")?;
        Ok(())
    }

    fn install_gradle(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["gradle", "gradle-completion"])?;
        Ok(())
    }

    fn install_graphic_card_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_graphic_card_laptop_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_groovy(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("groovy")?;
        Ok(())
    }

    fn install_handbrake(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("handbrake")?;
        Ok(())
    }

    fn install_inkscape(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("inkscape")?;
        Ok(())
    }

    fn install_insync(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_intellij(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("intellij-idea")?;
        Ok(())
    }

    fn install_jdk(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("openjdk")?;
        unix::symlink(
            self,
            &format!("{}/openjdk/libexec/openjdk.jdk", self.get_brew_prefix()?),
            "/Library/Java/JavaVirtualMachines/openjdk.jdk",
        )?;
        unix::set_java_home(self, ".zshrc", "$(/usr/libexec/java_home)")?;
        unix::set_java_home(self, ".bashrc", "$(/usr/libexec/java_home)")?;
        unix::add_to_path(self, ".zshrc", "$JAVA_HOME/bin")?;
        unix::add_to_path(self, ".bashrc", "$JAVA_HOME/bin")?;
        Ok(())
    }

    fn install_keepassxc(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("keepassxc")?;
        Ok(())
    }

    async fn install_kubectl(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_helm(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_latex(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_lutris(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_maven(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("maven")?;
        Ok(())
    }

    fn install_makemkv(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_microcode(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn install_minikube(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_mkvtoolnix(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_nextcloud_client(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("nextcloud")?;
        Ok(())
    }

    async fn install_nodejs(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("nvm")?;
        let brew_prefix = self.get_brew_prefix()?;
        let mut zshrc = OpenOptions::new()
            .append(true)
            .open(format!("{}/.zshrc", self.get_home_dir()))?;
        writeln!(zshrc, "export NVM_DIR=\"$HOME/.nvm\"")?;
        writeln!(zshrc, "[ -s \"{}/opt/nvm/nvm.sh\" ] && . \"{}/opt/nvm/nvm.sh\"  # This loads nvm", &brew_prefix, &brew_prefix)?;
        writeln!(zshrc, "[ -s \"{}/opt/nvm/etc/bash_completion.d/nvm\" ] && . \"{}/opt/nvm/etc/bash_completion.d/nvm\"  # This loads nvm bash_completion", &brew_prefix, &brew_prefix)?;

        let mut bashrc = OpenOptions::new()
            .append(true)
            .open(format!("{}/.bashrc", self.get_home_dir()))?;
        writeln!(bashrc, "export NVM_DIR=\"$HOME/.nvm\"")?;
        writeln!(bashrc, "[ -s \"{}/opt/nvm/nvm.sh\" ] && . \"{}/opt/nvm/nvm.sh\"  # This loads nvm", &brew_prefix, &brew_prefix)?;
        writeln!(bashrc, "[ -s \"{}/opt/nvm/etc/bash_completion.d/nvm\" ] && . \"{}/opt/nvm/etc/bash_completion.d/nvm\"  # This loads nvm bash_completion", &brew_prefix, &brew_prefix)?;

        self.execute("nvm install node --latest-npm", false)?;
        self.execute("npm install --global yarn", false)?;
        Ok(())
    }

    async fn install_nordvpn(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.app_store_install_application("1116599239")?;
        Ok(())
    }

    fn install_nvidia_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_nvidia_laptop_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_obs_studio(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("obs")?;
        Ok(())
    }

    fn install_onedrive(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("onedrive")?;
        Ok(())
    }

    fn install_origin(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_powertop(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_python(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("python")?;
        Ok(())
    }

    async fn install_rust(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("rustup")?;
        self.execute("rustup-init -y", true)?;
        Ok(())
    }

    fn install_slack(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.app_store_install_application("803453959")?;
        Ok(())
    }

    fn install_spotify(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("spotify")?;
        Ok(())
    }

    fn install_steam(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("steam")?;
        Ok(())
    }

    fn install_sweet_home_3d(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("sweet-home3d")?;
        Ok(())
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn std::error::Error>> {
        system::download_file(
            "https://raw.githubusercontent.com/Homebrew/install/master/install.sh",
            "brew-install",
        ).await?;
        unix::recursively_chmod("brew-install", &0o755, &0o755)?;
        self.execute("./brew-install", false)?;
        fs::remove_file("brew-install")?;

        self.install_application("mas")?;
        self.cask_install_application("scroll-reverser")?;
        Ok(())
    }

    fn install_telnet(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("telnet")?;
        Ok(())
    }

    async fn install_themes(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_tlp(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_tmux(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["tmux", "reattach-to-user-namespace"])?;
        unix::setup_tmux(self)?;
        let mut file = OpenOptions::new()
            .append(true)
            .open(format!("{}/.tmux.conf", self.get_home_dir()))?;
        writeln!(file, "bind -T copy-mode-vi y send-keys -X copy-pipe-and-cancel 'reattach-to-user-namespace pbcopy'")?;
        Ok(())
    }

    fn install_vim(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_vlc(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("vlc")?;
        Ok(())
    }

    fn install_vm_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn install_vscode(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cask_install_application("visual-studio-code")?;
        Ok(())
    }

    async fn install_wifi(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_window_manager(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_wget(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_wine(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_xcode(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.app_store_install_application("497799835")?;
        Ok(())
    }

    async fn install_zsh(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_applications(vec!["zsh", "zsh-autosuggestions"])?;
        unix::setup_zsh(self, Some(&format!("{}/bin/zsh", self.get_brew_prefix()?))).await?;
        Ok(())
    }

    fn set_development_shortcuts(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn set_development_environment_settings(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn setup_power_saving_tweaks(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update_os(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.update_os_repo()?;
        self.execute("brew upgrade", false)?;
        Ok(())
    }

    fn update_os_repo(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.execute("brew update", false)?;
        Ok(())
    }
}
