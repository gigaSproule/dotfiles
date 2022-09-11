use std::error::Error;
use std::fs;
use std::process::Command;

use async_trait::async_trait;
use registry::{Hive, Security};

use crate::config::Config;
use crate::system;
use crate::system::System;

pub(crate) struct Windows<'s> {
    config: &'s Config,
}

impl<'s> Windows<'s> {
    pub(crate) fn new(config: &'s Config) -> Self {
        Windows { config }
    }

    fn execute_powershell(
        &self,
        command: &str,
        _super_user: bool,
    ) -> Result<String, Box<dyn Error>> {
        let mut powershell = Command::new("powershell");
        let command = powershell.arg(command);
        system::run_command(command, true, self.config.dry_run)
    }

    fn execute_wsl(&self, command: &str, print_output: bool) -> Result<String, Box<dyn Error>> {
        let mut wsl = Command::new("wsl -d Ubuntu");
        let command = wsl.arg(command);
        system::run_command(command, print_output, self.config.dry_run)
    }

    fn install_wsl(&self, application: &str) -> Result<String, Box<dyn Error>> {
        self.execute_wsl(format!("-u root apt install {}", application).as_str(), true)
    }

    fn is_installed(&self, application: &str) -> Result<bool, Box<dyn Error>> {
        let mut choco = Command::new("choco");
        let choco_command = choco.args(vec!["list", "-e", application, "--local-only"]);
        let choco_output = system::run_command(choco_command, false, false)?;
        if !choco_output.contains("0 packages installed") {
            return Ok(true);
        }
        let mut import_module = Command::new("powershell");
        let import_module_command = import_module.args(vec!["Import-Module", "-Name", application]);
        let import_module_output = system::run_command(import_module_command, false, false)?;
        if !import_module_output.contains("was not loaded because no valid module file was found") {
            return Ok(true);
        }
        let regkey = Hive::LocalMachine.open(
            r"Software\Microsoft\Windows\CurrentVersion\Uninstall",
            Security::Read,
        )?;
        let mut found = false;
        for key in regkey.keys() {
            let opened = key.unwrap().open(Security::Read).unwrap();
            let display_name = opened.value("DisplayName");
            if display_name.is_err() {
                continue;
            }
            if display_name.unwrap().to_string().starts_with(application) {
                found = true;
                break;
            }
        }
        if found {
            return Ok(true);
        }
        let regkey = Hive::LocalMachine.open(
            r"Software\Wow6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
            Security::Read,
        )?;
        for key in regkey.keys() {
            let opened = key.unwrap().open(Security::Read).unwrap();
            let display_name = opened.value("DisplayName");
            if display_name.is_err() {
                continue;
            }
            if display_name.unwrap().to_string().starts_with(application) {
                found = true;
                break;
            }
        }
        if found {
            return Ok(true);
        }
        Ok(false)
    }

    fn is_installed_wsl(&self, application: &str) -> Result<bool, Box<dyn Error>> {
        let dpkg_output = self.execute_wsl(&format!("dpkg -l {}", application), false)?;
        if !dpkg_output.starts_with("dpkg-query: no packages found matching") {
            return Ok(true);
        }
        Ok(false)
    }
}

#[async_trait]
impl<'s> System for Windows<'s> {
    fn execute(&self, command: &str, _super_user: bool) -> Result<String, Box<dyn Error>> {
        let mut cmd = Command::new("cmd");
        let child = cmd.args(vec!["/c", command]);
        system::run_command(child, true, self.config.dry_run)
    }

    fn get_home_dir(&self) -> String {
        system::get_home_dir()
    }

    fn install_applications(&self, applications: Vec<&str>) -> Result<String, Box<dyn Error>> {
        self.execute(
            format!("choco install {}", applications.join(" ")).as_str(),
            true,
        )
    }

    fn install_android_studio(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("androidstudio")? {
            self.install_application("androidstudio")?;
        }
        Ok(())
    }

    fn install_archiver(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("7zip")? {
            self.install_application("7zip")?;
        }
        Ok(())
    }

