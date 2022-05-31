use crate::config::Config;
use crate::system::System;

pub(crate) async fn install<'s>(
    config: &'s Config,
    system: &dyn System,
) -> Result<(), Box<dyn std::error::Error>> {
    system.setup_user_bin()?;

    println!("Installing Distro Specific Extras");
    system.install_system_extras().await?;
    system.update_os()?;

    println!("Installing Window Manager");
    system.install_window_manager()?;
    println!("Installing Graphic Card Tools");
    system.install_graphic_card_tools()?;

    println!("Installing Bash");
    system.install_bash()?;
    println!("Installing Cryptomator");
    system.install_cryptomator()?;
    println!("Installing ConEmu");
    system.install_conemu()?;
    println!("Installing Curl");
    system.install_curl()?;
    println!("Installing KeepassXC");
    system.install_keepassxc()?;
    println!("Install Networking Tools");
    system.install_networking_tools()?;
    println!("Installing tmux");
    system.install_tmux()?;
    println!("Installing Vim");
    system.install_vim()?;
    println!("Installing Wget");
    system.install_wget()?;
    println!("Installing ZSH");
    system.install_zsh().await?;

    if config.browsers {
        println!("Installing Firefox");
        system.install_firefox()?;
        println!("Installing Google Chrome");
        system.install_google_chrome().await?;
        println!("Installing Microsoft Edge");
        system.install_microsoft_edge()?;
    }

    if config.development {
        println!("Installing Android Studio");
        // system.install_android_studio();
        println!("Installing Eclipse");
        // system.install_eclipse().await?;
        println!("Installing Gradle");
        system.install_gradle()?;
        println!("Installing Git");
        system.install_git()?;
        println!("Installing Groovy");
        system.install_groovy()?;
        println!("Installing IntelliJ");
        system.install_intellij()?;
        println!("Installing Java");
        system.install_jdk()?;
        println!("Installing Maven");
        system.install_maven()?;
        println!("Installing NodeJS");
        system.install_nodejs().await?;
        println!("Installing Python");
        system.install_python()?;
        println!("Installing Rust");
        system.install_rust().await?;
        println!("Installing Slack");
        system.install_slack()?;
        println!("Installing VSCode");
        system.install_vscode()?;
        println!("Installing Xcode");
        system.install_xcode()?;
        println!("Setting development specific shortcuts");
        system.set_development_shortcuts()?;
        println!("Setting development environment settings");
        system.set_development_environment_settings()?;
    }

    if config.docker {
        println!("Installing Docker");
        system.install_docker()?;
        println!("Installing Kubectl");
        system.install_kubectl().await?;
        println!("Installing Helm");
        system.install_helm()?;
        println!("Installing Minikube");
        // system.install_minikube();
    }

    if config.gaming {
        println!("Installing Discord");
        system.install_discord()?;
        println!("Installing Epic Games");
        system.install_epic_games()?;
        println!("Installing GOG Galaxy");
        system.install_gog_galaxy()?;
        println!("Installing Lutris");
        system.install_lutris()?;
        println!("Installing Origin");
        system.install_origin()?;
        println!("Installing Steam");
        system.install_steam()?;
        println!("Installing Wine");
        system.install_wine()?;
    }

    if config.gcp {
        println!("Installing Google Cloud SDK");
        system.install_google_cloud_sdk()?;
    }

    if config.images {
        println!("Installing Gimp");
        system.install_gimp()?;
        println!("Installing Inkscape");
        system.install_inkscape()?;
    }

    if config.laptop {
        println!("Installing Bluetooth");
        system.install_bluetooth()?;
        println!("Installing FWUPD");
        system.install_firmware_updater()?;
        println!("Installing Graphics Card Tools for Laptop");
        system.install_graphic_card_laptop_tools()?;
        println!("Installing Microcode");
        system.install_microcode()?;
        println!("Installing Powertop");
        system.install_powertop()?;
        println!("Installing TLP");
        system.install_tlp()?;
        println!("Install WiFi");
        system.install_wifi().await?;
        println!("Setup power saving tweaks");
        system.setup_power_saving_tweaks()?;
    }

    if config.modelling {
        println!("Installing Blender");
        system.install_blender()?;
    }

    if config.personal {
        println!("Installing Dropbox");
        system.install_dropbox()?;
        println!("Installing Google Drive");
        system.install_google_drive()?;
        println!("Installing GPG");
        system.install_gpg()?;
        println!("Installing Insync");
        system.install_insync()?;
        println!("Installing LaTeX");
        system.install_latex()?;
        println!("Installing Nextcloud Client");
        system.install_nextcloud_client()?;
        println!("Installing OneDrive");
        system.install_onedrive()?;
        println!("Installing Spotify");
        system.install_spotify()?;
        println!("Installing SweetHome3D");
        system.install_sweet_home_3d()?;
        println!("Installing themes");
        system.install_themes().await?;
    }

    if config.recording {
        println!("Installing OBS Studio");
        system.install_obs_studio()?;
    }

    if config.ripping {
        println!("Installing Handbrake");
        system.install_handbrake()?;
        println!("Installing MakeMKV");
        system.install_makemkv()?;
        println!("Installing MKVToolNix");
        system.install_mkvtoolnix()?;
    }

    if config.video {
        println!("Installing Codecs");
        system.install_codecs().await?;
        println!("Installing VLC");
        system.install_vlc()?;
    }

    if config.video_editing {
        println!("Installing DaVinci Resolve");
        system.install_davinci_resolve()?;
    }

    if config.vm {
        println!("Installing VM Tools");
        system.install_vm_tools()?;
    }

    if config.vpn {
        println!("Installing NordVPN");
        system.install_nordvpn().await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::system::MockSystem;
    use async_trait::async_trait;
    use mockall::{mock, predicate::eq};

    use super::*;

    #[test]
    fn test_install_browsers() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: true,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            help: false,
            images: false,
            laptop: false,
            modelling: false,
            personal: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_firefox()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_google_chrome()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_microsoft_edge()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_development() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            development: true,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            help: false,
            images: false,
            laptop: false,
            modelling: false,
            personal: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_gradle()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_git()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_groovy()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_intellij()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_jdk()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_maven()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_nodejs()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_python()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_rust()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_slack()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_vscode()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_xcode()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_set_development_shortcuts()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_set_development_environment_settings()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_docker() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            development: false,
            docker: true,
            dry_run: false,
            gaming: false,
            gcp: false,
            help: false,
            images: false,
            laptop: false,
            modelling: false,
            personal: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_docker()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_kubectl()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_helm()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_gaming() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: true,
            gcp: false,
            help: false,
            images: false,
            laptop: false,
            modelling: false,
            personal: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_discord()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_epic_games()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_gog_galaxy()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_lutris()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_origin()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_steam()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_wine()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_gcp() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: true,
            help: false,
            images: false,
            laptop: false,
            modelling: false,
            personal: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_google_cloud_sdk()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_images() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            help: false,
            images: true,
            laptop: false,
            modelling: false,
            personal: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_gimp()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_inkscape()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_laptop() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            help: false,
            images: false,
            laptop: true,
            modelling: false,
            personal: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_bluetooth()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_firmware_updater()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_graphic_card_laptop_tools()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_microcode()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_powertop()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_tlp()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_wifi()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_setup_power_saving_tweaks()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_modelling() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            help: false,
            images: false,
            laptop: false,
            modelling: true,
            personal: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_blender()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_personal() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            help: false,
            images: false,
            laptop: false,
            modelling: false,
            personal: true,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_dropbox()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_google_drive()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_gpg()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_insync()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_latex()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_nextcloud_client()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_onedrive()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_spotify()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_sweet_home_3d()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_themes()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_recording() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            help: false,
            images: false,
            laptop: false,
            modelling: false,
            personal: false,
            recording: true,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_obs_studio()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_ripping() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            help: false,
            images: false,
            laptop: false,
            modelling: false,
            personal: false,
            recording: false,
            ripping: true,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_handbrake()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_makemkv()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_mkvtoolnix()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_video() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            help: false,
            images: false,
            laptop: false,
            modelling: false,
            personal: false,
            recording: false,
            ripping: false,
            video: true,
            video_editing: false,
            vm: false,
            vpn: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_codecs()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_vlc()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_video_editing() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            help: false,
            images: false,
            laptop: false,
            modelling: false,
            personal: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: true,
            vm: false,
            vpn: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_davinci_resolve()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_vm() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            help: false,
            images: false,
            laptop: false,
            modelling: false,
            personal: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: true,
            vpn: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_vm_tools()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_vpn() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            help: false,
            images: false,
            laptop: false,
            modelling: false,
            personal: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: true,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_nordvpn()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    fn get_mock_system<'s>(_config: &'s Config) -> MockSystem {
        let mut mock_system = MockSystem::new();
        mock_system
            .expect_setup_user_bin()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_system_extras()
            // .with(eq(config))
            .times(1)
            .returning(|_passed_config| Box::pin(async { Ok(()) }));
        mock_system.expect_update_os().times(1).returning(|| Ok(()));
        mock_system
            .expect_install_window_manager()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_graphic_card_tools()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_bash()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_cryptomator()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_conemu()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_curl()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_keepassxc()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_networking_tools()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_tmux()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_vim()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_wget()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_zsh()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
    }
}
