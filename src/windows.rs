use std::error::Error;
use std::fs;
use std::process::Command;

use async_trait::async_trait;

use registry::{Data, Hive, Security};
use utfx::U16CString;

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
        self.execute_wsl(
            format!("-u root apt -y install {}", application).as_str(),
            true,
        )
    }

    fn is_installed(&self, application: &str) -> Result<bool, Box<dyn Error>> {
        let mut winget = Command::new("winget");
        let winget_command = winget.args(vec!["list", "--id", application]);
        let winget_output = system::run_command(winget_command, false, false)?;
        if !winget_output.contains("No installed package found matching input criteria.") {
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

    fn refreshenv(&self) -> Result<String, Box<dyn Error>> {
        self.execute_powershell(
            "$env:Path = [System.Environment]::GetEnvironmentVariable(\"Path\",\"Machine\") + \";\" + [System.Environment]::GetEnvironmentVariable(\"Path\",\"User\")",
            false,
        )
    }
}

#[async_trait]
impl System for Windows<'_> {
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
            format!(
                "winget install --accept-source-agreements --accept-package-agreements --id {}",
                applications.join(" ")
            )
            .as_str(),
            true,
        )
    }

    fn install_affinity_suite(&self) -> Result<(), Box<dyn Error>> {
        // Affinity Photo 2
        if !self.is_installed("9P8DVF1XW02V")? {
            self.install_application("9P8DVF1XW02V")?;
        }
        // Affinity Publisher 2
        if !self.is_installed("9NTV2DZ11KD9")? {
            self.install_application("9NTV2DZ11KD9")?;
        }
        // Affinity Designer 2
        if !self.is_installed("9N2D0P16C80H")? {
            self.install_application("9N2D0P16C80H")?;
        }
        Ok(())
    }

    fn install_android_studio(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Google.AndroidStudio")? {
            self.install_application("Google.AndroidStudio")?;
        }
        Ok(())
    }

    fn install_archiver(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("7zip.7zip")? {
            self.install_application("7zip.7zip")?;
        }
        Ok(())
    }

    fn install_audacity(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Audacity.Audacity")? {
            self.install_application("Audacity.Audacity")?;
        }

        if !self.is_installed("Gyan.FFmpeg.Shared")? {
            self.install_application("Gyan.FFmpeg.Shared")?;
        }
        Ok(())
    }

    fn install_authy(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Twilio.Authy")? {
            self.install_application("Twilio.Authy")?;
        }
        Ok(())
    }

    fn install_bambu_studio(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Bambulab.Bambustudio")? {
            self.install_application("Bambulab.Bambustudio")?;
        }
        Ok(())
    }

    fn install_bash(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_blender(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("BlenderFoundation.Blender")? {
            self.install_application("BlenderFoundation.Blender")?;
        }
        Ok(())
    }

    fn install_bluetooth(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_calibre(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("calibre.calibre")? {
            self.install_application("calibre.calibre")?;
        }
        Ok(())
    }

    fn install_cplusplus(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("GnuWin32.Make")? {
            self.install_application("GnuWin32.Make")?;
            let regkey = Hive::LocalMachine.open(
                r"System\CurrentControlSet\Control\Session Manager\Environment",
                Security::Read | Security::Write,
            )?;
            for value in regkey.values() {
                let mut opened = value.unwrap();
                let name = opened.name();
                if name.to_string().unwrap() == "Path" {
                    println!("{:?}", name);
                    opened.set_data(Data::String(
                        U16CString::from_str(format!(
                            "{};C:\\Program Files (x86)\\GnuWin32\\bin",
                            opened.data()
                        ))
                        .unwrap(),
                    ))?;
                    break;
                }
            }
            self.refreshenv()?;
        }
        if !self.is_installed("Kitware.CMake")? {
            self.install_application("Kitware.CMake")?;
        }
        if !self.is_installed("Microsoft.VisualStudio.2022.BuildTools")? {
            self.install_application("Microsoft.VisualStudio.2022.BuildTools --silent --override \"--wait --quiet --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended\"")?;
        }
        Ok(())
    }

    async fn install_codecs(&self) -> Result<(), Box<dyn Error>> {
        system::setup_codecs(self).await?;
        fs::create_dir_all("C:\\Program Data\\aacs")?;
        fs::copy(
            format!("{}/.config/aacs/keydb.cfg", self.get_home_dir()).as_str(),
            "C:\\Program Data\\aacs",
        )?;
        Ok(())
    }

    async fn install_cryptomator(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Cryptomator.Cryptomator")? {
            self.install_application("Cryptomator.Cryptomator")?;
        }
        Ok(())
    }

    fn install_curl(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("curl")? {
            self.install_wsl("curl")?;
        }
        // TODO: What to replace this with
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
        if !self.is_installed("Discord.Discord")? {
            self.install_application("Discord.Discord")?;
        }
        Ok(())
    }

    fn install_disk_usage_analyser(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("WinDirStat.WinDirStat")? {
            self.install_application("WinDirStat.WinDirStat")?;
        }
        Ok(())
    }

    fn install_development_extras(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_docker(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Docker.DockerDesktop")? {
            self.install_application("Docker.DockerDesktop")?;
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

    async fn install_eclipse(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Should this just be removed?
        if !self.is_installed("eclipse")? {
            self.install_application("eclipse")?;
        }
        Ok(())
    }

    fn install_epic_games(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("EpicGames.EpicGamesLauncher")? {
            self.install_application("EpicGames.EpicGamesLauncher")?;
        }
        Ok(())
    }

    fn install_exact_audio_copy(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("AndreWiethoff.ExactAudioCopy")? {
            self.install_application("AndreWiethoff.ExactAudioCopy")?;
        }
        Ok(())
    }

    async fn install_exercism(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Exercism.CLI")? {
            self.install_application("Exercism.CLI")?;
        }
        Ok(())
    }

    fn install_firefox(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Mozilla.Firefox")? {
            self.install_application("Mozilla.Firefox")?;
        }
        Ok(())
    }

    fn install_firmware_updater(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_git(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("git")? {
            self.install_wsl("git")?;
            self.execute_wsl("git config --global user.name \"Benjamin Sproule\"", false)?;
            self.execute_wsl(
                "git config --global user.email benjamin@benjaminsproule.com",
                false,
            )?;
            self.execute_wsl(
                "git config --global credential.helper cache --timeout=86400",
                false,
            )?;
        }
        if !self.config.wsl {
            if !self.is_installed("Git.Git")? {
                self.install_application("Git.Git")?;
                self.refreshenv()?;
            }
            system::setup_git_config(self)?;
            self.execute("git config --system core.longpaths true", true)?;
            if !self.is_installed("posh-git")? {
                self.execute_powershell(
                    "Install-Module -Name posh-git -Force -Confirm:$False",
                    true,
                )?;
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
        if !self.is_installed("GIMP.GIMP")? {
            self.install_application("GIMP.GIMP")?;
        }
        Ok(())
    }

    async fn install_godot(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("GodotEngine.GodotEngine.Mono")? {
            self.install_application("GodotEngine.GodotEngine.Mono")?;
        }
        Ok(())
    }

    fn install_gog_galaxy(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("GOG.Galaxy")? {
            self.install_application("GOG.Galaxy")?;
        }
        Ok(())
    }

    async fn install_google_chrome(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Google.Chrome")? {
            self.install_application("Google.Chrome")?;
        }
        Ok(())
    }

    fn install_google_cloud_sdk(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("gcloudsdk")? {
            self.install_wsl("gcloudsdk")?;
        }
        if !self.config.wsl && !self.is_installed("Google.CloudSDK")? {
            self.install_application("Google.CloudSDK")?;
        }
        Ok(())
    }

    fn install_google_drive(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Google.Drive")? {
            self.install_application("Google.Drive")?;
        }
        Ok(())
    }

    fn install_gpg(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("GnuPG.Gpg4win")? {
            self.install_application("GnuPG.Gpg4win")?;
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
        if !self.is_installed("Gramps.Gramps")? {
            self.install_application("Gramps.Gramps")?;
        }
        Ok(())
    }

    fn install_groovy(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("groovy")? {
            self.install_wsl("groovy")?;
        }
        if !self.config.wsl && !self.is_installed("Apache.Groovy.4")? {
            self.install_application("Apache.Groovy.4")?;
        }
        Ok(())
    }

    fn install_handbrake(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("HandBrake.HandBrake")? {
            self.install_application("HandBrake.HandBrake")?;
        }
        Ok(())
    }

    fn install_inkscape(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Inkscape.Inkscape")? {
            self.install_application("Inkscape.Inkscape")?;
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
        if !self.is_installed("JetBrains.IntelliJIDEA.Ultimate")? {
            self.install_application("JetBrains.IntelliJIDEA.Ultimate")?;
        }
        Ok(())
    }

    fn install_jdk(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("openjdk-19-jdk")? {
            self.install_wsl("openjdk-19-jdk")?;
        }
        if !self.config.wsl && !self.is_installed("EclipseAdoptium.Temurin.19.JDK")? {
            self.install_application("EclipseAdoptium.Temurin.19.JDK")?;
        }
        Ok(())
    }

    fn install_keepassxc(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("KeePassXCTeam.KeePassXC")? {
            self.install_application("KeePassXCTeam.KeePassXC")?;
        }
        Ok(())
    }

    async fn install_kubectl(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Kubernetes.kubectl")? {
            self.install_application("Kubernetes.kubectl")?;
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
        if !self.is_installed("ChristianSchenk.MiKTeX")? {
            self.install_application("ChristianSchenk.MiKTeX")?;
        }
        if !self.is_installed("TeXstudio.TeXstudio")? {
            self.install_application("TeXstudio.TeXstudio")?;
        }
        Ok(())
    }

    fn install_office(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("9WZDNCRD29V9")? {
            self.install_application("9WZDNCRD29V9")?;
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
        if !self.is_installed("GuinpinSoft.MakeMKV")? {
            self.install_application("GuinpinSoft.MakeMKV")?;
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
        if !self.is_installed("Kubernetes.minikube")? {
            self.install_application("Kubernetes.minikube")?;
        }
        Ok(())
    }

    fn install_mkvtoolnix(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("MKVToolNix.MKVToolNix")? {
            self.install_application("MKVToolNix.MKVToolNix")?;
        }
        Ok(())
    }

    fn install_networking_tools(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Insecure.Nmap")? {
            self.install_application("Insecure.Nmap")?;
        }
        self.execute(
            "dism /online /Enable-Feature /FeatureName:TelnetClient /NoRestart",
            true,
        )?;
        Ok(())
    }

    fn install_nextcloud_client(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Nextcloud.NextcloudDesktop")? {
            self.install_application("Nextcloud.NextcloudDesktop")?;
        }
        Ok(())
    }

    async fn install_nodejs(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("nvm")? {
            self.execute_wsl(
                "wget -q https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh",
                false,
            )?;
            self.execute_wsl("-u root ./install.sh", true)?;
            self.execute_wsl("rm ./install.sh", false)?;
            self.execute_wsl("echo 'export NVM_DIR=\"$HOME/.nvm\"' > ~/.bashrc", false)?;
            self.execute_wsl(
                "echo '[ -s \"$NVM_DIR/nvm.sh\" ] && \\. \"$NVM_DIR/nvm.sh\"  # This loads nvm' >> ~/.bashrc",
                false,
            )?;
            self.execute_wsl("echo '[ -s \"$NVM_DIR/bash_completion\" ] && \\. \"$NVM_DIR/bash_completion\"  # This loads nvm bash_completion' >> ~/.bashrc", false)?;
            self.execute_wsl("nvm install node", false)?;
        }
        if !self.config.wsl {
            if !self.is_installed("CoreyButler.NVMforWindows")? {
                self.install_application("CoreyButler.NVMforWindows")?;
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
            self.refreshenv()?;
            self.execute("nvm install latest", false)?;
            self.execute("nvm use latest", false)?;
            self.refreshenv()?;
            self.execute_powershell("npm install --global yarn", true)?;
        }
        Ok(())
    }

    async fn install_nordvpn(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("NordVPN.NordVPN")? {
            self.install_application("NordVPN.NordVPN")?;
        }
        Ok(())
    }

    fn install_nvidia_tools(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Nvidia.GeForceExperience")? {
            self.install_application("Nvidia.GeForceExperience")?;
        }
        Ok(())
    }

    fn install_nvidia_laptop_tools(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_obs_studio(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("OBSProject.OBSStudio")? {
            self.install_application("OBSProject.OBSStudio")?;
        }
        Ok(())
    }

    fn install_onedrive(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_origin(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("ElectronicArts.EADesktop")? {
            self.install_application("ElectronicArts.EADesktop")?;
        }
        Ok(())
    }

    fn install_powertop(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn install_python(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("python3")? {
            self.install_wsl("python3")?;
        }
        if !self.config.wsl && !self.is_installed("Python.Python.3.11")? {
            self.install_application("Python.Python.3.11")?;
        }
        Ok(())
    }

    fn install_retroarch(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Libretro.RetroArch")? {
            self.install_application("Libretro.RetroArch")?;
        }
        Ok(())
    }

    async fn install_rust(&self) -> Result<(), Box<dyn Error>> {
        if self.config.wsl && !self.is_installed_wsl("rustup")? {
            self.execute_wsl("wget -q -O rustup-install https://sh.rustup.rs", false)?;
            self.execute_wsl("./rustup-install -y", false)?;
            self.execute_wsl("rm ./rustup-install", false)?;
        }
        // Always install outside of WSL in case this needs updating for Windows
        if !self.is_installed("Rustlang.Rustup")? {
            self.install_application("Rustlang.Rustup")?;
        }
        // Required for compilation
        if !self.is_installed("Microsoft.VisualStudio.2022.BuildTools")? {
            self.install_application("Microsoft.VisualStudio.2022.BuildTools --silent --override \"--wait --quiet --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended\"")?;
        }
        Ok(())
    }

    fn install_rust_rover(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed(" JetBrains.RustRover")? {
            self.install_application(" JetBrains.RustRover")?;
        }
        Ok(())
    }

    fn install_slack(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("SlackTechnologies.Slack")? {
            self.install_application("SlackTechnologies.Slack")?;
        }
        Ok(())
    }

    fn install_spotify(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Spotify.Spotify")? {
            self.install_application("Spotify.Spotify")?;
        }
        Ok(())
    }

    fn install_steam(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Valve.Steam")? {
            self.install_application("Valve.Steam")?;
        }
        Ok(())
    }

    fn install_strawberry_music_player(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("StrawberryMusicPlayer.Strawberry")? {
            self.install_application("StrawberryMusicPlayer.Strawberry")?;
        }
        Ok(())
    }

    fn install_sweet_home_3d(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("eTeks.SweetHome3D")? {
            self.install_application("eTeks.SweetHome3D")?;
        }
        Ok(())
    }

    async fn install_system_extras(&self) -> Result<(), Box<dyn Error>> {
        // Needed to install powershell modules
        self.execute_powershell("Set-ExecutionPolicy Unrestricted", true)?;
        let regkey = Hive::LocalMachine.open(
            r"SYSTEM\CurrentControlSet\Control\FileSystem",
            Security::Read | Security::Write,
        )?;
        for value in regkey.values() {
            let mut opened = value.unwrap();
            let name = opened.name();
            if name.to_string().unwrap() == "LongPathsEnabled" {
                println!("{:?}", name);
                opened.set_data(Data::U32(1))?;
                break;
            }
        }
        if !self.is_installed("Microsoft.PowerShell")? {
            self.install_application("Microsoft.PowerShell")?;
        }
        if !self.is_installed("Microsoft.WindowsTerminal")? {
            self.install_application("Microsoft.WindowsTerminal")?;
        }
        if !self.is_installed("gerardog.gsudo")? {
            self.install_application("gerardog.gsudo")?;
        }
        if self.config.development && self.config.wsl {
            self.execute_powershell("wsl --install -d Ubuntu", true)?;
            self.execute_wsl("-u root apt update", true)?;
            self.execute_wsl("-u root apt dist-upgrade", true)?;
            if !self.is_installed("dorssel.usbipd-win")? {
                self.install_application("dorssel.usbipd-win")?;
            }
            // TODO: Download Linux binary, copy into Ubuntu WSL and run with development only flag
        }
        Ok(())
    }

    fn install_terraform(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("Hashicorp.Terraform")? {
            self.install_application("Hashicorp.Terraform")?;
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
        if !self.is_installed("vim.vim")? {
            self.install_application("vim.vim")?;
        }
        Ok(())
    }

    async fn install_vlc(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("VideoLAN.VLC")? {
            self.install_application("VideoLAN.VLC")?;
        }
        system::download_file(
            "https://vlc-bluray.whoknowsmy.name/files/win64/libaacs.dll",
            "C:\\Program Files\\VideoLAN\\VLC\\libaacs.dll"
                .to_string()
                .as_str(),
        )
        .await?;
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
        if !self.is_installed("Microsoft.VisualStudioCode")? {
            self.install_application("Microsoft.VisualStudioCode")?;
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
        if !self.config.wsl && !self.is_installed("JernejSimoncic.Wget")? {
            self.install_application("JernejSimoncic.Wget")?;
        }
        Ok(())
    }

    fn install_whatsapp(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("WhatsApp.WhatsApp")? {
            self.install_application("WhatsApp.WhatsApp")?;
        }
        Ok(())
    }

    fn install_wine(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn install_xbox_streaming(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed("9MV0B5HZVK9Z")? {
            self.install_application("9MV0B5HZVK9Z")?;
        }
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
        self.execute("winget upgrade --all", true)?;
        Ok(())
    }

    fn update_os_repo(&self) -> Result<(), Box<dyn Error>> {
        self.execute("winget update", true)?;
        Ok(())
    }
}
