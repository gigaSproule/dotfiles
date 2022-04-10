use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::process::Command;
use std::process::Stdio;

use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub(crate) trait System: Send + Sync + 'static {
    /// Executes the given command. It will run it as a super user if `super_user` is `true`.
    ///
    /// The returned Result contains the output of the command.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use system::System;
    ///
    /// let system: System = ...
    /// system.execute("mkdir /path/to/create", true);
    /// ```
    fn execute(
        &self,
        command: &str,
        super_user: bool,
    ) -> Result<String, Box<dyn std::error::Error>>;

    fn get_home_dir(&self) -> String;

    /// Installs the provided application.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use system::System;
    ///
    /// let system: System = ...
    /// system.install_application("application");
    /// ```
    fn install_application(&self, application: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.install_applications(vec![application])
    }

    /// Installs the provided applications.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use system::System;
    ///
    /// let system: System = ...
    /// system.install_applications(vec!["application1", "application2"]);
    /// ```
    fn install_applications<'a>(
        &self,
        applications: Vec<&'a str>,
    ) -> Result<String, Box<dyn std::error::Error>>;

    fn install_android_studio(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_bash(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_blender(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_bluetooth(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn install_codecs(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_conemu(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_cryptomator(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_curl(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_davinci_resolve(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_discord(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_docker(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_dropbox(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn install_eclipse(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_epic_games(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_firefox(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_firmware_updater(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_gog_galaxy(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn install_google_chrome(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_google_cloud_sdk(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_google_drive(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_git(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_gimp(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_gpg(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_gradle(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_graphic_card_tools(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_graphic_card_laptop_tools(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_groovy(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_handbrake(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_inkscape(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_insync(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_intellij(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_jdk(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_keepassxc(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn install_kubectl(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_helm(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_latex(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_lutris(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_maven(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_makemkv(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_microcode(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn install_minikube(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_mkvtoolnix(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_networking_tools(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_nextcloud_client(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn install_nodejs(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn install_nordvpn(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_nvidia_tools(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_nvidia_laptop_tools(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_obs_studio(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_onedrive(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_origin(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_powertop(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_python(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn install_rust(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_slack(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_spotify(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_steam(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_sweet_home_3d(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn install_system_extras(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn install_themes(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_tlp(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_tmux(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_vim(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_vlc(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_vm_tools(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_vscode(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn install_wifi(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_window_manager(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_wget(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_wine(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_xcode(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn install_zsh(&self) -> Result<(), Box<dyn std::error::Error>>;

    /// Sets the required global keyboard shortcuts that conflict with common IDE shortcuts.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use system::System;
    ///
    /// let system: System = ...
    /// system.set_development_shortcuts();
    /// ```
    fn set_development_shortcuts(&self) -> Result<(), Box<dyn std::error::Error>>;

    /// Sets the environment configuration for common local development requirements.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use system::System;
    ///
    /// let system: System = ...
    /// system.set_development_environment_settings();
    /// ```
    fn set_development_environment_settings(&self) -> Result<(), Box<dyn std::error::Error>>;

    /// Sets the environment configuration to enable the best possible power savings.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use system::System;
    ///
    /// let system: System = ...
    /// system.setup_power_saving_tweaks();
    /// ```
    fn setup_power_saving_tweaks(&self) -> Result<(), Box<dyn std::error::Error>>;

    /// Creates a directory for storing user specific binaries to be included on the PATH.
    ///
    /// This will be created under the users home directory.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use system::System;
    ///
    /// let system: System = ...
    /// system.setup_user_bin();
    /// ```
    fn setup_user_bin(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(format!("{}/bin", self.get_home_dir()).as_str())?;
        fs::create_dir_all(format!("{}/.local/bin", self.get_home_dir()).as_str())?;
        Ok(())
    }

    /// Updates all of the OS's software.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use system::System;
    ///
    /// let system: System = ...
    /// system.update_os();
    /// ```
    fn update_os(&self) -> Result<(), Box<dyn std::error::Error>>;

    /// Updates all of the OS's software repositories.
    ///
    /// This _does not_ update any software.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use system::System;
    ///
    /// let system: System = ...
    /// system.update_os_repo();
    /// ```
    fn update_os_repo(&self) -> Result<(), Box<dyn std::error::Error>>;
}

/// Downloads the file found at the given URL and saves it to the specified location.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use system;
///
/// system::download_file("https://some/amazing/file", "some_file").await?;
/// ```
pub(crate) async fn download_file(
    url: &str,
    downloaded_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    let mut file = match File::create(downloaded_file) {
        Err(why) => panic!("Couldn't create {}: {}", downloaded_file, why),
        Ok(file) => file,
    };
    let content = response.bytes().await?;
    file.write_all(&content)?;
    Ok(())
}

/// Returns whether the file contains the given string or not.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use system;
///
/// let contains_text = system::file_contains("/path/to/file", "text");
/// ```
pub(crate) fn file_contains(file: &str, contains: &str) -> bool {
    let file_result = OpenOptions::new().read(true).open(file);
    if file_result.is_err() {
        return false;
    }
    let mut actual_file = file_result.unwrap();
    if actual_file.metadata().is_err() {
        return false;
    }
    let mut buff = String::new();
    let result = actual_file.read_to_string(&mut buff);
    if result.is_err() {
        return false;
    }
    buff.contains(contains)
}

/// Returns the users home directory _without_ the trailing slash.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use system;
///
/// system::get_home_dir();
/// ```
pub(crate) fn get_home_dir() -> String {
    dirs::home_dir()
        .expect("Could not get home directory")
        .into_os_string()
        .into_string()
        .expect("Could not convert home directory to a &str")
}

/// Runs to given Command, printing out the std out and std error, returning a Result with a String of the joined std out and std error.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use system;
///
/// let mut command = Command::new("ls").args(vec!["-la", "."]);
/// system::run_command(command);
/// ```
pub(crate) fn run_command(command: &mut Command) -> Result<String, Box<dyn std::error::Error>> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute process");

    let mut output = Vec::new();

    {
        let stdout = child.stdout.as_mut().expect("Wasn't stdout");
        let stderr = child.stderr.as_mut().expect("Wasn't stderr");

        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        let stdout_lines = stdout_reader.lines();
        for line in stdout_lines {
            let string_line = line.unwrap();
            println!("{}", &string_line);
            output.push(string_line);
        }

        let stderr_lines = stderr_reader.lines();
        for line in stderr_lines {
            let string_line = line.unwrap();
            println!("{}", &string_line);
            output.push(string_line);
        }
    }

    child.wait()?;
    let string_output = output.join("\n");
    Ok(string_output)
}

/// Downloads and configures the codecs.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use system;
///
/// system::setup_codecs();
/// ```
pub(crate) async fn setup_codecs(system: &impl System) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(format!("{}/.config/aacs", system.get_home_dir()).as_str())?;
    download_file(
        "http://vlc-bluray.whoknowsmy.name/files/KEYDB.cfg",
        format!("{}/.config/aacs/KEYDB.cfg", system.get_home_dir()).as_str(),
    )
    .await?;
    Ok(())
}

/// Configures global git config values such as name, email, credential cache and global ignore file.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use system;
///
/// let system: system::System = ...
/// system::setup_git_config(&system);
/// ```
pub(crate) fn setup_git_config(system: &impl System) -> Result<(), Box<dyn std::error::Error>> {
    system.execute("git config --global user.name \"Benjamin Sproule\"", false)?;
    system.execute(
        "git config --global user.email benjamin@benjaminsproule.com",
        false,
    )?;
    system.execute(
        "git config --global credential.helper cache --timeout=86400",
        false,
    )?;
    system.execute(
        &format!(
            "git config --global core.excludesfile {}/.gitignore",
            get_home_dir()
        ),
        false,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_contains_file_does_not_exist() {
        let result = file_contains("tests/does-not-exist.txt", "content");
        assert_eq!(result, false);
    }

    #[test]
    fn test_file_contains_file_does_not_contain_text() {
        let result = file_contains("tests/test-file.txt", "does not exist");
        assert_eq!(result, false);
    }

    #[test]
    fn test_file_contains_file_contains_text() {
        let result = file_contains("tests/test-file.txt", "content");
        assert_eq!(result, true);
    }
}
