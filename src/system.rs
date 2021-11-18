use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Output;

use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub(crate) trait System: Send + Sync + 'static {
    /// Executes the given command. It will run it as a super user if `super_user` is `true`.
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
    fn execute(&self, command: &str, super_user: bool) -> Output;

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
    fn install_application(&self, application: &str) -> Output {
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
    fn install_applications<'a>(&self, applications: Vec<&'a str>) -> Output;

    fn install_android_studio(&self);

    fn install_blender(&self);

    fn install_bluetooth(&self);

    async fn install_codecs(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_conemu(&self);

    fn install_cryptomator(&self);

    fn install_curl(&self);

    fn install_davinci_resolve(&self) -> Result<(), std::io::Error>;

    fn install_discord(&self);

    fn install_docker(&self) -> Result<(), std::io::Error>;

    fn install_dropbox(&self);

    async fn install_eclipse(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_epic_games(&self);

    fn install_firefox(&self);

    fn install_firmware_updater(&self);

    fn install_gog_galaxy(&self);

    async fn install_google_chrome(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_google_cloud_sdk(&self) -> Result<(), std::io::Error>;

    fn install_google_drive(&self);

    fn install_git(&self) -> Result<(), std::io::Error>;

    fn install_gimp(&self);

    fn install_gpg(&self);

    fn install_gradle(&self);

    fn install_graphic_card_tools(&self);

    fn install_graphic_card_laptop_tools(&self);

    fn install_groovy(&self);

    fn install_handbrake(&self);

    fn install_inkscape(&self);

    fn install_insync(&self);

    fn install_intellij(&self);

    fn install_jdk(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_keepassxc(&self);

    async fn install_kubectl(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_helm(&self);

    fn install_latex(&self);

    fn install_lutris(&self);

    fn install_maven(&self);

    fn install_makemkv(&self);

    fn install_microcode(&self) -> Result<(), std::io::Error>;

    async fn install_minikube(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_mkvtoolnix(&self);

    fn install_nextcloud_client(&self);

    async fn install_nodejs(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn install_nordvpn(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_nvidia_tools(&self);

    fn install_nvidia_laptop_tools(&self);

    fn install_obs_studio(&self);

    fn install_onedrive(&self);

    fn install_origin(&self);

    fn install_powertop(&self);

    fn install_python(&self);

    async fn install_rust(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_slack(&self);

    fn install_spotify(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_steam(&self);

    fn install_sweet_home_3d(&self);

    async fn install_system_extras(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_telnet(&self);

    async fn install_themes(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_tlp(&self);

    fn install_tmux(&self) -> Result<(), std::io::Error>;

    fn install_vim(&self);

    fn install_vlc(&self);

    fn install_vm_tools(&self);

    fn install_vscode(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn install_wifi(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn install_window_manager(&self);

    fn install_wget(&self);

    fn install_wine(&self);

    fn install_xcode(&self);

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
    fn set_development_shortcuts(&self);

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
    fn set_development_environment_settings(&self) -> Result<(), std::io::Error>;

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
    fn setup_power_saving_tweaks(&self) -> Result<(), std::io::Error>;

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
    fn setup_user_bin(&self) -> Result<(), std::io::Error> {
        fs::create_dir_all(format!("{}/bin", get_home_dir()).as_str())?;
        fs::create_dir_all(format!("{}/.local/bin", get_home_dir()).as_str())
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
    fn update_os(&self);

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
    fn update_os_repo(&self);
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
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let content = response.bytes().await?;
    file.write_all(&content)?;
    Ok(())
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
pub(crate) async fn setup_codecs() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(format!("{}/.config/aacs", get_home_dir()).as_str())?;
    download_file(
        "http://vlc-bluray.whoknowsmy.name/files/KEYDB.cfg",
        format!("{}/.config/aacs/KEYDB.cfg", get_home_dir()).as_str(),
    )
    .await?;
    // let user_id = system.get_user_id();
    // let group_id = system.get_group_id();
    // system.recursively_chown(
    //     format!("{}/.config", get_home_dir()).as_str(),
    //     user_id,
    //     group_id,
    // )?;
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
pub(crate) fn setup_git_config(system: &dyn System) -> Result<(), std::io::Error> {
    system.execute("git config --global user.name \"Benjamin Sproule\"", false);
    system.execute(
        "git config --global user.email benjamin@benjaminsproule.com",
        false,
    );
    system.execute(
        "git config --global credential.helper cache --timeout=86400",
        false,
    );
    system.execute(
        &format!(
            "git config --global core.excludesfile {}/.gitignore",
            get_home_dir()
        ),
        false,
    );
    Ok(())
}
