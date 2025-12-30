use crate::config::Config;
use crate::system::System;
use log::info;

pub(crate) async fn install<'s>(
    config: &'s Config,
    system: &dyn System,
) -> Result<(), Box<dyn std::error::Error>> {
    system.setup_user_bin()?;

    info!("Installing Distro Specific Extras");
    system.install_system_extras().await?;
    system.update_os()?;

    if !config.cli_only {
        info!("Installing Window Manager");
        system.install_window_manager()?;
        info!("Installing Graphic Card Tools");
        system.install_graphic_card_tools().await?;
    }

    info!("Installing Bash");
    system.install_bash()?;
    info!("Installing Curl");
    system.install_curl()?;
    info!("Install Networking Tools");
    system.install_networking_tools()?;
    info!("Installing tmux");
    system.install_tmux()?;
    info!("Installing Vim");
    system.install_vim()?;
    info!("Installing Wget");
    system.install_wget()?;
    info!("Installing ZSH");
    system.install_zsh().await?;

    if !config.cli_only {
        info!("Installing archiver");
        system.install_archiver()?;
        info!("Installing Cryptomator");
        system.install_cryptomator().await?;
        info!("Installing KeepassXC");
        system.install_keepassxc()?;
        info!("Installing QuickLook");
        system.install_quicklook()?;
    }

    if config.browsers && !config.cli_only {
        info!("Installing Firefox");
        system.install_firefox()?;
        info!("Installing Google Chrome");
        system.install_google_chrome().await?;
        info!("Installing Microsoft Edge");
        system.install_microsoft_edge()?;
    }

    if config.development {
        info!("Installing C++");
        system.install_cplusplus()?;
        info!("Installing exercism");
        system.install_exercism().await?;
        info!("Installing Gradle");
        system.install_gradle()?;
        info!("Installing Git");
        system.install_git()?;
        info!("Installing Godot");
        system.install_godot().await?;
        info!("Installing Groovy");
        system.install_groovy()?;
        info!("Installing Java");
        system.install_jdk()?;
        info!("Installing Maven");
        system.install_maven()?;
        info!("Installing NodeJS");
        system.install_nodejs().await?;
        info!("Installing Python");
        system.install_python()?;
        info!("Installing Rust");
        system.install_rust().await?;
        info!("Install development specific extras");
        system.install_development_extras()?;
        info!("Setting development environment settings");
        system.set_development_environment_settings()?;

        if !config.cli_only {
            info!("Installing Android Studio");
            system.install_android_studio()?;
            // info!("Installing Eclipse");
            // system.install_eclipse().await?;
            info!("Installing IntelliJ");
            system.install_intellij()?;
            info!("Installing Rust Rover");
            system.install_rust_rover()?;
            info!("Installing Slack");
            system.install_slack()?;
            info!("Installing VSCode");
            system.install_vscode()?;
            info!("Installing Xcode");
            system.install_xcode()?;
            info!("Setting development specific shortcuts");
            system.set_development_shortcuts()?;
        }
    }

    if config.docker {
        info!("Installing Docker");
        system.install_docker()?;
        info!("Installing Kubectl");
        system.install_kubectl().await?;
        info!("Installing Helm");
        system.install_helm().await?;
        // info!("Installing Minikube");
        // system.install_minikube();
    }

    if config.gaming && !config.cli_only {
        info!("Installing Discord");
        system.install_discord()?;
        info!("Installing Epic Games");
        system.install_epic_games().await?;
        info!("Installing GOG Galaxy");
        system.install_gog_galaxy().await?;
        info!("Installing Lutris");
        system.install_lutris()?;
        info!("Installing Origin");
        system.install_origin()?;
        info!("Installing RetroArch");
        system.install_retroarch()?;
        info!("Installing Steam");
        system.install_steam()?;
        info!("Installing Wine");
        system.install_wine()?;
        info!("Installing Xbox streaming");
        system.install_xbox_streaming().await?;
    }

    if config.gcp {
        info!("Installing Google Cloud SDK");
        system.install_google_cloud_sdk()?;
    }

    if config.images && !config.cli_only {
        info!("Installing Affinity Suite");
        system.install_affinity_suite()?;
        info!("Installing Gimp");
        system.install_gimp()?;
        info!("Installing Inkscape");
        system.install_inkscape()?;
    }

    if config.infrastructure {
        info!("Installing Terraform");
        system.install_terraform()?;
    }

    if config.laptop {
        info!("Installing Bluetooth");
        system.install_bluetooth()?;
        info!("Installing FWUPD");
        system.install_firmware_updater()?;
        info!("Installing Microcode");
        system.install_microcode()?;
        info!("Installing Powertop");
        system.install_powertop()?;
        info!("Installing TLP");
        system.install_tlp()?;
        info!("Install WiFi");
        system.install_wifi().await?;
        info!("Setup power saving tweaks");
        system.setup_power_saving_tweaks()?;

        if !config.cli_only {
            info!("Installing Graphics Card Tools for Laptop");
            system.install_graphic_card_laptop_tools().await?;
        }
    }

    if config.modelling && !config.cli_only {
        info!("Installing Blender");
        system.install_blender()?;
        info!("Installing Bambu Studio");
        system.install_bambu_studio()?;
        info!("Installing OpenSCAD");
        system.install_openscad()?;
    }

    if config.personal {
        info!("Installing GPG");
        system.install_gpg()?;
        info!("Setup NAS");
        system.setup_nas()?;

        if !config.cli_only {
            // info!("Installing Authy");
            // system.install_authy()?;
            info!("Installing Calibre");
            system.install_calibre()?;
            info!("Installing Disk Usage Analyser");
            system.install_disk_usage_analyser()?;
            info!("Installing Google Drive");
            system.install_google_drive()?;
            info!("Installing Gramps");
            system.install_gramps()?;
            info!("Installing Insync");
            system.install_insync()?;
            info!("Installing LaTeX");
            system.install_latex()?;
            // info!("Installing Nextcloud Client");
            // system.install_nextcloud_client()?;
            info!("Installing Office");
            system.install_office()?;
            info!("Installing OneDrive");
            system.install_onedrive()?;
            // info!("Installing Spotify");
            // system.install_spotify()?;
            info!("Installing Tauon Music Box");
            system.install_tauon_music_box().await?;
            info!("Installing SweetHome3D");
            system.install_sweet_home_3d()?;
            info!("Installing themes");
            system.install_themes().await?;
            info!("Installing WhatsApp");
            system.install_whatsapp()?;
        }
    }

    if config.printer {
        info!("Installing printer drivers");
        system.install_printer_drivers()?;
    }

    if config.recording && !config.cli_only {
        info!("Installing Audacity");
        system.install_audacity()?;
        info!("Installing OBS Studio");
        system.install_obs_studio()?;
    }

    if config.ripping && !config.cli_only {
        info!("Installing Exact Audio Copy");
        system.install_exact_audio_copy()?;
        info!("Installing Handbrake");
        system.install_handbrake()?;
        info!("Installing MakeMKV");
        system.install_makemkv()?;
        info!("Installing MKVToolNix");
        system.install_mkvtoolnix()?;
        info!("Installing Whipper");
        system.install_whipper()?
    }

    if config.video && !config.cli_only {
        info!("Installing Codecs");
        system.install_codecs().await?;
        info!("Installing VLC");
        system.install_vlc().await?;
    }

    if config.video_editing && !config.cli_only {
        info!("Installing DaVinci Resolve");
        system.install_davinci_resolve()?;
    }

    if config.vm && !config.cli_only {
        info!("Installing VM Tools");
        system.install_vm_tools()?;
    }

    if config.vpn && !config.cli_only {
        info!("Installing NordVPN");
        system.install_nordvpn().await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::system::MockSystem;

    use super::*;

    #[test]
    fn test_install_browsers() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: true,
            cli_only: false,
            debug: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            gnome: false,
            help: false,
            images: false,
            infrastructure: false,
            kde: false,
            laptop: false,
            modelling: false,
            personal: false,
            printer: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
            wsl: false,
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
            cli_only: false,
            debug: false,
            development: true,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            gnome: false,
            help: false,
            images: false,
            infrastructure: false,
            kde: false,
            laptop: false,
            modelling: false,
            personal: false,
            printer: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
            wsl: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_cplusplus()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_exercism()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_gradle()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_git()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_godot()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_groovy()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_android_studio()
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
            .expect_install_rust_rover()
            .times(1)
            .returning(|| Ok(()));
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
            .expect_install_development_extras()
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
            cli_only: false,
            debug: false,
            development: false,
            docker: true,
            dry_run: false,
            gaming: false,
            gcp: false,
            gnome: false,
            help: false,
            images: false,
            infrastructure: false,
            kde: false,
            laptop: false,
            modelling: false,
            personal: false,
            printer: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
            wsl: false,
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
            .returning(|| Box::pin(async { Ok(()) }));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_gaming() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            cli_only: false,
            debug: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: true,
            gcp: false,
            gnome: false,
            help: false,
            images: false,
            infrastructure: false,
            kde: false,
            laptop: false,
            modelling: false,
            personal: false,
            printer: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
            wsl: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_discord()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_epic_games()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_gog_galaxy()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_lutris()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_origin()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_retroarch()
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
        mock_system
            .expect_install_xbox_streaming()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_gcp() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            cli_only: false,
            debug: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: true,
            gnome: false,
            help: false,
            images: false,
            infrastructure: false,
            kde: false,
            laptop: false,
            modelling: false,
            personal: false,
            printer: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
            wsl: false,
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
            cli_only: false,
            debug: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            gnome: false,
            help: false,
            images: true,
            infrastructure: false,
            kde: false,
            laptop: false,
            modelling: false,
            personal: false,
            printer: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
            wsl: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_affinity_suite()
            .times(1)
            .returning(|| Ok(()));
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
    fn test_install_infrastructure() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            cli_only: false,
            debug: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            gnome: false,
            help: false,
            images: false,
            infrastructure: true,
            kde: false,
            laptop: false,
            modelling: false,
            personal: false,
            printer: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
            wsl: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_terraform()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_laptop() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            cli_only: false,
            debug: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            gnome: false,
            help: false,
            images: false,
            infrastructure: false,
            kde: false,
            laptop: true,
            modelling: false,
            personal: false,
            printer: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
            wsl: false,
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
            .returning(|| Box::pin(async { Ok(()) }));
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
            cli_only: false,
            debug: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            gnome: false,
            help: false,
            images: false,
            infrastructure: false,
            kde: false,
            laptop: false,
            modelling: true,
            personal: false,
            printer: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
            wsl: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_blender()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_bambu_studio()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_openscad()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_personal() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            cli_only: false,
            debug: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            gnome: false,
            help: false,
            images: false,
            infrastructure: false,
            kde: false,
            laptop: false,
            modelling: false,
            personal: true,
            printer: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
            wsl: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_authy()
            .times(0)
            .returning(|| Ok(()));
        mock_system
            .expect_install_calibre()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_disk_usage_analyser()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_google_drive()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_gramps()
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
            .times(0)
            .returning(|| Ok(()));
        mock_system
            .expect_install_office()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_onedrive()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_spotify()
            .times(0)
            .returning(|| Ok(()));
        mock_system
            .expect_install_tauon_music_box()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_sweet_home_3d()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_themes()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_whatsapp()
            .times(1)
            .returning(|| Ok(()));
        mock_system.expect_setup_nas().times(1).returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_printer() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            cli_only: false,
            debug: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            gnome: false,
            help: false,
            images: false,
            infrastructure: false,
            kde: false,
            laptop: false,
            modelling: false,
            personal: false,
            printer: true,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
            wsl: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_printer_drivers()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_recording() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            cli_only: false,
            debug: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            gnome: false,
            help: false,
            images: false,
            infrastructure: false,
            kde: false,
            laptop: false,
            modelling: false,
            personal: false,
            printer: false,
            recording: true,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
            wsl: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_audacity()
            .times(1)
            .returning(|| Ok(()));
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
            cli_only: false,
            debug: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            gnome: false,
            help: false,
            images: false,
            infrastructure: false,
            kde: false,
            laptop: false,
            modelling: false,
            personal: false,
            printer: false,
            recording: false,
            ripping: true,
            video: false,
            video_editing: false,
            vm: false,
            vpn: false,
            wsl: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_exact_audio_copy()
            .times(1)
            .returning(|| Ok(()));
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
        mock_system
            .expect_install_whipper()
            .times(1)
            .returning(|| Ok(()));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_video() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            cli_only: false,
            debug: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            gnome: false,
            help: false,
            images: false,
            infrastructure: false,
            kde: false,
            laptop: false,
            modelling: false,
            personal: false,
            printer: false,
            recording: false,
            ripping: false,
            video: true,
            video_editing: false,
            vm: false,
            vpn: false,
            wsl: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_codecs()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_vlc()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_video_editing() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: false,
            cli_only: false,
            debug: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            gnome: false,
            help: false,
            images: false,
            infrastructure: false,
            kde: false,
            laptop: false,
            modelling: false,
            personal: false,
            printer: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: true,
            vm: false,
            vpn: false,
            wsl: false,
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
            cli_only: false,
            debug: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            gnome: false,
            help: false,
            images: false,
            infrastructure: false,
            kde: false,
            laptop: false,
            modelling: false,
            personal: false,
            printer: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: true,
            vpn: false,
            wsl: false,
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
            cli_only: false,
            debug: false,
            development: false,
            docker: false,
            dry_run: false,
            gaming: false,
            gcp: false,
            gnome: false,
            help: false,
            images: false,
            infrastructure: false,
            kde: false,
            laptop: false,
            modelling: false,
            personal: false,
            printer: false,
            recording: false,
            ripping: false,
            video: false,
            video_editing: false,
            vm: false,
            vpn: true,
            wsl: false,
        };
        let mut mock_system = get_mock_system(&config);
        mock_system
            .expect_install_nordvpn()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    #[test]
    fn test_install_cli_only() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let config = Config {
            browsers: true,
            cli_only: true,
            debug: true,
            development: true,
            docker: true,
            dry_run: true,
            gaming: true,
            gcp: true,
            gnome: true,
            help: true,
            images: true,
            infrastructure: true,
            kde: true,
            laptop: true,
            modelling: true,
            personal: true,
            printer: true,
            recording: true,
            ripping: true,
            video: true,
            video_editing: true,
            vm: true,
            vpn: true,
            wsl: true,
        };
        let mut mock_system = MockSystem::new();
        mock_system
            .expect_setup_user_bin()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_system_extras()
            // .with(eq(config))
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system.expect_update_os().times(1).returning(|| Ok(()));
        mock_system.expect_install_window_manager().times(0);
        mock_system.expect_install_graphic_card_tools().times(0);
        mock_system.expect_install_archiver().times(0);
        mock_system
            .expect_install_bash()
            .times(1)
            .returning(|| Ok(()));
        mock_system.expect_install_cryptomator().times(0);
        mock_system
            .expect_install_curl()
            .times(1)
            .returning(|| Ok(()));
        mock_system.expect_install_keepassxc().times(0);
        mock_system.expect_install_quicklook().times(0);
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
        mock_system.expect_install_firefox().times(0);
        mock_system.expect_install_google_chrome().times(0);
        mock_system.expect_install_microsoft_edge().times(0);
        mock_system
            .expect_install_cplusplus()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_exercism()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_gradle()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_git()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_godot()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_groovy()
            .times(1)
            .returning(|| Ok(()));
        mock_system.expect_install_android_studio().times(0);
        mock_system.expect_install_intellij().times(0);
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
            .expect_install_printer_drivers()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_python()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_rust()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system.expect_install_rust_rover().times(0);
        mock_system.expect_install_slack().times(0);
        mock_system.expect_install_vscode().times(0);
        mock_system.expect_install_xcode().times(0);
        mock_system
            .expect_install_development_extras()
            .times(1)
            .returning(|| Ok(()));
        mock_system.expect_set_development_shortcuts().times(0);
        mock_system
            .expect_set_development_environment_settings()
            .times(1)
            .returning(|| Ok(()));
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
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system.expect_install_discord().times(0);
        mock_system.expect_install_epic_games().times(0);
        mock_system.expect_install_gog_galaxy().times(0);
        mock_system.expect_install_lutris().times(0);
        mock_system.expect_install_origin().times(0);
        mock_system.expect_install_retroarch().times(0);
        mock_system.expect_install_steam().times(0);
        mock_system.expect_install_wine().times(0);
        mock_system.expect_install_xbox_streaming().times(0);
        mock_system
            .expect_install_google_cloud_sdk()
            .times(1)
            .returning(|| Ok(()));
        mock_system.expect_install_affinity_suite().times(0);
        mock_system.expect_install_gimp().times(0);
        mock_system.expect_install_inkscape().times(0);
        mock_system
            .expect_install_terraform()
            .times(1)
            .returning(|| Ok(()));
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
            .times(0);
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
        mock_system.expect_install_authy().times(0);
        mock_system.expect_install_blender().times(0);
        mock_system.expect_install_bambu_studio().times(0);
        mock_system.expect_install_calibre().times(0);
        mock_system.expect_install_google_drive().times(0);
        mock_system
            .expect_install_gpg()
            .times(1)
            .returning(|| Ok(()));
        mock_system.expect_install_gramps().times(0);
        mock_system.expect_install_insync().times(0);
        mock_system.expect_install_latex().times(0);
        mock_system.expect_install_nextcloud_client().times(0);
        mock_system.expect_install_office().times(0);
        mock_system.expect_install_onedrive().times(0);
        mock_system.expect_install_openscad().times(0);
        mock_system.expect_install_spotify().times(0);
        mock_system.expect_install_tauon_music_box().times(0);
        mock_system.expect_install_sweet_home_3d().times(0);
        mock_system.expect_install_themes().times(0);
        mock_system.expect_install_whatsapp().times(0);
        mock_system.expect_setup_nas().times(1).returning(|| Ok(()));
        mock_system.expect_install_audacity().times(0);
        mock_system.expect_install_obs_studio().times(0);
        mock_system.expect_install_exact_audio_copy().times(0);
        mock_system.expect_install_handbrake().times(0);
        mock_system.expect_install_makemkv().times(0);
        mock_system.expect_install_mkvtoolnix().times(0);
        mock_system.expect_install_whipper().times(0);
        mock_system.expect_install_codecs().times(0);
        mock_system.expect_install_vlc().times(0);
        mock_system.expect_install_davinci_resolve().times(0);
        mock_system.expect_install_vm_tools().times(0);
        mock_system.expect_install_nordvpn().times(0);

        assert!(rt.block_on(install(&config, &mock_system)).is_ok());
    }

    fn get_mock_system(_config: &Config) -> MockSystem {
        let mut mock_system = MockSystem::new();
        mock_system
            .expect_setup_user_bin()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_system_extras()
            // .with(eq(config))
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system.expect_update_os().times(1).returning(|| Ok(()));
        mock_system
            .expect_install_window_manager()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_graphic_card_tools()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_archiver()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_bash()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_cryptomator()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));
        mock_system
            .expect_install_curl()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_keepassxc()
            .times(1)
            .returning(|| Ok(()));
        mock_system
            .expect_install_quicklook()
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
