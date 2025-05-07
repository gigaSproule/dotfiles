use std::error::Error;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::{fs, io};

use crate::system;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use wgpu::{Adapter, Backends};

#[cfg(any(target_os = "linux", target_os = "windows"))]
static BACKENDS: Backends = Backends::VULKAN;

#[cfg(target_os = "macos")]
static BACKENDS: Backends = Backends::METAL;

#[async_trait]
#[cfg_attr(test, automock)]
pub(crate) trait System: Send + Sync {
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
    fn execute(&self, command: &str, super_user: bool) -> Result<String, Box<dyn Error>>;

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
    fn install_application(&self, application: &str) -> Result<String, Box<dyn Error>> {
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
    ) -> Result<String, Box<dyn Error>>;

    fn install_affinity_suite(&self) -> Result<(), Box<dyn Error>>;

    fn install_android_studio(&self) -> Result<(), Box<dyn Error>>;

    fn install_archiver(&self) -> Result<(), Box<dyn Error>>;

    fn install_audacity(&self) -> Result<(), Box<dyn Error>>;

    fn install_authy(&self) -> Result<(), Box<dyn Error>>;

    fn install_bambu_studio(&self) -> Result<(), Box<dyn Error>>;

    fn install_bash(&self) -> Result<(), Box<dyn Error>>;

    fn install_blender(&self) -> Result<(), Box<dyn Error>>;

    fn install_bluetooth(&self) -> Result<(), Box<dyn Error>>;

    fn install_calibre(&self) -> Result<(), Box<dyn Error>>;

    async fn install_codecs(&self) -> Result<(), Box<dyn Error>>;

    fn install_cplusplus(&self) -> Result<(), Box<dyn Error>>;

    async fn install_cryptomator(&self) -> Result<(), Box<dyn Error>>;

    fn install_curl(&self) -> Result<(), Box<dyn Error>>;

    fn install_davinci_resolve(&self) -> Result<(), Box<dyn Error>>;

    fn install_discord(&self) -> Result<(), Box<dyn Error>>;

    fn install_disk_usage_analyser(&self) -> Result<(), Box<dyn Error>>;

    fn install_development_extras(&self) -> Result<(), Box<dyn Error>>;

    fn install_docker(&self) -> Result<(), Box<dyn Error>>;

    async fn install_eclipse(&self) -> Result<(), Box<dyn Error>>;

    fn install_epic_games(&self) -> Result<(), Box<dyn Error>>;

    fn install_exact_audio_copy(&self) -> Result<(), Box<dyn Error>>;

    async fn install_exercism(&self) -> Result<(), Box<dyn Error>>;

    fn install_firefox(&self) -> Result<(), Box<dyn Error>>;

    fn install_firmware_updater(&self) -> Result<(), Box<dyn Error>>;

    fn install_git(&self) -> Result<(), Box<dyn Error>>;

    fn install_gimp(&self) -> Result<(), Box<dyn Error>>;

    async fn install_godot(&self) -> Result<(), Box<dyn Error>>;

    fn install_gog_galaxy(&self) -> Result<(), Box<dyn Error>>;

    async fn install_google_chrome(&self) -> Result<(), Box<dyn Error>>;

    fn install_google_cloud_sdk(&self) -> Result<(), Box<dyn Error>>;

    fn install_google_drive(&self) -> Result<(), Box<dyn Error>>;

    fn install_gpg(&self) -> Result<(), Box<dyn Error>>;

    fn install_gradle(&self) -> Result<(), Box<dyn Error>>;

    fn install_gramps(&self) -> Result<(), Box<dyn Error>>;

    fn install_graphic_card_tools(&self) -> Result<(), Box<dyn Error>> {
        let gpus = system::get_gpus();
        if gpus.iter().any(|gpu| gpu.to_lowercase().contains("nvidia")) {
            self.install_nvidia_tools()?;
        }
        Ok(())
    }

    fn install_graphic_card_laptop_tools(&self) -> Result<(), Box<dyn Error>> {
        let gpus = system::get_gpus();
        if gpus.iter().any(|gpu| gpu.to_lowercase().contains("nvidia")) {
            self.install_nvidia_laptop_tools()?;
        }
        if gpus.iter().any(|gpu| gpu.to_lowercase().contains("intel")) {
            self.install_intel_gpu_laptop_tools()?;
        }
        Ok(())
    }

    fn install_groovy(&self) -> Result<(), Box<dyn Error>>;

