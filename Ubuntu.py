import uuid
from typing import AnyStr, List

import distro

from Linux import Linux


class Ubuntu(Linux):
    def __init__(self):
        super().__init__()

    def add_apt_key(self, url):
        apt_file = '%s.apt' % uuid.uuid4()
        with open(apt_file, 'w') as f:
            f.write(self.execute(['curl', '-fsSL', url])['output'])
        self.execute(['apt-key', 'add', apt_file], super_user=True)
        self.delete_file(apt_file)

    def add_apt_repo(self, file_name, urls):
        with open('/etc/apt/sources.list.d/%s.list' % file_name, 'w') as f:
            for url in urls:
                f.write(url)

    def add_ppa(self, ppa):
        self.execute(['add-apt-repository', '-y', 'ppa:%s' % ppa], super_user=True)

    def install_applications(self, applications: List[AnyStr]):
        command = ['apt-get', 'install', '-y']
        command.extend(applications)
        self.execute(command, super_user=True)

    def install_chrome(self):
        self.download_file('https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb',
                           'google-chrome.deb')
        self.execute(['dpkg', '-i', 'google-chrome-stable_current_amd64.deb'], super_user=True)
        self.delete_file('google-chrome.deb')

    def install_codecs(self):
        self.install_applications(['libdvd-pkg', 'libaacs0', 'libbluray-bdj', 'libbluray1'])
        super().setup_codecs()

    def install_docker(self):
        self.add_apt_key('https://download.docker.com/linux/ubuntu/gpg')
        self.add_apt_repo('docker', [
            'deb [arch=amd64] https://download.docker.com/linux/ubuntu %s stable'
            % distro.lsb_release_info()['codename']
        ])
        self.update_os_repo()
        self.install_application('docker-ce')
        super().setup_docker()

    def install_dropbox(self):
        self.install_application('nautilus-dropbox')

    def install_eclipse(self):
        self.flatpak_install_application('eclipse')
        self.setup_eclipse()

    def install_flatpak(self):
        self.install_application('flatpak')

    def install_git(self):
        self.install_application('git')
        super().setup_git()

    def install_gpg(self):
        self.install_application('seahorse-nautilus')

    def install_jdk(self):
        self.add_ppa('webupd8team/java')
        self.update_os_repo()
        self.set_debconf('oracle-java8-installer', 'shared/accepted-oracle-license-v1-1')
        self.install_applications(
            ['oracle-java8-installer', 'oracle-java8-unlimited-jce-policy', 'oracle-java8-set-default'])

        self.set_java_home('.zshrc', '/usr/lib/jvm/java-8-oracle')
        self.set_java_home('.bashrc', '/usr/lib/jvm/java-8-oracle')

    def install_keepassxc(self):
        self.add_apt_repo('keepassxc', [
            'deb http://ppa.launchpad.net/phoerious/keepassxc/ubuntu %s main' % distro.lsb_release_info()['codename']])
        self.install_application('keepassxc')

    def install_lutris(self):
        self.add_apt_repo('lutris', [
            'deb http://download.opensuse.org/repositories/home:/strycore/xUbuntu_%s/ ./' % distro.version()
        ])
        self.add_apt_key(
            'http://download.opensuse.org/repositories/home:/strycore/xUbuntu_%s/Release.key' % distro.version())
        self.update_os_repo()
        self.install_application('lutris')

    def install_makemkv(self):
        self.install_applications([
            'build-essential', 'pkg-config', 'libc6-dev', 'libssl-dev', 'libexpat1-dev', 'libavcodec-dev',
            'libgl1-mesa-dev', 'libqt4-dev'
        ])

        makemkv_version = '1.10.10'

        self.download_file('http://www.makemkv.com/download/makemkv-oss-%s.tar.gz' % makemkv_version,
                           'makemkv-oss.tar.gz')
        self.untar_rename_root('makemkv-oss.tar.gz', 'makemkv-oss')
        self.execute('./configure', 'makemkv-oss')
        self.execute('make', 'makemkv-oss')
        self.execute('make install', 'makemkv-oss')
        self.delete_file('makemkv-oss')

        self.download_file('http://www.makemkv.com/download/makemkv-bin-%s.tar.gz' % makemkv_version,
                           'makemkv-bin.tar.gz')
        self.untar_rename_root('makemkv-bin.tar.gz', 'makemkv-bin')
        self.execute('make', 'makemkv-bin')
        self.execute('make install', 'makemkv-bin')
        self.delete_file('makemkv-bin')

    def install_microcode(self):
        # if cat /proc/cpuinfo | grep 'vendor' | uniq == "GenuineIntel":
        self.install_application('intel-microcode')
        # else:
        # self.install_application('amd-microcode')

    def install_nodejs(self):
        self.execute(['curl', '-sL', 'https://deb.nodesource.com/setup_8.x', '|', '-E', 'bash', '-'])
        self.update_os_repo()
        self.install_applications(['npm', 'nodejs'])

    def install_nordvpn(self):
        self.download_file('https://repo.nordvpn.com/deb/nordvpn/debian/pool/main/nordvpn-release_1.0.0_all.deb',
                           'nordvpn.deb')
        self.install_application('./nordvpn.deb')
        self.update_os_repo()
        self.install_application('nordvpn')

    def install_steam(self):
        self.install_application('steam-installer')

    def install_system_extras(self):
        self.set_debconf('ttf-mscorefonts-installer', 'msttcorefonts/accepted-mscorefonts-eula')
        self.install_applications(['ubuntu-restricted-extras', 'chrome-gnome-shell', 'gnome-tweaks'])

    def install_tmux(self):
        self.install_application('tmux')
        super().setup_tmux()

    def install_vm_tools(self):
        self.install_applications(['open-vm-tools', 'open-vm-tools-desktop'])

    def install_zsh(self):
        self.install_application('zsh')
        super().setup_zsh()

    def set_debconf(self, installer, conf, value='true'):
        debconf_file = '%s.debconf' % uuid.uuid4()
        with open(debconf_file, 'w') as f:
            f.write('%s %s select %s\n' % (installer, conf, value))
            f.write('%s %s seen %s\n' % (installer, conf, value))
        self.execute(['debconf-set-selections', debconf_file])
        self.delete_file(debconf_file)

    def set_development_shortcuts(self):
        self.execute(['gsettings', 'set', 'org.gnome.desktop.wm.keybindings' 'switch-to-workspace-up' '[]'])
        self.execute(['gsettings', 'set', 'org.gnome.desktop.wm.keybindings' 'switch-to-workspace-down' '[]'])
        self.execute(['gsettings', 'set', 'org.gnome.desktop.wm.keybindings' 'switch-to-workspace-left' '[]'])
        self.execute(['gsettings', 'set', 'org.gnome.desktop.wm.keybindings' 'switch-to-workspace-right' '[]'])
        self.execute(['gsettings', 'set', 'org.gnome.desktop.wm.keybindings' 'begin-move' '[]'])

    def update_os(self):
        self.update_os_repo()
        self.execute(['apt-get', '-y', 'full-upgrade'], super_user=True)

    def update_os_repo(self):
        self.execute(['apt-get', 'update'], super_user=True)
