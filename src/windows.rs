use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::{Command, Output, Stdio};

use async_trait::async_trait;

use crate::system;
use crate::system::System;

pub(crate) struct Windows {}

impl Default for Windows {
    fn default() -> Self {
        Windows {}
    }
}

impl Windows {
    fn execute_powershell(&self, command: &str, _super_user: bool) -> Output {
        let mut split = command.split_whitespace();
        Command::new("powershell")
            .arg("/C")
            .args(&mut split)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect(format!("Failed to execute process `{}`", command).as_str())
    }
}

#[async_trait]
impl System for Windows {
    fn execute(&self, command: &str, _super_user: bool) -> Output {
        let mut split = command.split_whitespace();
        Command::new("cmd")
            .arg("/C")
            .args(&mut split)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect(format!("Failed to execute process `{}`", command).as_str())
    }

    fn install_applications(&self, applications: Vec<&str>) -> Output {
        self.execute(
            format!("choco install {}", applications.join(" ")).as_str(),
            true,
        )
    }

    fn install_android_studio(&self) {
        self.install_application("androidstudio");
    }

    fn install_blender(&self) {
        self.install_application("blender");
    }

    fn install_bluetooth(&self) {
        // no-op
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn std::error::Error>> {
        system::setup_codecs().await
    }

    fn install_conemu(&self) {
        self.install_application("conemu");
    }

    fn install_cryptomator(&self) {
        self.install_application("cryptomator");
    }

    fn install_curl(&self) {
        self.install_application("curl");
    }

    fn install_davinci_resolve(&self) -> Result<(), std::io::Error> {
        open::that("https://www.blackmagicdesign.com/uk/products/davinciresolve/studio")
    }

    fn install_discord(&self) {
        self.install_application("discord");
    }

    fn install_docker(&self) -> Result<(), std::io::Error> {
        self.install_application("docker-desktop");
        self.execute_powershell("Install-Module -Name DockerCompletion -Force", true);
        self.execute_powershell("Import-Module DockerCompletion", true);
        fs::write(
            "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\profile.ps1",
            "Import-Module DockerCompletion\r\n".as_bytes(),
        )?;
        Ok(())
    }

    fn install_dropbox(&self) {
        self.install_application("dropbox");
    }

    async fn install_eclipse(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("eclipse");
        Ok(())
    }

    fn install_epic_games(&self) {
        self.install_application("epicgameslauncher");
    }

    fn install_firefox(&self) {
        self.install_application("firefox");
    }

    fn install_firmware_updater(&self) {
        // no-op
    }

    fn install_gog_galaxy(&self) {
        self.install_application("goggalaxy");
    }

    async fn install_google_chrome(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("googlechrome");
        Ok(())
    }

    fn install_google_cloud_sdk(&self) -> Result<(), std::io::Error> {
        self.install_application("gcloudsdk");
        Ok(())
    }

    fn install_google_drive(&self) {
        self.install_application("google-drive-file-stream");
    }

    fn install_git(&self) -> Result<(), std::io::Error> {
        self.install_application("git");
        system::setup_git_config(self)?;
        self.execute("git config --system core.longpaths true", false);
        self.install_application("poshgit");
        Ok(())
    }

    fn install_gimp(&self) {
        self.install_application("gimp");
    }

    fn install_gpg(&self) {
        self.install_application("gpg4win");
    }

    fn install_gradle(&self) {
        self.install_application("gradle");
    }

    fn install_graphic_card_tools(&self) {
        self.install_nvidia_tools();
    }

    fn install_graphic_card_laptop_tools(&self) {
        self.install_nvidia_laptop_tools();
    }

    fn install_groovy(&self) {
        self.install_application("groovy");
    }

    fn install_handbrake(&self) {
        self.install_application("handbrake");
    }

    fn install_inkscape(&self) {
        self.install_application("inkscape");
    }

    fn install_insync(&self) {
        // no-op
    }

    fn install_intellij(&self) {
        self.install_application("intellijidea-ultimate");
    }

    fn install_jdk(&self) -> Result<(), std::io::Error> {
        self.install_application("adoptopenjdk");
        Ok(())
    }

    fn install_keepassxc(&self) {
        self.install_application("keepassxc");
    }

    async fn install_kubectl(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("kubernetes-cli");
        Ok(())
    }

    fn install_helm(&self) {
        self.install_application("kubernetes-helm");
    }

    fn install_latex(&self) {
        self.install_application("texlive");
        self.execute(" C:\\texlive\\2021\\bin\\win32\\tlmgr.bat install latexmk enumitem titlesec latexindent", true);
    }

    fn install_lutris(&self) {
        // no-op
    }

    fn install_maven(&self) {
        self.install_application("maven");
    }

    fn install_makemkv(&self) {
        self.install_application("makemkv");
    }

    fn install_microcode(&self) -> Result<(), std::io::Error> {
        Ok(())
    }

    async fn install_minikube(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("minikube");
    }

    fn install_mkvtoolnix(&self) {
        self.install_application("mkvtoolnix");
    }

    fn install_nextcloud_client(&self) {
        self.install_application("nextcloud-client");
    }

    async fn install_nodejs(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("nvm");
        let mut file = OpenOptions::new()
            .append(true)
            .open("C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\profile.ps1")?;

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
        self.execute_powershell("refreshenv", false);
        self.execute("nvm install latest", false);
        let stdout = &self.execute("nvm list", false).stdout;
        let output =
            std::str::from_utf8(stdout).expect("Could not find any installed npm versions");
        for output_version in output.split("\n") {
            if output_version != "" {
                self.execute(
                    format!("nvm use {}", output_version.replace(" ", "")).as_str(),
                    false,
                );
                break;
            }
        }
        self.execute_powershell("refreshenv", false);
        self.execute_powershell("npm install --global yarn", true);
        Ok(())
    }

    async fn install_nordvpn(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.install_application("nordvpn");
        Ok(())
    }

    fn install_nvidia_tools(&self) {
        self.install_application("geforce-experience");
    }

    fn install_nvidia_laptop_tools(&self) {
        // no-op
    }

    fn install_obs_studio(&self) {
        self.install_application("obs-studio");
    }

    fn install_onedrive(&self) {
        // no-op
    }

    fn install_origin(&self) {
        self.install_application("origin");
    }

    fn install_powertop(&self) {
        // no-op
    }

    fn install_python(&self) {
        self.install_application("python");
    }

    fn install_rust(&self) {
        self.install_application("rustup.install");
    }

    fn install_slack(&self) {
        self.install_application("slack");
    }

    fn install_spotify(&self) {
        self.install_application("spotify");
    }

    fn install_steam(&self) {
        self.install_application("steam");
    }

    fn install_sweet_home_3d(&self) {
        self.install_application("sweet-home-3d");
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.execute_powershell("Set-ExecutionPolicy Unrestricted", true);
        system::download_file("https://chocolatey.org/install.ps1", "install.ps1").await?;
        self.execute_powershell("iex .\\install.ps1", true);
        self.execute_powershell(
            "Install-PackageProvider -Name NuGet -MinimumVersion 2.8.5.201 -Force",
            true,
        );
        self.execute_powershell(
            "Import-Module \"$env:ProgramData\\chocolatey\\helpers\\chocolateyInstaller.psm1\"",
            true,
        );
        self.execute_powershell("refreshenv", true);
        self.execute("REG ADD HKLM\\SYSTEM\\CurrentControlSet\\Control\\FileSystem /v LongPathsEnabled /t REG_DWORD /d 1 /f", true);
        self.install_application("7zip");
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

    fn install_tmux(&self) {
        // no-op
    }

    fn install_vim(&self) {
        self.install_application("vim");
    }

    fn install_vlc(&self) {
        self.install_application("vlc");
    }

    fn install_vm_tools(&self) {
        // no-op
    }

    fn install_vscode(&self) {
        self.install_application("vscode");
    }

    async fn install_wifi(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install_window_manager(&self) {
        // no-op
    }

    fn install_wget(&self) {
        self.install_application("wget");
    }

    fn install_wine(&self) {
        // no-op
    }

    fn install_xcode(&self) {
        // no-op
    }

    async fn install_zsh(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn set_development_shortcuts(&self) {
        // no-op
    }

    fn set_development_environment_settings(&self) {
        // no-op
    }

    fn setup_power_saving_tweaks(&self) {
        // no-op
    }

    fn update_os(&self) {
        self.execute("choco upgrade all", true);
    }

    fn update_os_repo(&self) {
        // no-op
    }
}