    fn install_handbrake(&self) -> Result<(), Box<dyn Error>>;

    fn install_inkscape(&self) -> Result<(), Box<dyn Error>>;

    fn install_insync(&self) -> Result<(), Box<dyn Error>>;

    fn install_intel_gpu_laptop_tools(&self) -> Result<(), Box<dyn Error>>;

    fn install_intellij(&self) -> Result<(), Box<dyn Error>>;

    fn install_jdk(&self) -> Result<(), Box<dyn Error>>;

    fn install_keepassxc(&self) -> Result<(), Box<dyn Error>>;

    async fn install_kubectl(&self) -> Result<(), Box<dyn Error>>;

    async fn install_helm(&self) -> Result<(), Box<dyn Error>>;

    fn install_latex(&self) -> Result<(), Box<dyn Error>>;

    fn install_office(&self) -> Result<(), Box<dyn Error>>;

    fn install_openscad(&self) -> Result<(), Box<dyn Error>>;

    fn install_lutris(&self) -> Result<(), Box<dyn Error>>;

    fn install_maven(&self) -> Result<(), Box<dyn Error>>;

    fn install_makemkv(&self) -> Result<(), Box<dyn Error>>;

    fn install_microcode(&self) -> Result<(), Box<dyn Error>>;

    fn install_microsoft_edge(&self) -> Result<(), Box<dyn Error>>;

    async fn install_minikube(&self) -> Result<(), Box<dyn Error>>;

    fn install_mkvtoolnix(&self) -> Result<(), Box<dyn Error>>;

    fn install_networking_tools(&self) -> Result<(), Box<dyn Error>>;

    fn install_nextcloud_client(&self) -> Result<(), Box<dyn Error>>;

    async fn install_nodejs(&self) -> Result<(), Box<dyn Error>>;

    async fn install_nordvpn(&self) -> Result<(), Box<dyn Error>>;

    fn install_nvidia_tools(&self) -> Result<(), Box<dyn Error>>;

    fn install_nvidia_laptop_tools(&self) -> Result<(), Box<dyn Error>>;

    fn install_obs_studio(&self) -> Result<(), Box<dyn Error>>;

    fn install_onedrive(&self) -> Result<(), Box<dyn Error>>;

    fn install_origin(&self) -> Result<(), Box<dyn Error>>;

    fn install_retroarch(&self) -> Result<(), Box<dyn Error>>;

    fn install_powertop(&self) -> Result<(), Box<dyn Error>>;

    fn install_python(&self) -> Result<(), Box<dyn Error>>;

    async fn install_rust(&self) -> Result<(), Box<dyn Error>>;

    fn install_rust_rover(&self) -> Result<(), Box<dyn Error>>;

    fn install_slack(&self) -> Result<(), Box<dyn Error>>;

    fn install_spotify(&self) -> Result<(), Box<dyn Error>>;

    fn install_steam(&self) -> Result<(), Box<dyn Error>>;

    fn install_strawberry_music_player(&self) -> Result<(), Box<dyn Error>>;

    fn install_sweet_home_3d(&self) -> Result<(), Box<dyn Error>>;

    async fn install_system_extras(&self) -> Result<(), Box<dyn Error>>;

    fn install_terraform(&self) -> Result<(), Box<dyn Error>>;

    async fn install_themes(&self) -> Result<(), Box<dyn Error>>;

    fn install_tlp(&self) -> Result<(), Box<dyn Error>>;

    fn install_tmux(&self) -> Result<(), Box<dyn Error>>;

    fn install_vim(&self) -> Result<(), Box<dyn Error>>;

    async fn install_vlc(&self) -> Result<(), Box<dyn Error>>;

    fn install_vm_tools(&self) -> Result<(), Box<dyn Error>>;

    fn install_vscode(&self) -> Result<(), Box<dyn Error>>;

    async fn install_wifi(&self) -> Result<(), Box<dyn Error>>;

    fn install_window_manager(&self) -> Result<(), Box<dyn Error>>;

    fn install_wget(&self) -> Result<(), Box<dyn Error>>;

    fn install_whatsapp(&self) -> Result<(), Box<dyn Error>>;

    fn install_wine(&self) -> Result<(), Box<dyn Error>>;

    async fn install_xbox_streaming(&self) -> Result<(), Box<dyn Error>>;

    fn install_xcode(&self) -> Result<(), Box<dyn Error>>;

