use async_trait::async_trait;
use log::info;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use crate::config::Config;
use crate::system::{self, System};
use crate::unix;

#[derive(Debug)]
pub(crate) struct Mac<'s> {
    config: &'s Config,
}

impl<'s> Mac<'s> {
    pub(crate) fn new(config: &'s Config) -> Self {
        Mac { config }
    }

    fn app_store_install_application(
        &self,
        application_id: &str,
    ) -> Result<String, Box<dyn Error>> {
        self.execute(&format!("mas install {}", application_id), false)
    }

    fn cask_install_application(&self, application: &str) -> Result<String, Box<dyn Error>> {
        self.execute(&format!("brew install --cask {}", application), false)
    }

    fn get_brew_prefix(&self) -> Result<String, Box<dyn Error>> {
        self.execute("brew --prefix", false)
    }

    fn is_installed(&self, app: &str) -> Result<bool, Box<dyn Error>> {
        let mut command = std::process::Command::new("osascript");
        let arged = command.args(vec!["-e", &format!("id of application \"{}\"", app)]);
        let osascript_output = system::run_command(arged, false, false)?;
        if !osascript_output.contains("execution error") {
            return Ok(true);
        }
        let brew_output = unix::execute(&format!("brew list {}", app), false, false, false)?;
        if !brew_output.is_empty() && !brew_output.starts_with("Error: No ") {
            return Ok(true);
        }
        let which_output = unix::execute(&format!("which {}", app), false, false, false)?;
        if !which_output.is_empty() {
            return Ok(true);
        }
        let mas_output = unix::execute(&format!("mas info {}", app), false, false, false)?;
        if !mas_output.starts_with("No results found") {
            return Ok(true);
        }
        if let Ok(entry) = fs::read_dir("/Applications") {
            if entry.into_iter().any(|f| {
                f.unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_lowercase()
                    .starts_with(&app.to_lowercase())
            }) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

#[async_trait]
impl<'s> System for Mac<'s> {
    fn execute(&self, command: &str, super_user: bool) -> Result<String, Box<dyn Error>> {
        unix::execute(command, super_user, true, self.config.dry_run)
    }

    fn get_home_dir(&self) -> String {
        system::get_home_dir()
    }

    fn install_applications(&self, applications: Vec<&str>) -> Result<String, Box<dyn Error>> {
        self.execute(&format!("brew install {}", applications.join(" ")), false)
    }

    fn install_affinity_suite(&self) -> Result<(), Box<dyn Error>> {
        // Affinity Photo 2
        if !self.is_installed("affinity-photo")? {
            self.cask_install_application("affinity-photo")?;
        }
        // Affinity Publisher 2
        if !self.is_installed("affinity-publisher")? {
            self.cask_install_application("affinity-publisher")?;
        }
        // Affinity Designer 2
        if !self.is_installed("affinity-designer")? {
            self.cask_install_application("affinity-designer")?;
        }
        Ok(())
    }

    fn install_android_studio(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("android-studio")? {
            self.cask_install_application("android-studio")?;
        }
        Ok(())
    }

    fn install_archiver(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("the-unarchiver")? {
            self.cask_install_application("the-unarchiver")?;
        }
        Ok(())
    }

    fn install_audacity(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("audacity")? {
            self.cask_install_application("audacity")?;
        }

        if !self.is_installed("ffmpeg")? {
            self.install_application("ffmpeg")?;
        }
        Ok(())
    }

    fn install_authy(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("authy")? {
            self.cask_install_application("authy")?;
        }
        Ok(())
    }

    fn install_bambu_studio(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("bambu-studio")? {
            self.cask_install_application("bambu-studio")?;
        }
        Ok(())
    }

    fn install_bash(&self) -> Result<(), Box<dyn Error>> {
        unix::setup_bash(self)?;
        let bashrc = format!("{}/.bashrc", self.get_home_dir());
        let mut bashrc_file = OpenOptions::new().append(true).open(bashrc)?;
        writeln!(
            bashrc_file,
            "eval \"$({}/bin/brew shellenv)\"",
            self.get_brew_prefix()?
        )?;
        Ok(())
    }

    fn install_blender(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("blender")? {
            self.cask_install_application("blender")?;
        }
        Ok(())
    }

    fn install_bluetooth(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_calibre(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("calibre")? {
            self.cask_install_application("calibre")?;
        }
        Ok(())
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn Error>> {
        system::setup_codecs(self).await?;
        system::download_file(
            "https://vlc-bluray.whoknowsmy.name/files/mac/libaacs.dylib",
            "/usr/local/lib/libaacs.dylib".to_string().as_str(),
        )
        .await?;
        let user_id = unix::get_user_id();
        let group_id = unix::get_group_id();
        unix::recursively_chown(
            &format!("{}/.config", self.get_home_dir()),
            &user_id,
            &group_id,
        )?;
        Ok(())
    }

    fn install_cplusplus(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("gcc")? {
            self.install_application("gcc")?;
        }
        if !self.is_installed("make")? {
            self.install_application("make")?;
        }
        if !self.is_installed("cmake")? {
            self.install_application("cmake")?;
        }
        Ok(())
    }

    async fn install_cryptomator(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("cryptomator")? {
            self.cask_install_application("cryptomator")?;
        }
        Ok(())
    }

    fn install_curl(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_davinci_resolve(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("DaVinci Resolve")? {
            open::that("https://www.blackmagicdesign.com/uk/products/davinciresolve/studio")?;
        }
        Ok(())
    }

    fn install_discord(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("discord")? {
            self.cask_install_application("discord")?;
        }
        Ok(())
    }

    fn install_disk_usage_analyser(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_development_extras(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_docker(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("docker")? {
            self.cask_install_application("docker")?;
        }
        Ok(())
    }

    async fn install_eclipse(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("eclipse")? {
            self.cask_install_application("eclipse-java")?;
        }
        Ok(())
    }

    async fn install_epic_games(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_exact_audio_copy(&self) -> Result<(), Box<dyn Error>> {
        // Ensure Wine is installed
        // Ensure dotnet20 and dotnet40 in Wine
        // Install EAC into Wine
        Ok(())
    }

    async fn install_exercism(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("exercism")? {
            self.install_application("exercism")?;
        }
        Ok(())
    }

    fn install_firefox(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("firefox")? {
            self.cask_install_application("firefox")?;
        }
        Ok(())
    }

    fn install_firmware_updater(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_git(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("git")? {
            self.install_application("git")?;
        }
        system::setup_git_config(self)?;
        Ok(())
    }

    fn install_gimp(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("gimp")? {
            self.cask_install_application("gimp")?;
        }
        Ok(())
    }

    async fn install_godot(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("godot-mono")? {
            self.cask_install_application("godot-mono")?;
        }
        Ok(())
    }

    async fn install_gog_galaxy(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn install_google_chrome(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Google Chrome")? {
            self.cask_install_application("google-chrome")?;
        }
        Ok(())
    }

    fn install_google_cloud_sdk(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn install_google_drive(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Google Drive")? {
            self.cask_install_application("google-drive")?;
        }
        Ok(())
    }

    fn install_gpg(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("GPG Keychain")? {
            self.cask_install_application("gpg-suite")?;
        }
        Ok(())
    }

    fn install_gradle(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("gradle")? {
            self.install_applications(vec!["gradle", "gradle-completion"])?;
        }
        Ok(())
    }

    fn install_gramps(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("gramps")? {
            self.install_application("gramps")?;
        }
        Ok(())
    }

    fn install_graphic_card_laptop_tools(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_groovy(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("groovy")? {
            self.install_application("groovy")?;
        }
        Ok(())
    }

    fn install_handbrake(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("handbrake")? {
            self.cask_install_application("handbrake")?;
        }
        Ok(())
    }

    fn install_inkscape(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("inkscape")? {
            self.cask_install_application("inkscape")?;
        }
        Ok(())
    }

    fn install_insync(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_intel_gpu_laptop_tools(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_intellij(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("IntelliJ IDEA")? {
            self.cask_install_application("intellij-idea")?;
        }
        Ok(())
    }

    fn install_jdk(&self) -> Result<(), Box<dyn Error>> {
        if !Path::new(&format!(
            "{}/opt/openjdk/libexec/openjdk.jdk",
            self.get_brew_prefix()?
        ))
        .exists()
        {
            self.install_application("openjdk")?;
            unix::symlink(
                self,
                &format!(
                    "{}/opt/openjdk/libexec/openjdk.jdk",
                    self.get_brew_prefix()?
                ),
                "/Library/Java/JavaVirtualMachines/openjdk.jdk",
            )?;
        }
        unix::set_java_home(self, ".zshrc", "$(/usr/libexec/java_home)")?;
        unix::set_java_home(self, ".bashrc", "$(/usr/libexec/java_home)")?;
        unix::add_to_path(self, ".zshrc", "$JAVA_HOME/bin")?;
        unix::add_to_path(self, ".bashrc", "$JAVA_HOME/bin")?;
        Ok(())
    }

    fn install_keepassxc(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("keepassxc")? {
            self.cask_install_application("keepassxc")?;
        }
        Ok(())
    }

    async fn install_kubectl(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    async fn install_helm(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn install_latex(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("texlive")? {
            self.install_application("texlive")?;
        }
        Ok(())
    }

    fn install_office(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("libreoffice")? {
            self.cask_install_application("libreoffice")?;
        }
        Ok(())
    }

    fn install_openscad(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("openscad@snapshot")? {
            self.cask_install_application("openscad@snapshot");
        }
        Ok(())
    }

    fn install_lutris(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_maven(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("mvn")? {
            self.install_application("maven")?;
        }
        Ok(())
    }

    fn install_makemkv(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn install_microcode(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_microsoft_edge(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Microsoft Edge")? {
            self.cask_install_application("microsoft-edge")?;
        }
        Ok(())
    }

    async fn install_minikube(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn install_mkvtoolnix(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn install_networking_tools(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("inetutils")? {
            self.install_application("inetutils")?;
        }
        if !self.is_installed("nmap")? {
            self.install_application("nmap")?;
        }
        Ok(())
    }

    fn install_nextcloud_client(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("nextcloud")? {
            self.cask_install_application("nextcloud")?;
        }
        Ok(())
    }

    async fn install_nodejs(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("nvm")? {
            self.install_application("nvm")?;
        }
        let brew_prefix = self.get_brew_prefix()?;
        let content = format!("export NVM_DIR=\"$HOME/.nvm\"\n\
        [ -s \"{}/opt/nvm/nvm.sh\" ] && . \"{}/opt/nvm/nvm.sh\"  # This loads nvm\n\
        [ -s \"{}/opt/nvm/etc/bash_completion.d/nvm\" ] && . \"{}/opt/nvm/etc/bash_completion.d/nvm\"  # This loads nvm bash_completion", &brew_prefix, &brew_prefix, &brew_prefix, &brew_prefix);
        system::add_to_file(&format!("{}/.zshrc", self.get_home_dir()), &content)?;
        system::add_to_file(&format!("{}/.bashrc", self.get_home_dir()), &content)?;
        let zsh_nvm_dir = "autoload -U add-zsh-hook\n\
        load-nvmrc() {\n\
            local node_version=\"$(nvm version)\"\n\
            local nvmrc_path=\"$(nvm_find_nvmrc)\"\n\
            if [ -n \"$nvmrc_path\" ]; then\n\
                local nvmrc_node_version=$(nvm version \"$(cat \"${nvmrc_path}\")\")\n\
                if [ \"$nvmrc_node_version\" = \"N/A\" ]; then\n\
                    nvm install\n\
                elif [ \"$nvmrc_node_version\" != \"$node_version\" ]; then\n\
                    nvm use\n\
                fi\n\
            elif [ \"$node_version\" != \"$(nvm version default)\" ]; then\n\
                echo \"Reverting to nvm default version\"\n\
                nvm use default\n\
            fi\n\
        }\n\
        add-zsh-hook chpwd load-nvmrc\n\
        load-nvmrc";
        system::add_to_file(&format!("{}/.zshrc", self.get_home_dir()), zsh_nvm_dir)?;
        let bash_nvm_dir = "cdnvm() {\n\
            command cd \"$@\";\n\
            nvm_path=$(nvm_find_up .nvmrc | tr -d '\n')\n\
            # If there are no .nvmrc file, use the default nvm version\n\
            if [[ ! $nvm_path = *[^[:space:]]* ]]; then\n\
                declare default_version;\n\
                default_version=$(nvm version default);\n\
                # If there is no default version, set it to `node`\n\
                # This will use the latest version on your machine\n\
                if [[ $default_version == \"N/A\" ]]; then\n\
                    nvm alias default node;\n\
                    default_version=$(nvm version default);\n\
                fi\n\
                # If the current version is not the default version, set it to use the default version\n\
                if [[ $(nvm current) != \"$default_version\" ]]; then\n\
                    nvm use default;\n\
                fi\n\
            elif [[ -s $nvm_path/.nvmrc && -r $nvm_path/.nvmrc ]]; then\n\
                declare nvm_version\n\
                nvm_version=$(<\"$nvm_path\"/.nvmrc)\n\
                declare locally_resolved_nvm_version\n\
                # `nvm ls` will check all locally-available versions\n\
                # If there are multiple matching versions, take the latest one\n\
                # Remove the `->` and `*` characters and spaces\n\
                # `locally_resolved_nvm_version` will be `N/A` if no local versions are found\n\
                locally_resolved_nvm_version=$(nvm ls --no-colors \"$nvm_version\" | tail -1 | tr -d '\\->*' | tr -d '[:space:]')\n\
                # If it is not already installed, install it\n\
                # `nvm install` will implicitly use the newly-installed version\n\
                if [[ \"$locally_resolved_nvm_version\" == \"N/A\" ]]; then\n\
                    nvm install \"$nvm_version\";\n\
                elif [[ $(nvm current) != \"$locally_resolved_nvm_version\" ]]; then\n\
                    nvm use \"$nvm_version\";\n\
                fi\n\
            fi\n\
        }\n\
        alias cd='cdnvm'\n\
        cd \"$PWD\"";
        system::add_to_file(&format!("{}/.bashrc", self.get_home_dir()), bash_nvm_dir)?;

        self.execute("nvm install node --latest-npm", false)?;
        self.execute("npm install --global yarn", false)?;
        Ok(())
    }

    async fn install_nordvpn(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("905953485")? {
            self.app_store_install_application("905953485")?;
        }
        Ok(())
    }

    fn install_nvidia_tools(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_nvidia_laptop_tools(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_obs_studio(&self) -> Result<(), Box<dyn Error>> {
        self.cask_install_application("obs")?;
        Ok(())
    }

    fn install_onedrive(&self) -> Result<(), Box<dyn Error>> {
        self.cask_install_application("onedrive")?;
        Ok(())
    }

    fn install_origin(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_powertop(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_printer_drivers(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_python(&self) -> Result<(), Box<dyn Error>> {
        if !Path::new(&format!(
            "{}/opt/python/libexec/bin",
            self.get_brew_prefix()?
        ))
        .exists()
        {
            self.install_application("python")?;
        }
        let content = format!(
            "export PATH=\"$PATH:{}/opt/python/libexec/bin\"",
            self.get_brew_prefix()?
        );
        system::add_to_file(&format!("{}/.zshrc", self.get_home_dir()), &content)?;
        system::add_to_file(&format!("{}/.bashrc", self.get_home_dir()), &content)?;
        Ok(())
    }

    fn install_quicklook(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_retroarch(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn install_rust(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("rustup")? {
            self.install_application("rustup")?;
            self.execute("rustup-init -y", true)?;
        }
        let content = "source $HOME/.cargo/env";
        system::add_to_file(&format!("{}/.zshrc", self.get_home_dir()), content)?;
        system::add_to_file(&format!("{}/.bashrc", self.get_home_dir()), content)?;

        Ok(())
    }

    fn install_rust_rover(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("rustrover")? {
            self.cask_install_application("rustrover")?;
        }
        Ok(())
    }

    fn install_slack(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("803453959")? {
            self.app_store_install_application("803453959")?;
        }
        Ok(())
    }

    fn install_spotify(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("spotify")? {
            self.cask_install_application("spotify")?;
        }
        Ok(())
    }

    fn install_steam(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("steam")? {
            self.cask_install_application("steam")?;
        }
        Ok(())
    }

    fn install_sweet_home_3d(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Sweet Home 3D")? {
            self.cask_install_application("sweet-home3d")?;
        }
        Ok(())
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("brew")? {
            system::download_file(
                "https://raw.githubusercontent.com/Homebrew/install/master/install.sh",
                "brew-install",
            )
            .await?;
            unix::recursively_chmod("brew-install", &0o755, &0o755)?;
            self.execute("NONINTERACTIVE=1 ./brew-install", false)?;
            fs::remove_file("brew-install")?;
        }

        let zshrc = format!("{}/.zshrc", self.get_home_dir());
        system::add_to_file(
            &zshrc,
            &format!("eval \"$({}/bin/brew shellenv)\"", self.get_brew_prefix()?),
        )?;

        let bashrc = format!("{}/.bashrc", self.get_home_dir());
        system::add_to_file(
            &bashrc,
            &format!("eval \"$({}/bin/brew shellenv)\"", self.get_brew_prefix()?),
        )?;

        if !self.is_installed("mas")? {
            self.install_application("mas")?;
        }
        if !self.is_installed("Scroll Reverser")? {
            self.cask_install_application("scroll-reverser")?;
        }
        if cfg!(target_arch = "aarch64") {
            self.execute("softwareupdate --install-rosetta --agree-to-license", true)?;
        }
        Ok(())
    }

    // TODO: This hasn't been tested on a Mac, so this may not extract properly
    async fn install_tauon_music_box(&self) -> Result<(), Box<dyn Error>> {
        let version = "8.2.2";
        system::download_file(
            format!(
                "https://github.com/Taiko2k/Tauon/releases/download/v{0}/TauonMusicBox.dmg",
                &version
            )
            .as_str(),
            "tauon-music-box.dmg",
        )
        .await?;
        self.execute("hdiutil attach tauon-music-box.dmg", true)?;
        fs::copy(
            format!("/Volumes/TauonMusicBox {}/TauonMusicBox.app", &version),
            "/Applications",
        )?;
        self.execute(
            format!("hdiutil detach /Volumes/TauonMusicBox {}", &version).as_str(),
            true,
        )?;
        fs::remove_file("tauon-music-box.dmg")?;
        Ok(())
    }

    fn install_terraform(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("terraform")? {
            self.install_application("terraform")?;
        }
        Ok(())
    }

    async fn install_themes(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_tlp(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_tmux(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("tmux")? {
            self.install_application("tmux")?;
        }
        if !self.is_installed("reattach-to-user-namespace")? {
            self.install_application("reattach-to-user-namespace")?;
        }
        unix::setup_tmux(self)?;
        system::add_to_file(&format!("{}/.tmux.conf", self.get_home_dir()), "bind -T copy-mode-vi y send-keys -X copy-pipe-and-cancel 'reattach-to-user-namespace pbcopy'")?;
        Ok(())
    }

    fn install_vim(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn install_vlc(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("vlc")? {
            self.cask_install_application("vlc")?;
        }
        Ok(())
    }

    fn install_vm_tools(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn install_vscode(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Visual Studio Code")? {
            self.cask_install_application("visual-studio-code")?;
        }
        Ok(())
    }

    async fn install_wifi(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_window_manager(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_wget(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_whatsapp(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_whipper(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_wine(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn install_xbox_streaming(&self) -> Result<(), Box<dyn Error>> {
        // if !self.is_installed("9MV0B5HZVK9Z")? {
        let version = "2.3.3";
        system::download_file(
            format!("https://github.com/unknownskl/greenlight/releases/download/v{0}/Greenlight-{0}-universal.dmg", &version).as_str(),
            "greenlight.dmg",
        ).await?;
        self.execute("hdiutil attach greenlight.dmg", true)?;
        fs::copy(
            format!("/Volumes/Greenlight {}-universal/Greenlight.app", &version),
            "/Applications",
        )?;
        self.execute(
            format!("hdiutil detach /Volumes/Greenlight {}-universal", &version).as_str(),
            true,
        )?;
        fs::remove_file("greenlight.dmg")?;
        // }
        Ok(())
    }

    fn install_xcode(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("497799835")? {
            self.app_store_install_application("497799835")?;
        }
        Ok(())
    }

    async fn install_zsh(&self) -> Result<(), Box<dyn Error>> {
        if !Path::new(&format!("{}/bin/zsh", self.get_brew_prefix()?)).exists() {
            self.install_applications(vec!["zsh", "zsh-autosuggestions"])?;
        }
        unix::setup_zsh(self, Some(&format!("{}/bin/zsh", self.get_brew_prefix()?))).await?;

        let zshrc = format!("{}/.zshrc", self.get_home_dir());
        system::add_to_file(
            &zshrc,
            &format!("eval \"$({}/bin/brew shellenv)\"", self.get_brew_prefix()?),
        )?;

        Ok(())
    }

    fn set_development_shortcuts(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn set_development_environment_settings(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn setup_nas(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn setup_power_saving_tweaks(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn setup_user_bin(&self) -> Result<(), Box<dyn Error>> {
        unix::setup_user_bin(self)?;
        Ok(())
    }

    fn update_os(&self) -> Result<(), Box<dyn Error>> {
        self.update_os_repo()?;
        self.execute("brew upgrade", false)?;
        Ok(())
    }

    fn update_os_repo(&self) -> Result<(), Box<dyn Error>> {
        self.execute("brew update", false)?;
        Ok(())
    }
}