    fn install_bash(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_blender(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("blender")? {
            self.install_application("blender")?;
        }
        Ok(())
    }

    fn install_bluetooth(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_calibre(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("calibre")? {
            self.install_application("calibre")?;
        }
        Ok(())
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn Error>> {
        system::setup_codecs(self).await
    }

    async fn install_cryptomator(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("cryptomator")? {
            self.install_application("cryptomator")?;
        }
        if !self.is_installed("Dokan")? {
            // Required for reading files on VFS mounts
            system::download_file(
                "https://github.com/dokan-dev/dokany/releases/download/v1.5.1.1000/DokanSetup.exe",
                "DokanSetup.exe",
            ).await?;
            self.execute_powershell(
                "Invoke-Expression -Command \".\\DokanSetup.exe /passive /norestart\"",
                true,
            )?;
            fs::remove_file("DokanSetup.exe")?;
            println!(
                "You will need to restart your machine for the kernel driver changes to take affect."
            );
        }
        Ok(())
    }

    fn install_curl(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("curl")? {
            self.install_wsl("curl")?;
        }
        if !self.config.wsl && !self.is_installed("curl")? {
            self.install_application("curl")?;
        }
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
            self.install_application("discord")?;
        }
        Ok(())
    }

    fn install_disk_usage_analyser(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("windirstat")? {
            self.install_application("windirstat")?;
        }
        Ok(())
    }

    fn install_development_extras(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_docker(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("docker-desktop")? {
            self.install_application("docker-desktop")?;
        }
        if !self.is_installed("DockerCompletion")? {
            self.execute_powershell("Install-Module -Name DockerCompletion -Force", true)?;
            self.execute_powershell("Import-Module -Name DockerCompletion", true)?;
            // TODO: Append if needed instead of blindly re-creating file
            fs::write(
                "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\profile.ps1",
                "Import-Module DockerCompletion\r\n".as_bytes(),
            )?;
        }
        Ok(())
    }

    fn install_dropbox(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("dropbox")? {
            self.install_application("dropbox")?;
        }
        Ok(())
    }

    async fn install_eclipse(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Should this just be removed?
        if !self.is_installed("eclipse")? {
            self.install_application("eclipse")?;
        }
        Ok(())
    }

    fn install_epic_games(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("epicgameslauncher")? {
            self.install_application("epicgameslauncher")?;
        }
        Ok(())
    }

    fn install_firefox(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("firefox")? {
            self.install_application("firefox")?;
        }
        Ok(())
    }

    fn install_firmware_updater(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_git(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("git")? {
            self.install_wsl("git")?;
        }
        if !self.config.wsl {
            if !self.is_installed("git")? {
                self.install_application("git")?;
            }
            system::setup_git_config(self)?;
            self.execute("git config --system core.longpaths true", false)?;
            if !self.is_installed("posh-git")? {
                self.execute_powershell("Install-Module -Name posh-git -Force", true)?;
                self.execute_powershell("Import-Module -Name posh-git", true)?;
                // TODO: Append if needed instead of blindly re-creating file
                fs::write(
                    "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\profile.ps1",
                    "Import-Module posh-git\r\n".as_bytes(),
                )?;
            }
        }
        Ok(())
    }

    fn install_gimp(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("gimp")? {
            self.install_application("gimp")?;
        }
        Ok(())
    }

    fn install_gog_galaxy(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("goggalaxy")? {
            self.install_application("goggalaxy")?;
        }
        Ok(())
    }

    async fn install_google_chrome(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("googlechrome")? {
            self.install_application("googlechrome")?;
        }
        Ok(())
    }

    fn install_google_cloud_sdk(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("gcloudsdk")? {
            self.install_wsl("gcloudsdk")?;
        }
        if !self.config.wsl && !self.is_installed("gcloudsdk")? {
            self.install_application("gcloudsdk")?;
        }
        Ok(())
    }

    fn install_google_drive(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("google-drive-file-stream")? {
            self.install_application("google-drive-file-stream")?;
        }
        Ok(())
    }

    fn install_gpg(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("gpg4win")? {
            self.install_application("gpg4win")?;
        }
        Ok(())
    }

    fn install_gradle(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("gradle")? {
            self.install_wsl("gradle")?;
        }
        // TODO: Alternative?
        if !self.config.wsl && !self.is_installed("gradle")? {
            self.install_application("gradle")?;
        }
        Ok(())
    }

    fn install_gramps(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("gramps")? {
            self.install_application("gramps")?;
        }
        Ok(())
    }

    fn install_graphic_card_tools(&self) -> Result<(), Box<dyn Error>> {
        self.install_nvidia_tools()?;
        Ok(())
    }

    fn install_graphic_card_laptop_tools(&self) -> Result<(), Box<dyn Error>> {
        self.install_nvidia_laptop_tools()?;
        Ok(())
    }

    fn install_groovy(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("groovy")? {
            self.install_wsl("groovy")?;
        }
        if !self.config.wsl && !self.is_installed("groovy")? {
            self.install_application("groovy")?;
        }
        Ok(())
    }

    fn install_handbrake(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("handbrake")? {
            self.install_application("handbrake")?;
        }
        Ok(())
    }

    fn install_inkscape(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("inkscape")? {
            self.install_application("inkscape")?;
        }
        Ok(())
    }

    fn install_insync(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_intellij(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("intellijidea-ultimate")? {
            self.install_application("intellijidea-ultimate")?;
        }
        Ok(())
    }

    fn install_jdk(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("openjdk")? {
            self.install_wsl("openjdk")?;
        }
        if !self.config.wsl && !self.is_installed("adoptopenjdk")? {
            self.install_application("adoptopenjdk")?;
        }
        Ok(())
    }

    fn install_keepassxc(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("keepassxc")? {
            self.install_application("keepassxc")?;
        }
        Ok(())
    }

    async fn install_kubectl(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("kubernetes-cli")? {
            self.install_application("kubernetes-cli")?;
        }
        Ok(())
    }

    async fn install_helm(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Alternative?
        if !self.is_installed("kubernetes-helm")? {
            self.install_application("kubernetes-helm")?;
        }
        Ok(())
    }

    fn install_latex(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Alternative?
        if !self.is_installed("texlive")? {
            self.install_application("texlive")?;
            self.execute(" C:\\texlive\\2021\\bin\\win32\\tlmgr.bat install latexmk enumitem titlesec latexindent", true)?;
        }
        Ok(())
    }

    fn install_libreoffice(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("libreoffice-fresh")? {
            self.install_application("libreoffice-fresh")?;
        }
        Ok(())
    }

    fn install_lutris(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_maven(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("maven")? {
            self.install_wsl("maven")?;
        }
        if !self.config.wsl && !self.is_installed("maven")? {
            self.install_application("maven")?;
        }
        Ok(())
    }

    fn install_makemkv(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("makemkv")? {
            self.install_application("makemkv")?;
        }
        Ok(())
    }

    fn install_microcode(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_microsoft_edge(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn install_minikube(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("minikube")? {
            self.install_application("minikube")?;
        }
        Ok(())
    }

    fn install_mkvtoolnix(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("mkvtoolnix")? {
            self.install_application("mkvtoolnix")?;
        }
        Ok(())
    }

    fn install_networking_tools(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("nmap")? {
            self.install_application("nmap")?;
        }
        self.execute(
            "dism /online /Enable-Feature /FeatureName:TelnetClient",
            true,
        )?;
        Ok(())
    }

    fn install_nextcloud_client(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("nextcloud-client")? {
            self.install_application("nextcloud-client")?;
        }
        Ok(())
    }

    async fn install_nodejs(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("nvm")? {
            self.install_wsl("nvm")?;
        }
        if !self.config.wsl {
            if !self.is_installed("nvm")? {
                self.install_application("nvm")?;
            }

            let nvm_script = "function callnvm() {{\n\
          # Always use argument version if there is one\n\
          $versionDesired = $args[0]\n\
          if (($versionDesired -eq \"\" -Or $versionDesired -eq $null) -And (Test-Path .nvmrc -PathType Any)) {{\n\
               # if we have an nvmrc and no argument supplied, use the version in the file\n\
               $versionDesired = $(Get-Content .nvmrc).replace( 'v', '' );\n\
          }}\n\
               Write-Host \"Requesting version '$($versionDesired)'\"\n\
          if ($versionDesired -eq \"\") {{\n\
              Write-Host \"A node version needs specifying as an argument if there is no .nvmrc\"\n\
          }} else {{\n\
              $response = nvm use $versionDesired;\n\
              if ($response -match \"is not installed\") {{\n\
                  if ($response -match \"64-bit\") {{\n\
                    $response = nvm install $versionDesired x64\n\
                  }} else {{\n\
                    $response = nvm install $versionDesired x86\n\
                  }}\n\
                  Write-Host $response\n\
                  $response = nvm use $versionDesired;\n\
              }}\n\
              Write-Host $response\n\
          }}\n\
       }}\n\
       Set-Alias nvmu -value \"callnvm\"";
            system::add_to_file(
                &format!(
                    r"{}\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1",
                    self.get_home_dir()
                ),
                nvm_script,
            )?;
            self.execute_powershell("refreshenv", false)?;
            self.execute("nvm install latest", false)?;
            let stdout = &self.execute("nvm list", false);
            let output = stdout
                .as_ref()
                .expect("Could not find any installed npm versions");
            for output_version in output.split("\n") {
                if output_version != "" {
                    self.execute(
                        format!("nvm use {}", output_version.replace(" ", "")).as_str(),
                        false,
                    )?;
                    break;
                }
            }
            self.execute_powershell("refreshenv", false)?;
            self.execute_powershell("npm install --global yarn", true)?;
        }
        Ok(())
    }

    async fn install_nordvpn(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("nordvpn")? {
            self.install_application("nordvpn")?;
        }
        Ok(())
    }

    fn install_nvidia_tools(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("geforce-experience")? {
            self.install_application("geforce-experience")?;
        }
        Ok(())
    }

    fn install_nvidia_laptop_tools(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_obs_studio(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("obs-studio")? {
            self.install_application("obs-studio")?;
        }
        Ok(())
    }

    fn install_onedrive(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_origin(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("origin")? {
            self.install_application("origin")?;
        }
        Ok(())
    }

    fn install_powertop(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_python(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("python")? {
            self.install_wsl("python")?;
        }
        if !self.config.wsl && !self.is_installed("python")? {
            self.install_application("python")?;
        }
        Ok(())
    }

    fn install_retroarch(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("retroarch")? {
            self.install_application("retroarch")?;
        }
        Ok(())
    }

    async fn install_rust(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("rustup")? {
            self.install_wsl("rustup")?;
        }
        // Always install outside of WSL in case this needs updating for Windows
        if !self.is_installed("rustup.install")? {
            self.install_application("rustup.install")?;
        }
        // Required for compilation
        if !self.is_installed("visualstudio2022buildtools")? {
            self.install_application("visualstudio2022buildtools")?;
        }
        Ok(())
    }

    fn install_slack(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("slack")? {
            self.install_application("slack")?;
        }
        Ok(())
    }

    fn install_spotify(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("spotify")? {
            self.install_application("spotify")?;
        }
        Ok(())
    }

    fn install_steam(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("steam-client")? {
            self.install_application("steam-client")?;
        }
        Ok(())
    }

    fn install_strawberry_music_player(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("strawberrymusicplayer")? {
            self.install_application("strawberrymusicplayer")?;
        }
        Ok(())
    }

    fn install_sweet_home_3d(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("sweet-home-3d")? {
            self.install_application("sweet-home-3d")?;
        }
        Ok(())
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn Error>> {
        // Needed to install powershell modules
        self.execute_powershell("Set-ExecutionPolicy Unrestricted", true)?;
        system::download_file("https://chocolatey.org/install.ps1", "install.ps1").await?;
        self.execute_powershell("iex .\\install.ps1", true)?;
        self.execute_powershell(
            "Install-PackageProvider -Name NuGet -MinimumVersion 2.8.5.201 -Force",
            true,
        )?;
        self.execute_powershell(
            "Import-Module \"$env:ProgramData\\chocolatey\\helpers\\chocolateyInstaller.psm1\"",
            true,
        )?;
        self.execute_powershell("refreshenv", true)?;
        self.execute("REG ADD HKLM\\SYSTEM\\CurrentControlSet\\Control\\FileSystem /v LongPathsEnabled /t REG_DWORD /d 1 /f", true)?;
        if !self.is_installed("powershell-core")? {
            self.install_application("powershell-core")?;
        }
        if !self.is_installed("microsoft-windows-terminal")? {
            self.install_application("microsoft-windows-terminal")?;
        }
        if self.config.development && self.config.wsl {
            self.execute_powershell("wsl --install -d Ubuntu", true)?;
            // TODO: Download Linux binary, copy into Ubuntu WSL and run with development only flag
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
        Ok(())
    }

    fn install_vim(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("vim")? {
            self.install_application("vim")?;
        }
        Ok(())
    }

    fn install_vlc(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("vlc")? {
            self.install_application("vlc")?;
        }
        Ok(())
    }

    fn install_vm_tools(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_vscode(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("vscode")? {
            self.install_wsl("vscode")?;
        }
        // Always install outside of WSL in case this needs updating for Windows
        if !self.is_installed("vscode")? {
            self.install_application("vscode")?;
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
        if self.config.wsl && !self.is_installed_wsl("wget")? {
            self.install_wsl("wget")?;
        }
        if !self.config.wsl && !self.is_installed("wget")? {
            self.install_application("wget")?;
        }
        Ok(())
    }

    fn install_wine(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_xcode(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn install_zsh(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn set_development_shortcuts(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn set_development_environment_settings(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn setup_power_saving_tweaks(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn update_os(&self) -> Result<(), Box<dyn Error>> {
        self.execute("choco upgrade all", true)?;
        Ok(())
    }

    fn update_os_repo(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