    async fn install_zsh(&self) -> Result<(), Box<dyn Error>>;

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
    fn set_development_shortcuts(&self) -> Result<(), Box<dyn Error>>;

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
    fn set_development_environment_settings(&self) -> Result<(), Box<dyn Error>>;

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
    fn setup_power_saving_tweaks(&self) -> Result<(), Box<dyn Error>>;

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
    fn setup_user_bin(&self) -> Result<(), Box<dyn Error>>;

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
    fn update_os(&self) -> Result<(), Box<dyn Error>>;

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
    fn update_os_repo(&self) -> Result<(), Box<dyn Error>>;
}

/// Adds the content to the file, only if it doesn't already exist within the file.
///
/// # Example
///
/// ```no_run
/// use system;
///
/// system::add_to_file(".zshrc", "export MY_VAR=\"my value\""); // Will add to the file
/// system::add_to_file(".zshrc", "export MY_VAR=\"my value\""); // Will not do anything
/// ```
pub(crate) fn add_to_file(file: &str, content: &str) -> Result<(), io::Error> {
    let path = Path::new(file);
    let mut components = path.components();
    components.next_back();
    let directory = components.as_path();
    fs::create_dir_all(directory)?;
    if !file_contains(file, content) {
        let mut actual_file = OpenOptions::new().create(true).append(true).open(file)?;
        writeln!(actual_file, "{}", content)?;
    }
    Ok(())
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
pub(crate) async fn download_file(url: &str, downloaded_file: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;

    let mut file = match File::create(downloaded_file) {
        Err(why) => panic!("Couldn't create {}: {}", downloaded_file, why),
        Ok(file) => file,
    };
    let content = response.bytes().await?;
    file.write_all(&content)?;
    Ok(())
}

/// Extracts all the contents of the given zip file into the desired directory.
///
/// TODO: Implement removing the top level directory in the zip file.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use system;
/// use std::path::Path;
///
/// system::extract_zip(Path::new("path/to/file.zip"), Path::new("path/to/destination"), false)?;
/// ```
pub(crate) fn extract_zip(
    zip_file: &Path,
    target_dir: &Path,
    _remove_top_level: bool,
) -> Result<(), Box<dyn Error>> {
    if !target_dir.exists() {
        fs::create_dir_all(target_dir)?;
    }

    let file = fs::File::open(zip_file)?;

    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => target_dir.join(path),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if file.is_dir() {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }
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

/// Returns a list of GPUs.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use system;
///
/// system::get_gpus();
/// ```
pub(crate) fn get_gpus() -> Vec<String> {
    let instance = wgpu::Instance::default();
    let adapters: Vec<Adapter> = instance.enumerate_adapters(BACKENDS);

    let mut names: Vec<String> = Vec::new();

    for adapter in adapters {
        names.push(adapter.get_info().name.to_string());
    }

    names
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

/// Optionally runs to given Command (based on dry_run), optionally printing out the std out and std error (based on print_output), returning a Result with a String of the joined std out and std error.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use system;
///
/// let mut command = Command::new("ls").args(vec!["-la", "."]);
/// system::run_command(command, true, false);
/// ```
pub(crate) fn run_command(
    command: &mut Command,
    print_output: bool,
    dry_run: bool,
) -> Result<String, Box<dyn Error>> {
    let mut dry_run_command = Command::new("echo");
    let actual_command = if dry_run {
        let args: Vec<&OsStr> = command.get_args().collect();
        let joined_args = args
            .iter()
            .map(|arg| arg.to_str().unwrap())
            .collect::<Vec<&str>>()
            .join(" ");
        let program = command.get_program();
        dry_run_command.arg(format!("{} {}", program.to_str().unwrap(), joined_args))
    } else {
        command
    };

    let mut child = actual_command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute process");

    let mut output: Vec<String> = Vec::new();

    {
        let stdout = child.stdout.as_mut().expect("Wasn't stdout");
        let stderr = child.stderr.as_mut().expect("Wasn't stderr");

        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        let stdout_lines = stdout_reader.lines();
        for line in stdout_lines {
            let string_line = line.unwrap();
            if print_output {
                println!("{}", &string_line);
            }
            output.push(string_line);
        }

        let stderr_lines = stderr_reader.lines();
        for line in stderr_lines {
            let string_line = line.unwrap();
            if print_output {
                println!("{}", &string_line);
            }
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
pub(crate) async fn setup_codecs(system: &impl System) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(format!("{}/.config/aacs", system.get_home_dir()).as_str())?;
    download_file(
        "http://fvonline-db.bplaced.net/fv_download.php?lang=eng",
        "keydb_eng.zip",
    )
    .await?;
    extract_zip(
        Path::new("keydb_eng.zip"),
        Path::new(format!("{}/.config/aacs/", system.get_home_dir()).as_str()),
        false,
    )?;
    fs::remove_file("keydb_eng.zip")?;
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
pub(crate) fn setup_git_config(system: &impl System) -> Result<(), Box<dyn Error>> {
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
    open::that("https://docs.github.com/en/authentication/managing-commit-signature-verification/telling-git-about-your-signing-key")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    #[test]
    fn test_file_contains_file_does_not_exist() {
        let result = file_contains("tests/does-not-exist.txt", "content");
        assert!(!result);
    }

    #[test]
    fn test_file_contains_file_does_not_contain_text() {
        let result = file_contains("tests/test-file.txt", "does not exist");
        assert!(!result);
    }

    #[test]
    #[serial]
    fn test_add_to_file_appends_content_to_file() {
        let path = &"tests/created-file.txt";
        let create_result = File::create(path);
        create_result.expect("Failed to create file");

        let result = add_to_file(path, "content");
        assert!(result.is_ok());

        let mut created_file = File::open(path).unwrap();
        let mut file_contents = String::new();
        created_file
            .read_to_string(&mut file_contents)
            .expect("Failed to read content of file");
        assert_eq!(file_contents, "content\n");

        let delete_result = fs::remove_file(path);
        delete_result.expect("Failed to delete file");
    }

    #[test]
    #[serial]
    fn test_add_to_file_creates_file_if_not_exist() {
        let path = &"tests/created-file.txt";

        let result = add_to_file(path, "content");
        assert!(result.is_ok());

        let mut created_file = File::open(path).unwrap();
        let mut file_contents = String::new();
        created_file
            .read_to_string(&mut file_contents)
            .expect("Failed to read content of file");
        assert_eq!(file_contents, "content\n");

        let delete_result = fs::remove_file(path);
        delete_result.expect("Failed to delete file");
    }

    #[test]
    #[serial]
    fn test_add_to_file_does_not_duplicate_content() {
        let path = &"tests/created-file.txt";
        let create_result = File::create(path);
        create_result.expect("Failed to create file");

        let first_write_result = add_to_file(path, "content");
        assert!(first_write_result.is_ok());
        let second_write_result = add_to_file(path, "content");
        assert!(second_write_result.is_ok());

        let mut created_file = File::open(path).unwrap();
        let mut file_contents = String::new();
        created_file
            .read_to_string(&mut file_contents)
            .expect("Failed to read content of file");
        assert_eq!(file_contents, "content\n");

        let delete_result = fs::remove_file(path);
        delete_result.expect("Failed to delete file");
    }

    #[test]
    #[serial]
    fn test_extract_zip_produces_correct_directory() {
        let zip_file = Path::new("tests/test-file.zip");
        let target_dir = Path::new("tmp/zip-test-dir");

        let extract_result = extract_zip(zip_file, target_dir, false);
        extract_result.expect("Failed to extract the zip file");

        assert!(Path::new("tmp/zip-test-dir/top-level").exists());
        assert!(Path::new("tmp/zip-test-dir/top-level/sub-directory").exists());
        assert!(Path::new("tmp/zip-test-dir/top-level/sub-directory/file.txt").exists());

        let delete_result = fs::remove_dir_all(target_dir);
        delete_result.expect("Failed to delete the target directory");
    }

    #[ignore = "Needs to be implemented"]
    #[test]
    #[serial]
    fn test_extract_zip_produces_correct_directory_removing_top_level() {
        let zip_file = Path::new("tests/test-file.zip");
        let target_dir = Path::new("tmp/zip-test-dir");

        let extract_result = extract_zip(zip_file, target_dir, true);
        extract_result.expect("Failed to extract the zip file");

        assert!(!Path::new("tmp/zip-test-dir/top-level").exists());
        assert!(Path::new("tmp/zip-test-dir/sub-directory").exists());
        assert!(Path::new("tmp/zip-test-dir/sub-directory/file.txt").exists());

        let delete_result = fs::remove_dir_all(target_dir);
        delete_result.expect("Failed to delete the target directory");
    }
}
