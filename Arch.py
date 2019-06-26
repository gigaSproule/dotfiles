from typing import List, AnyStr

from Linux import Linux


class Arch(Linux):
    def __init__(self):
        super().__init__()

    def aur_install_application(self, application: AnyStr):
        self.aur_install_applications([application])

    def aur_install_applications(self, applications: List[AnyStr]):
        for application in applications:
            output_file = '%s.tar.gz' % application
            self.download_file('https://aur.archlinux.org/cgit/aur.git/snapshot/%s.tar.gz' % application,
                               output_file)
            self.execute(['tar', '-xvf', output_file])
            self.execute(['makepkg', '-Acsi', '--noconfirm'], application)
            self.delete_file(output_file)
            self.delete_file(application)

    def enable_service(self, service: AnyStr):
        self.execute(['systemctl', 'enable', service], super_user=True)

    def install_applications(self, applications: List[AnyStr]):
        command = ['pacman', '-Sy', '--noconfirm', '--needed']
        command.extend(applications)
        self.execute(command, super_user=True)

    def install_bluetooth(self):
        self.install_applications(['bluez', 'bluez-utils'])
        self.enable_service('bluetooth')

    def install_chrome(self):
        self.aur_install_application('google-chrome')

    def install_codecs(self):
        self.install_applications(['libdvdread', 'libdvdcss', 'libdvdnav', 'libbluray', 'libaacs'])
        super().setup_codecs()

    def install_curl(self):
        self.install_application('curl')

    def install_docker(self):
        self.install_application('docker')
        super().setup_docker()

    def install_dropbox(self):
        self.aur_install_applications(['dropbox', 'nautilus-dropbox'])

    def install_eclipse(self):
        self.install_application('eclipse-jee')
        self.setup_eclipse()

    def install_firefox(self):
        self.install_application('firefox')

    def install_firmware_updater(self):
        self.install_application('fwupd')
        self.enable_service('fwupd')

    def install_flatpak(self):
        self.install_application('flatpak')

    def install_git(self):
        self.install_application('git')
        super().setup_git()

    def install_gpg(self):
        self.install_applications(['seahorse', 'seahorse-nautilus'])

    def install_graphic_card_tools(self):
        # if nvidia
        self.install_applications(
            ['nvidia', 'nvidia-utils', 'lib32-nvidia-utils', 'nvidia-settings', 'vulkan-icd-loader',
             'lib32-vulkan-icd-loader'])
        # else

    def install_graphic_card_tools_laptop(self):
        # if nvidia
        self.install_application('bumblebee')
        self.enable_service('bumblebeed')

        lines = []
        with open('/etc/bumblebee/bumblebee.conf', 'r') as f:
            driver_nvidia = False
            for line in f.readlines():
                if line.startswith('[driver-nvidia]'):
                    driver_nvidia = True
                elif driver_nvidia and line.startswith('PMMethod='):
                    split = line.split('=')
                    line = split[0] + '=none'
                    driver_nvidia = False
                lines.append(line)

        with open('/etc/bumblebee/bumblebee.conf', 'w') as f:
            f.writelines(lines)

        with open('/etc/X11/xorg.conf.d/01-noautogpu.conf', 'w') as f:
            f.writelines(['Section "ServerFlags"',
                          'Option "AutoAddGPU" "off"',
                          'EndSection'])

        with open('/etc/modprobe.d/disable-ipmi.conf', 'w') as f:
            f.writelines(['install ipmi_msghandler /usr/bin/false',
                          'install ipmi_devintf /usr/bin/false'])

        with open('/etc/modprobe.d/disable-nvidia.conf', 'w') as f:
            f.write('install nvidia /bin/false')

        with open('/etc/modprobe.d/blacklist.conf', 'w') as f:
            f.writelines(
                ['blacklist nouveau',
                 'blacklist rivafb',
                 'blacklist nvidiafb',
                 'blacklist rivatv',
                 'blacklist nv',
                 'blacklist nvidia',
                 'blacklist nvidia-drm',
                 'blacklist nvidia-modeset',
                 'blacklist nvidia-uvm',
                 'blacklist ipmi_msghandler',
                 'blacklist ipmi_devintf'])

        self.copy_config('laptop/enablegpu', 'bin/enablegpu')
        self.copy_config('laptop/disablegpu', 'bin/disablegpu')

        with open('/etc/systemd/system/disable-nvidia-on-shutdown.service', 'w') as f:
            f.writelines(['[Unit]',
                          'Description=Disables Nvidia GPU on OS shutdown',
                          '',
                          '[Service]',
                          'Type=oneshot',
                          'RemainAfterExit=true',
                          'ExecStart=/bin/true',
                          'ExecStop=/bin/bash -c "mv /etc/modprobe.d/disable-nvidia.conf.disable /etc/modprobe.d/disable-nvidia.conf || true"',
                          '',
                          '[Install]',
                          'WantedBy=multi-user.target'])
        self.reload_service_daemons()
        self.enable_service('disable-nvidia-on-shutdown')
        with open('/etc/tmpfiles.d/nvidia_pm.conf', 'w') as f:
            f.write('w /sys/bus/pci/devices/0000:01:00.0/power/control - - - - auto')
        # else

    def install_jdk(self):
        self.install_application('jdk-openjdk')

        self.set_java_home('.zshrc', '/usr/lib/jvm/java-12-jdk')
        self.set_java_home('.bashrc', '/usr/lib/jvm/java-12-jdk')

    def install_keepassxc(self):
        self.install_application('keepassxc')

    def install_makemkv(self):
        self.aur_install_application('makemkv')

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

    def install_mkvtoolnix(self):
        self.install_application('mkvtoolnix-gui')

    def install_lutris(self):
        self.install_application('lutris')

    def install_nextcloud_client(self):
        self.install_application('nextcloud-client')

    def install_nodejs(self):
        self.install_applications(['npm', 'nodejs'])

    def install_nordvpn(self):
        self.aur_install_application('nordvpn-bin')
        self.enable_service('nordvpnd')

    def install_steam(self):
        self.install_application('steam')

    def install_system_extras(self):
        self.install_flatpak()
        self.install_applications(['base-devel'])

        def edit_file():
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
            lines.extend(['[archlinuxfr]\n', 'SigLevel = Never\n', 'Server = http://repo.archlinux.fr/\$arch'])
            with open('/etc/pacman.conf', 'w') as f:
                f.writelines(lines)

        self.run_as_super_user(edit_file)
        self.execute(['pacman', '-Sy', '--noconfirm', 'yaourt', 'firefox', 'wget'], super_user=True)

    def install_tlp(self):
        super().install_tlp()
        self.enable_service('tlp')

    def install_tmux(self):
        self.install_application('tmux')
        self.aur_install_application('tmux-bash-completion')
        super().setup_tmux()

    def install_vscode(self):
        self.install_application('code')

    def install_wifi(self):
        self.copy_file('/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin',
                       '/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin.bak', super_user=True)
        self.download_file(
            'https://github.com/kvalo/ath10k-firmware/raw/master/QCA6174/hw3.0/4.4.1.c3/firmware-6.bin_WLAN.RM.4.4.1.c3-00035',
            '/lib/firmware/ath10k/QCA6174/hw3.0/firmware-6.bin', super_user=True)

    def install_window_manager(self):
        self.install_applications(['gnome', 'libcanberra'])
        self.enable_service('gdm')
        self.enable_service('NetworkManager')

    def install_zsh(self):
        self.install_applications(['zsh', 'zsh-completions'])
        super().setup_zsh()

    def reload_service_daemons(self):
        self.execute(['systemctl', 'daemon-reload'], super_user=True)

    def set_development_shortcuts(self):
        self.execute(['gsettings', 'set', 'org.gnome.desktop.wm.keybindings' 'switch-to-workspace-up' '[]'])
        self.execute(['gsettings', 'set', 'org.gnome.desktop.wm.keybindings' 'switch-to-workspace-down' '[]'])
        self.execute(['gsettings', 'set', 'org.gnome.desktop.wm.keybindings' 'switch-to-workspace-left' '[]'])
        self.execute(['gsettings', 'set', 'org.gnome.desktop.wm.keybindings' 'switch-to-workspace-right' '[]'])
        self.execute(['gsettings', 'set', 'org.gnome.desktop.wm.keybindings' 'begin-move' '[]'])

    def setup_power_saving_tweaks(self):
        # if contents of /sys/devices/virtual/dmi/id/product_name is "XPS 15 9570"
        device_name = ''
        with open('/sys/devices/virtual/dmi/id/product_name', 'r') as f:
            for line in f.readlines():
                device_name = line

        if device_name == 'XPS 15 9570':
            # Set contents of /sys/power/mem_sleep to "s2idle [deep]"
            with open('/sys/power/mem_sleep', 'w') as f:
                f.write('s2idle [deep]')

            # Set GRUB_CMDLINE_LINUX_DEFAULT="mem_sleep_default=deep" in /etc/default/grub
            with open('/etc/default/grub', 'r') as f:
                lines = []
                for line in f.readlines():
                    if line.startswith('#Include = /etc/pacman.d/mirrorlist'):
                        line = line.replace('#', '', 1)
                    lines.append(line)
            with open('/etc/default/grub', 'w') as f:
                f.writelines(lines)

    def update_os(self):
        self.update_os_repo()
        self.execute(['pacman' '-Syu', '--noconfirm'], super_user=True)

    def update_os_repo(self):
        self.execute(['pacman' '-S'], super_user=True)
