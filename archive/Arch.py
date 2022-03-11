from typing import List, AnyStr

from Linux import Linux


class Arch(Linux):
    def __init__(self):
        super().__init__()

    def aur_install_application(self, application: AnyStr):
        self.aur_install_applications([application])

    def aur_install_applications(self, applications: List[AnyStr]):
        command = ['yay', '-S', '--noconfirm', '--needed'] + applications
        self.execute(command, super_user=False)

    def enable_service(self, service: AnyStr):
        self.execute(['systemctl', 'enable', service])

    def install_applications(self, applications: List[AnyStr]):
        command = ['pacman', '-S', '--noconfirm', '--needed']
        command.extend(applications)
        self.execute(command)

    def install_android_studio(self):
        self.aur_install_application('android-studio')

    def install_blender(self):
        self.install_application('blender')

    def install_bluetooth(self):
        self.install_applications(['bluez', 'bluez-utils'])
        self.enable_service('bluetooth')

    def install_codecs(self):
        self.install_applications(['libdvdread', 'libdvdcss', 'libdvdnav', 'libbluray', 'libaacs'])
        self.setup_codecs()

    def install_cryptomator(self):
        self.aur_install_application('cryptomator')

    def install_davinci_resolve(self):
        self.aur_install_application('davinci-resolve-studio')

    def install_discord(self):
        self.install_application('discord')

    def install_docker(self):
        self.install_application('docker')
        self.setup_docker()

    def install_dropbox(self):
        self.aur_install_applications(['dropbox', 'nautilus-dropbox'])

    def install_eclipse(self):
        self.aur_install_application('eclipse-jee')
        self.setup_eclipse()

    def install_firefox(self):
        self.install_application('firefox')

    def install_firmware_updater(self):
        self.install_application('fwupd')
        self.enable_service('fwupd')

    def install_flatpak(self):
        self.install_application('flatpak')

    def install_google_chrome(self):
        self.aur_install_application('google-chrome')

    def install_google_cloud_sdk(self):
        self.aur_install_application('google-cloud-sdk')

    def install_git(self):
        self.install_application('git')
        self.setup_git()

    def install_gimp(self):
        self.install_application('gimp')

    def install_gpg(self):
        self.install_applications(['seahorse', 'seahorse-nautilus'])

    def install_helm(self):
        self.install_application('helm')

    def install_inkscape(self):
        self.install_application('inkscape')

    def install_insync(self):
        self.aur_install_application('insync')

    def install_intellij(self):
        self.aur_install_application('intellij-idea-ultimate-edition')

    def install_jdk(self):
        self.install_application('jdk-openjdk')
        self.set_java_home('.zshrc.custom', '/usr/lib/jvm/default')
        self.set_java_home('.bashrc.custom', '/usr/lib/jvm/default')

    def install_keepassxc(self):
        self.install_application('keepassxc')

    def install_kubectl(self):
        self.install_application('kubectl')

    def install_lutris(self):
        self.install_application('lutris')

    def install_makemkv(self):
        self.aur_install_applications(['makemkv', 'ccextractor'])

    def install_microcode(self):
        cpu_name = ''
        with open('/proc/cpuinfo', 'r') as f:
            for line in f.readlines():
                if line.startswith('vendor_id'):
                    cpu_name = line.split(':')[1].strip()
        if cpu_name == 'GenuineIntel':
            self.install_application('intel-ucode')
        else:
            self.install_application('amd-ucode')

    def install_minikube(self):
        self.install_application('minikube')

    def install_mkvtoolnix(self):
        self.install_application('mkvtoolnix-gui')

    def install_nextcloud_client(self):
        self.install_application('nextcloud-client')

    def install_nodejs(self):
        self.aur_install_application('nvm')
        self.setup_nodejs()

    def install_nordvpn(self):
        self.aur_install_application('nordvpn-bin')
        self.enable_service('nordvpnd')

    def install_nvidia_tools(self):
        self.install_applications(
            ['nvidia', 'nvidia-utils', 'lib32-nvidia-utils', 'nvidia-settings', 'vulkan-icd-loader',
             'lib32-vulkan-icd-loader'])

    def install_nvidia_laptop_tools(self):
        self.install_application('nvidia-prime')

    def install_obs_studio(self):
        self.install_application('obs-studio')

    def install_slack(self):
        self.aur_install_application('slack-desktop')

    def install_spotify(self):
        self.aur_install_application('spotify')

    def install_steam(self):
        self.install_application('steam')

    def install_system_extras(self):
        self.install_applications(['base-devel', 'ttf-dejavu'])

        with open('/etc/pacman.conf', 'r') as f:
            lines = []
            enable_multilib = False
            for line in f.readlines():
                if line.startswith('#[multilib]'):
                    line = line.replace('#', '', 1)
                    enable_multilib = True
                if enable_multilib and line.startswith('#Include = /etc/pacman.d/mirrorlist'):
                    line = line.replace('#', '', 1)
                    enable_multilib = False
                lines.append(line)
        with open('/etc/pacman.conf', 'w') as f:
            f.writelines(lines)

        self.install_applications(['yay', 'wget'])

    def install_specific_themes(self):
        self.install_theme_cyberpunk_neon()
        self.install_theme_paper_icon()
        self.install_theme_suru_plus()

    def install_theme_paper_icon(self):
        self.aur_install_application('paper-icon-theme-git')

    def install_theme_suru_plus(self):
        self.aur_install_application('suru-plus-git')

    def install_tlp(self):
        super().install_tlp()
        self.enable_service('tlp')

    def install_tmux(self):
        self.install_applications(['tmux', 'xclip'])
        self.aur_install_application('tmux-bash-completion')
        self.setup_tmux()

    def install_vscode(self):
        self.install_application('code')

    def install_wifi(self):
        self.copy_file('/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin',
                       '/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin.bak')
        self.download_file(
            'https://github.com/kvalo/ath10k-firmware/raw/master/QCA6174/hw3.0/4.4.1.c3/firmware-6.bin_WLAN.RM.4.4.1.c3-00035',
            '/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin')

    def install_window_manager(self):
        self.install_applications(['gnome', 'libcanberra', 'libappindicator-gtk3'])
        self.aur_install_application('gnome-shell-extension-appindicator')
        self.enable_service('gdm')
        self.enable_service('NetworkManager')

    def install_zsh(self):
        self.install_applications(['zsh', 'zsh-completions'])
        self.setup_zsh()

    def reload_service_daemons(self):
        self.execute(['systemctl', 'daemon-reload'])

    def set_development_shortcuts(self):
        self.set_gnome_development_shortcuts()

    def setup_power_saving_tweaks(self):
        device_name = ''
        with open('/sys/devices/virtual/dmi/id/product_name', 'r') as f:
            for line in f.readlines():
                device_name = line

        if device_name == 'XPS 15 9570':
            with open('/sys/power/mem_sleep', 'w') as f:
                f.write('s2idle [deep]\n')

            with open('/etc/default/grub', 'r') as f:
                lines = []
                for line in f.readlines():
                    if line.startswith('GRUB_CMDLINE_LINUX_DEFAULT='):
                        split_line = line.split('=')
                        value = split_line[1].replace('"', '')
                        value += ' mem_sleep_default=deep'
                        line = '%s="%s"' % (split_line[0], value)
                    lines.append(line)
            with open('/etc/default/grub', 'w') as f:
                f.writelines(lines)

    def update_os(self):
        self.update_os_repo()
        self.execute(['pacman', '-Syu', '--noconfirm'])

    def update_os_repo(self):
        self.execute(['pacman', '-Sy'])