#[derive(Debug, PartialEq)]
pub(crate) struct Config {
    pub browsers: bool,
    pub cli_only: bool,
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
        assert_eq!(config.browsers, false);
        assert_eq!(config.cli_only, false);
        assert_eq!(config.development, false);
        assert_eq!(config.docker, false);
        assert_eq!(config.dry_run, false);
        assert_eq!(config.gaming, false);
        assert_eq!(config.gcp, false);
        assert_eq!(config.gnome, false);
        assert_eq!(config.help, false);
        assert_eq!(config.images, false);
        assert_eq!(config.infrastructure, false);
        assert_eq!(config.kde, false);
        assert_eq!(config.laptop, false);
        assert_eq!(config.modelling, false);
        assert_eq!(config.personal, false);
        assert_eq!(config.recording, false);
        assert_eq!(config.ripping, false);
        assert_eq!(config.video, false);
        assert_eq!(config.video_editing, false);
        assert_eq!(config.vm, false);
        assert_eq!(config.vpn, false);
        assert_eq!(config.wsl, true);
    }

    #[test]
    fn parse_sets_browser_to_true() {
        let config = parse(vec!["--browsers".to_string()]);
        assert_eq!(config.browsers, true);
    }

    #[test]
    fn parse_sets_cli_only_to_true() {
        let config = parse(vec!["--cli-only".to_string()]);
        assert_eq!(config.cli_only, true);
    }

    #[test]
    fn parse_sets_development_to_true() {
        let config = parse(vec!["--development".to_string()]);
        assert_eq!(config.development, true);
    }

    #[test]
    fn parse_sets_docker_to_true() {
        let config = parse(vec!["--docker".to_string()]);
        assert_eq!(config.docker, true);
    }

    #[test]
    fn parse_sets_dry_run_to_true() {
        let config = parse(vec!["--dry-run".to_string()]);
        assert_eq!(config.dry_run, true);
    }

    #[test]
    fn parse_sets_gaming_to_true() {
        let config = parse(vec!["--gaming".to_string()]);
        assert_eq!(config.gaming, true);
    }

    #[test]
    fn parse_sets_gcp_to_true() {
        let config = parse(vec!["--gcp".to_string()]);
        assert_eq!(config.gcp, true);
    }

    #[test]
    fn parse_sets_gnome_to_true() {
        let config = parse(vec!["--gnome".to_string()]);
        assert_eq!(config.gnome, true);
    }

    #[test]
    fn parse_sets_help_to_true() {
        let config = parse(vec!["--help".to_string()]);
        assert_eq!(config.help, true);
    }

    #[test]
    fn parse_sets_images_to_true() {
        let config = parse(vec!["--images".to_string()]);
        assert_eq!(config.images, true);
    }

    #[test]
    fn parse_sets_infrastructure_to_true() {
        let config = parse(vec!["--infrastructure".to_string()]);
        assert_eq!(config.infrastructure, true);
    }

    #[test]
    fn parse_sets_kde_to_true() {
        let config = parse(vec!["--kde".to_string()]);
        assert_eq!(config.kde, true);
    }

    #[test]
    fn parse_sets_laptop_to_true() {
        let config = parse(vec!["--laptop".to_string()]);
        assert_eq!(config.laptop, true);
    }

    #[test]
    fn parse_sets_modelling_to_true() {
        let config = parse(vec!["--modelling".to_string()]);
        assert_eq!(config.modelling, true);
    }

    #[test]
    fn parse_sets_personal_to_true() {
        let config = parse(vec!["--personal".to_string()]);
        assert_eq!(config.personal, true);
    }

    #[test]
    fn parse_sets_recording_to_true() {
        let config = parse(vec!["--recording".to_string()]);
        assert_eq!(config.recording, true);
    }

    #[test]
    fn parse_sets_ripping_to_true() {
        let config = parse(vec!["--ripping".to_string()]);
        assert_eq!(config.ripping, true);
    }

    #[test]
    fn parse_sets_video_to_true() {
        let config = parse(vec!["--video".to_string()]);
        assert_eq!(config.video, true);
    }

    #[test]
    fn parse_sets_video_editing_to_true() {
        let config = parse(vec!["--video-editing".to_string()]);
        assert_eq!(config.video_editing, true);
    }

    #[test]
    fn parse_sets_vm_to_true() {
        let config = parse(vec!["--vm".to_string()]);
        assert_eq!(config.vm, true);
    }

    #[test]
    fn parse_sets_vpn_to_true() {
        let config = parse(vec!["--vpn".to_string()]);
        assert_eq!(config.vpn, true);
    }

    #[test]
    fn parse_sets_wsl_to_false() {
        let config = parse(vec!["--not-wsl".to_string()]);
        assert_eq!(config.wsl, false);
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
            "--recording".to_string(),
            "--ripping".to_string(),
            "--video".to_string(),
            "--video-editing".to_string(),
            "--vm".to_string(),
            "--vpn".to_string(),
            "--not-wsl".to_string(),
        ]);
        assert_eq!(config.browsers, true);
        assert_eq!(config.cli_only, true);
        assert_eq!(config.development, true);
        assert_eq!(config.docker, true);
        assert_eq!(config.dry_run, true);
        assert_eq!(config.gaming, true);
        assert_eq!(config.gcp, true);
        assert_eq!(config.help, true);
        assert_eq!(config.images, true);
        assert_eq!(config.laptop, true);
        assert_eq!(config.modelling, true);
        assert_eq!(config.personal, true);
        assert_eq!(config.recording, true);
        assert_eq!(config.ripping, true);
        assert_eq!(config.video, true);
        assert_eq!(config.video_editing, true);
        assert_eq!(config.vm, true);
        assert_eq!(config.vpn, true);
        assert_eq!(config.wsl, false);
    }
}
