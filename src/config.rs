#[derive(Debug, PartialEq)]
pub(crate) struct Config {
    pub browsers: bool,
    pub cli_only: bool,
    pub debug: bool,
    pub development: bool,
    pub docker: bool,
    pub dry_run: bool,
    pub gaming: bool,
    pub gcp: bool,
    pub gnome: bool,
    pub infrastructure: bool,
    pub images: bool,
    pub kde: bool,
    pub help: bool,
    pub laptop: bool,
    pub modelling: bool,
    pub personal: bool,
    pub printer: bool,
    pub recording: bool,
    pub ripping: bool,
    pub video: bool,
    pub video_editing: bool,
    pub vm: bool,
    pub vpn: bool,
    pub wsl: bool,
}

pub(crate) fn parse(args: Vec<String>) -> Config {
    Config {
        browsers: args.contains(&"--browsers".to_string()),
        cli_only: args.contains(&"--cli-only".to_string()),
        debug: args.contains(&"--debug".to_string()),
        development: args.contains(&"--development".to_string()),
        docker: args.contains(&"--docker".to_string()),
        dry_run: args.contains(&"--dry-run".to_string()),
        gaming: args.contains(&"--gaming".to_string()),
        gcp: args.contains(&"--gcp".to_string()),
        gnome: args.contains(&"--gnome".to_string()),
        help: args.contains(&"--help".to_string()),
        images: args.contains(&"--images".to_string()),
        infrastructure: args.contains(&"--infrastructure".to_string()),
        kde: args.contains(&"--kde".to_string()),
        laptop: args.contains(&"--laptop".to_string()),
        modelling: args.contains(&"--modelling".to_string()),
        personal: args.contains(&"--personal".to_string()),
        printer: args.contains(&"--printer".to_string()),
        recording: args.contains(&"--recording".to_string()),
        ripping: args.contains(&"--ripping".to_string()),
        video: args.contains(&"--video".to_string()),
        video_editing: args.contains(&"--video-editing".to_string()),
        vm: args.contains(&"--vm".to_string()),
        vpn: args.contains(&"--vpn".to_string()),
        wsl: !args.contains(&"--not-wsl".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sets_correctly_for_empty_args() {
        let config = parse(vec![]);
        assert!(!config.browsers);
        assert!(!config.cli_only);
        assert!(!config.development);
        assert!(!config.docker);
        assert!(!config.dry_run);
        assert!(!config.gaming);
        assert!(!config.gcp);
        assert!(!config.gnome);
        assert!(!config.help);
        assert!(!config.images);
        assert!(!config.infrastructure);
        assert!(!config.kde);
        assert!(!config.laptop);
        assert!(!config.modelling);
        assert!(!config.personal);
        assert!(!config.recording);
        assert!(!config.ripping);
        assert!(!config.video);
        assert!(!config.video_editing);
        assert!(!config.vm);
        assert!(!config.vpn);
        assert!(config.wsl);
    }

    #[test]
    fn parse_sets_browser_to_true() {
        let config = parse(vec!["--browsers".to_string()]);
        assert!(config.browsers);
    }

    #[test]
    fn parse_sets_cli_only_to_true() {
        let config = parse(vec!["--cli-only".to_string()]);
        assert!(config.cli_only);
    }

    #[test]
    fn parse_sets_development_to_true() {
        let config = parse(vec!["--development".to_string()]);
        assert!(config.development);
    }

    #[test]
    fn parse_sets_docker_to_true() {
        let config = parse(vec!["--docker".to_string()]);
        assert!(config.docker);
    }

    #[test]
    fn parse_sets_dry_run_to_true() {
        let config = parse(vec!["--dry-run".to_string()]);
        assert!(config.dry_run);
    }

    #[test]
    fn parse_sets_gaming_to_true() {
        let config = parse(vec!["--gaming".to_string()]);
        assert!(config.gaming);
    }

    #[test]
    fn parse_sets_gcp_to_true() {
        let config = parse(vec!["--gcp".to_string()]);
        assert!(config.gcp);
    }

    #[test]
    fn parse_sets_gnome_to_true() {
        let config = parse(vec!["--gnome".to_string()]);
        assert!(config.gnome);
    }

    #[test]
    fn parse_sets_help_to_true() {
        let config = parse(vec!["--help".to_string()]);
        assert!(config.help);
    }

    #[test]
    fn parse_sets_images_to_true() {
        let config = parse(vec!["--images".to_string()]);
        assert!(config.images);
    }

    #[test]
    fn parse_sets_infrastructure_to_true() {
        let config = parse(vec!["--infrastructure".to_string()]);
        assert!(config.infrastructure);
    }

    #[test]
    fn parse_sets_kde_to_true() {
        let config = parse(vec!["--kde".to_string()]);
        assert!(config.kde);
    }

    #[test]
    fn parse_sets_laptop_to_true() {
        let config = parse(vec!["--laptop".to_string()]);
        assert!(config.laptop);
    }

    #[test]
    fn parse_sets_modelling_to_true() {
        let config = parse(vec!["--modelling".to_string()]);
        assert!(config.modelling);
    }

    #[test]
    fn parse_sets_personal_to_true() {
        let config = parse(vec!["--personal".to_string()]);
        assert!(config.personal);
    }

    #[test]
    fn parse_sets_recording_to_true() {
        let config = parse(vec!["--recording".to_string()]);
        assert!(config.recording);
    }

    #[test]
    fn parse_sets_ripping_to_true() {
        let config = parse(vec!["--ripping".to_string()]);
        assert!(config.ripping);
    }

    #[test]
    fn parse_sets_video_to_true() {
        let config = parse(vec!["--video".to_string()]);
        assert!(config.video);
    }

    #[test]
    fn parse_sets_video_editing_to_true() {
        let config = parse(vec!["--video-editing".to_string()]);
        assert!(config.video_editing);
    }

    #[test]
    fn parse_sets_vm_to_true() {
        let config = parse(vec!["--vm".to_string()]);
        assert!(config.vm);
    }

    #[test]
    fn parse_sets_vpn_to_true() {
        let config = parse(vec!["--vpn".to_string()]);
        assert!(config.vpn);
    }

    #[test]
    fn parse_sets_wsl_to_false() {
        let config = parse(vec!["--not-wsl".to_string()]);
        assert!(!config.wsl);
    }

    #[test]
    fn parse_sets_correctly_for_all_args() {
        let config = parse(vec![
            "--browsers".to_string(),
            "--cli-only".to_string(),
            "--development".to_string(),
            "--docker".to_string(),
            "--dry-run".to_string(),
            "--gaming".to_string(),
            "--gcp".to_string(),
            "--help".to_string(),
            "--images".to_string(),
            "--laptop".to_string(),
            "--modelling".to_string(),
            "--personal".to_string(),
            "--printer".to_string(),
            "--recording".to_string(),
            "--ripping".to_string(),
            "--video".to_string(),
            "--video-editing".to_string(),
            "--vm".to_string(),
            "--vpn".to_string(),
            "--not-wsl".to_string(),
        ]);
        assert!(config.browsers);
        assert!(config.cli_only);
        assert!(config.development);
        assert!(config.docker);
        assert!(config.dry_run);
        assert!(config.gaming);
        assert!(config.gcp);
        assert!(config.help);
        assert!(config.images);
        assert!(config.laptop);
        assert!(config.modelling);
        assert!(config.personal);
        assert!(config.printer);
        assert!(config.recording);
        assert!(config.ripping);
        assert!(config.video);
        assert!(config.video_editing);
        assert!(config.vm);
        assert!(config.vpn);
        assert!(!config.wsl);
    }
}
