use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

use async_trait::async_trait;

use crate::config::Config;
use crate::system;
use crate::system::System;

pub(crate) struct Windows<'a> {
    config: &'a Config,
}

impl<'a> Windows<'a> {
    fn execute_powershell(
        &self,
        command: &str,
        _super_user: bool,
    ) -> Result<String, Box<dyn Error>> {
        let mut powershell = Command::new("powershell");
        let command = powershell.arg(command);
        system::run_command(command, self.config.dry_run)
    }

    fn execute_wsl(&self, command: &str) -> Result<String, Box<dyn Error>> {
        let mut wsl = Command::new("wsl");
        let command = wsl.arg(command);
        system::run_command(command, self.config.dry_run)
    }

    fn install_wsl(&self, application: &str) -> Result<String, Box<dyn Error>> {
        self.execute_wsl(format!("sudo apt install {}", application).as_str())
    }
}

#[async_trait]
impl<'a> System<'a> for Windows<'a> {
    fn new(config: &'a Config) -> Self {
        if !is_elevated::is_elevated() {
            panic!("Need to run this with administrator privileges.")
        }
        Windows { config }
    }

    fn execute(&self, command: &str, _super_user: bool) -> Result<String, Box<dyn Error>> {
        let mut cmd = Command::new("cmd");
        let child = cmd.arg(command);
        system::run_command(child, self.config.dry_run)
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
        self.install_application("androidstudio")?;
        Ok(())
    }

    fn install_bash(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_blender(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("blender")?;
        Ok(())
    }

    fn install_bluetooth(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn Error>> {
        system::setup_codecs(self).await
    }

    fn install_conemu(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("conemu")?;
        Ok(())
    }

    fn install_cryptomator(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("cryptomator")?;
        Ok(())
    }

    fn install_curl(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("curl")?;
        Ok(())
    }

    fn install_davinci_resolve(&self) -> Result<(), Box<dyn Error>> {
        open::that("https://www.blackmagicdesign.com/uk/products/davinciresolve/studio")?;
        Ok(())
    }

    fn install_discord(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("discord")?;
        Ok(())
    }

    fn install_docker(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("docker-desktop")?;
        self.execute_powershell("Install-Module -Name DockerCompletion -Force", true)?;
        self.execute_powershell("Import-Module DockerCompletion", true)?;
        fs::write(
            "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\profile.ps1",
            "Import-Module DockerCompletion\r\n".as_bytes(),
        )?;
        Ok(())
    }

    fn install_dropbox(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("dropbox")?;
        Ok(())
    }

    async fn install_eclipse(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("eclipse")?;
        Ok(())
    }

    fn install_epic_games(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("epicgameslauncher")?;
        Ok(())
    }

    fn install_firefox(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("firefox")?;
        Ok(())
    }

    fn install_firmware_updater(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_gog_galaxy(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("goggalaxy")?;
        Ok(())
    }

    async fn install_google_chrome(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("googlechrome")?;
        Ok(())
    }

    fn install_google_cloud_sdk(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("gcloudsdk")?;
        Ok(())
    }

    fn install_google_drive(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("google-drive-file-stream")?;
        Ok(())
    }

    fn install_git(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("git")?;
        system::setup_git_config(self)?;
        self.execute("git config --system core.longpaths true", false)?;
        self.install_application("poshgit")?;
        self.install_wsl("git")?;
        Ok(())
    }

    fn install_gimp(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("gimp")?;
        Ok(())
    }

    fn install_gpg(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("gpg4win")?;
        Ok(())
    }

    fn install_gradle(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("gradle")?;
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
        self.install_application("groovy")?;
        Ok(())
    }

    fn install_handbrake(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("handbrake")?;
        Ok(())
    }

    fn install_inkscape(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("inkscape")?;
        Ok(())
    }

    fn install_insync(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_intellij(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("intellijidea-ultimate")?;
        Ok(())
    }

    fn install_jdk(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("adoptopenjdk")?;
        Ok(())
    }

    fn install_keepassxc(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("keepassxc")?;
        Ok(())
    }

    async fn install_kubectl(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("kubernetes-cli")?;
        Ok(())
    }

    fn install_helm(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("kubernetes-helm")?;
        Ok(())
    }

    fn install_latex(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("texlive")?;
        self.execute(" C:\\texlive\\2021\\bin\\win32\\tlmgr.bat install latexmk enumitem titlesec latexindent", true)?;
        Ok(())
    }

    fn install_lutris(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_maven(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("maven")?;
        Ok(())
    }

    fn install_makemkv(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("makemkv")?;
        Ok(())
    }

    fn install_microcode(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_microsoft_edge(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn install_minikube(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("minikube")?;
        Ok(())
    }

    fn install_mkvtoolnix(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("mkvtoolnix")?;
        Ok(())
    }

    fn install_networking_tools(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("nmap")?;
        self.execute(
            "dism /online /Enable-Feature /FeatureName:TelnetClient",
            true,
        )?;
        Ok(())
    }

    fn install_nextcloud_client(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("nextcloud-client")?;
        Ok(())
    }

    async fn install_nodejs(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("nvm")?;
        let mut file = OpenOptions::new().create(true).append(true).open(format!(
            "{}\\Documents\\WindowsPowerShell\\Microsoft.PowerShell_profile.ps1",
            self.get_home_dir()
        ))?;

        writeln!(file, "function callnvm() {{")?;
        writeln!(file, "   # Always use argument version if there is one")?;
        writeln!(file, "   $versionDesired = $args[0]")?;
        writeln!(file, "   if (($versionDesired -eq \"\" -Or $versionDesired -eq $null) -And (Test-Path .nvmrc -PathType Any)) {{")?;
        writeln!(
            file,
            "       # if we have an nvmrc and no argument supplied, use the version in the file"
        )?;
        writeln!(
            file,
            "       $versionDesired = $(Get-Content .nvmrc).replace( 'v', '' );"
        )?;
        writeln!(file, "   }}")?;
        writeln!(
            file,
            "   Write-Host \"Requesting version '$($versionDesired)'\""
        )?;
        writeln!(file, "   if ($versionDesired -eq \"\") {{")?;
        writeln!(file, "       Write-Host \"A node version needs specifying as an argument if there is no .nvmrc\"")?;
        writeln!(file, "   }} else {{")?;
        writeln!(file, "       $response = nvm use $versionDesired;")?;
        writeln!(file, "       if ($response -match \"is not installed\") {{")?;
        writeln!(file, "           if ($response -match \"64-bit\") {{")?;
        writeln!(
            file,
            "               $response = nvm install $versionDesired x64"
        )?;
        writeln!(file, "           }} else {{")?;
        writeln!(
            file,
            "               $response = nvm install $versionDesired x86"
        )?;
        writeln!(file, "           }}")?;
        writeln!(file, "           Write-Host $response")?;
        writeln!(file, "           $response = nvm use $versionDesired;")?;
        writeln!(file, "       }}")?;
        writeln!(file, "       Write-Host $response")?;
        writeln!(file, "   }}")?;
        writeln!(file, "}}")?;
        writeln!(file, "Set-Alias nvmu -value \"callnvm\"")?;
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
        Ok(())
    }

    async fn install_nordvpn(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("nordvpn")?;
        Ok(())
    }

    fn install_nvidia_tools(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("geforce-experience")?;
        Ok(())
    }

    fn install_nvidia_laptop_tools(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_obs_studio(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("obs-studio")?;
        Ok(())
    }

    fn install_onedrive(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_origin(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("origin")?;
        Ok(())
    }

    fn install_powertop(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_python(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("python")?;
        Ok(())
    }

    async fn install_rust(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("rustup.install")?;
        self.install_application("visualstudio2022buildtools")?;
        self.install_application("windows-sdk-10.1")?;
        Ok(())
    }

    fn install_slack(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("slack")?;
        Ok(())
    }

    fn install_spotify(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("spotify")?;
        Ok(())
    }

    fn install_steam(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("steam-client")?;
        Ok(())
    }

    fn install_sweet_home_3d(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("sweet-home-3d")?;
        Ok(())
    }

    async fn install_system_extras(&self, config: &Config) -> Result<(), Box<dyn Error>> {
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
        self.install_application("7zip")?;
        self.install_application("microsoft-windows-terminal")?;
        if config.development {
            self.execute_powershell("wsl --install", true)?;
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
        self.install_application("vim")?;
        Ok(())
    }

    fn install_vlc(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("vlc")?;
        Ok(())
    }

    fn install_vm_tools(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_vscode(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("vscode")?;
        Ok(())
    }

    async fn install_wifi(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_window_manager(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_wget(&self) -> Result<(), Box<dyn Error>> {
        self.install_application("wget")?;
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
