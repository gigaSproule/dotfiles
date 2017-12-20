import os

from LinuxCommands import LinuxCommands
from LinuxCommands import execute


class Arch(LinuxCommands):
    def __init__(self):
        super().__init__()

    def install_applications(self, applications):
        command = ['yaourt', '-Sy', '--noconfirm', '--needed']
        command.extend(applications)
        execute(command)

    def install_chromium(self):
        self.install_application('chromium')

    def install_codecs(self):
        self.install_applications(['libdvdread', 'libdvdcss', 'libdvdnav', 'libbluray', 'libaacs'])
        super().setup_codecs()

    def install_curl(self):
        self.install_application('curl')

    def install_deb(self):
        print('TODO')

    def install_distro_extras(self):
        with open('/etc/pacman.conf', 'r') as f:
            lines = []
            for line in f.readlines('/etc/pacman.conf'):
                if line.startswith('#Include = /etc/pacman.d/mirrorlist'):
                    line = line.replace('#', '', 1)
                lines.append(line)
        lines.extend(['[archlinuxfr]\n', 'SigLevel = Never\n', 'Server = http://repo.archlinux.fr/\$arch'])
        with open('/etc/pacman.conf', 'w') as f:
            f.writelines(lines)
        execute(['pacman', '-Sy', '--noconfirm', 'yaourt', 'firefox', 'wget'])

    def install_docker(self):
        self.install_application('docker')
        super().setup_docker()

    def install_dropbox(self):
        self.install_applications(['aur/dropbox', 'aur/nautilus-dropbox'])

    def install_eclipse(self):
        self.install_application('eclipse-jee')

    def install_git(self):
        self.install_application('git')
        super().setup_git()

    def install_intellij(self):
        self.install_application('aur/intellij-idea-ultimate-edition')

    def install_jdk(self):
        self.install_application('aur/jdk')
        execute(['archlinux-java', 'set', 'java-8-jdk'])

        self.set_java_home('.zshrc', '/usr/lib/jvm/java-8-jdk')
        self.set_java_home('.bashrc', '/usr/lib/jvm/java-8-jdk')

    def install_jq(self):
        self.install_application('aur/jq-git')

    def install_keepassxc(self):
        self.install_application('keepassxc')

    def install_mcollective(self):
        print('TODO')

    def install_lutris(self):
        self.install_application('aur/lutris')

    def install_nextcloud_client(self):
        self.install_application('aur/nextcloud-client')

    def install_nodejs(self):
        self.install_applications(['npm', 'nodejs'])

    def install_openvpn(self):
        self.install_applications(['openvpn', 'networkmanager-openvpn'])
        super().setup_openvpn()

    def install_steam(self):
        self.install_application('steam')

    def install_rpm(self):
        print('TODO')

    def install_terraform(self):
        self.install_application('aur/terraform')

    def install_tmux(self):
        self.install_application('tmux')
        self.install_application('aur/tmux-bash-completion')

    def install_zsh(self):
        self.install_applications(['zsh', 'zsh-completions'])
        super().setup_zsh()

    def set_development_shortcuts(self):
        # Allow for alt dragging the cursor (rather than the window)
        execute(['dconf', 'write', '/org/gnome/desktop/wm/preferences/mouse-button-modifier',
                 '\'"<Shift><Control><Alt><Super>Button20"\''])
        print('Remove keyboard shortcuts under Navigation for ctrl + alt + left/right')
        print('Remove keyboard shortcut under System for ctrl + alt + l')
        print('Remove keyboard shortcuts under Windows for ctrl + alt + s, alt + f7')

    def update_os(self):
        self.update_os_repo()
        execute(['pacman' '-Syu', '--noconfirm'])

    def update_os_repo(self):
        execute(['pacman' '-S'])
