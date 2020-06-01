import uuid
from typing import AnyStr, List

import distro

from Linux import Linux


class Ubuntu(Linux):
    def __init__(self):
        super().__init__()

    def add_apt_key(self, url: AnyStr):
        apt_file = '%s.apt' % uuid.uuid4()
        with open(apt_file, 'w') as f:
            f.write(self.execute(['curl', '-fsSL', url])['output'])
        self.execute(['apt-key', 'add', apt_file])
        self.delete_file(apt_file)

    def add_apt_repo(self, file_name: AnyStr, urls: List[AnyStr]):
        with open('/etc/apt/sources.list.d/%s.list' % file_name, 'w') as f:
            for url in urls:
                f.write(url)

    def add_ppa(self, ppa):
        self.execute(['add-apt-repository', '-y', 'ppa:%s' % ppa])

    def install_applications(self, applications: List[AnyStr]):
        command = ['apt-get', 'install', '-y']
        command.extend(applications)
        self.execute(command)

    def install_android_studio(self):
        self.add_ppa('maarten-fonville/android-studio')
        self.update_os_repo()
        self.install_application('android-studio')

    def install_chrome(self):
        self.download_file('https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb',
                           'google-chrome.deb')
        self.execute(['dpkg', '-i', 'google-chrome-stable_current_amd64.deb'])
        self.delete_file('google-chrome.deb')

    def install_codecs(self):
        self.install_applications(['libdvd-pkg', 'libaacs0', 'libbluray-bdj', 'libbluray1'])
        self.setup_codecs()

    def install_docker(self):
        self.add_apt_key('https://download.docker.com/linux/ubuntu/gpg')
        self.add_apt_repo('docker', [
            'deb [arch=amd64] https://download.docker.com/linux/ubuntu %s stable'
            % distro.lsb_release_info()['codename']
        ])
        self.update_os_repo()
        self.install_application('docker-ce')
        self.setup_docker()

    def install_dropbox(self):
        self.install_application('nautilus-dropbox')

    def install_eclipse(self):
        self.flatpak_install_application('eclipse')
        self.setup_eclipse()

    def install_flatpak(self):
        self.install_application('flatpak')

    def install_google_cloud_sdk(self):
        self.add_apt_key('https://packages.cloud.google.com/apt/doc/apt-key.gpg')
        self.add_apt_repo('google-cloud-sdk', 'deb https://packages.cloud.google.com/apt cloud-sdk main')
        self.install_application('google-cloud-sdk')

    def install_git(self):
        self.install_application('git')
        self.setup_git()

    def install_gpg(self):
        self.install_application('seahorse-nautilus')

    def install_jdk(self):
        self.add_ppa('webupd8team/java')
        self.update_os_repo()
        self.set_debconf('oracle-java8-installer', 'shared/accepted-oracle-license-v1-1')
        self.install_applications(
            ['oracle-java8-installer', 'oracle-java8-unlimited-jce-policy', 'oracle-java8-set-default'])
        self.set_java_home('.zshrc.custom', '/usr/lib/jvm/java-8-oracle')
        self.set_java_home('.bashrc.custom', '/usr/lib/jvm/java-8-oracle')

    def install_keepassxc(self):
        self.add_apt_repo('keepassxc', [
            'deb http://ppa.launchpad.net/phoerious/keepassxc/ubuntu %s main' % distro.lsb_release_info()['codename']])
        self.install_application('keepassxc')

    def install_lutris(self):
        self.add_apt_key(
            'http://download.opensuse.org/repositories/home:/strycore/xUbuntu_%s/Release.key' % distro.version())
        self.add_apt_repo('lutris', [
            'deb http://download.opensuse.org/repositories/home:/strycore/xUbuntu_%s/ ./' % distro.version()
        ])
        self.update_os_repo()
        self.install_application('lutris')

    def install_makemkv(self):
        self.add_ppa('heyarje/makemkv-beta')
        self.update_os_repo()
        self.install_applications(['makemkv-bin', 'makemkv-oss'])
        self.install_application('ccextractor')

    def install_microcode(self):
        # if cat /proc/cpuinfo | grep 'vendor' | uniq == "GenuineIntel":
        self.install_application('intel-microcode')
        # else:
        # self.install_application('amd-microcode')

    def install_mkvtoolnix(self):
        self.install_application('mkvtoolnix-gui')

    def install_nextcloud_client(self):
        self.install_application('nextcloud-desktop')

    def install_nodejs(self):
        self.execute(['curl', '-sL', 'https://deb.nodesource.com/setup_14.x', '|', '-E', 'bash', '-'])
        self.add_apt_key('https://dl.yarnpkg.com/debian/pubkey.gpg')
        self.add_apt_repo('yarn.list', ['https://dl.yarnpkg.com/debian/ stable main'])
        self.update_os_repo()
        self.install_applications(['nodejs', 'npm', 'yarn'])

    def install_nordvpn(self):
        self.download_file('https://repo.nordvpn.com/deb/nordvpn/debian/pool/main/nordvpn-release_1.0.0_all.deb',
                           'nordvpn.deb')
        self.install_application('./nordvpn.deb')
        self.update_os_repo()
        self.install_application('nordvpn')

    def install_obs_studio(self):
        self.add_ppa('obsproject/obs-studio')
        self.update_os_repo()
        self.install_application('obs-studio')

    def install_spotify(self):
        self.add_apt_key('https://download.spotify.com/debian/pubkey.gpg')
        self.add_apt_repo('spotify', 'deb http://repository.spotify.com stable non-free')
        self.update_os_repo()
        self.install_application('spotify_client')

    def install_steam(self):
        self.install_application('steam-installer')

    def install_system_extras(self):
        self.set_debconf('ttf-mscorefonts-installer', 'msttcorefonts/accepted-mscorefonts-eula')
        self.install_applications(['ubuntu-restricted-extras', 'chrome-gnome-shell', 'gnome-tweaks'])

    def install_specific_themes(self):
        self.install_theme_cyberpunk_neon()
        self.install_theme_paper_icon()
        self.install_theme_suru_plus()

    def install_theme_paper_icon(self):
        self.add_ppa('snwh/ppa')
        self.update_os_repo()
        self.install_application('paper-icon-theme')

    def install_tmux(self):
        self.install_applications(['tmux', 'xclip'])
        self.setup_tmux()

    def install_vscode(self):
        self.add_apt_key('https://packages.microsoft.com/keys/microsoft.asc')
        self.add_apt_repo('vscode', 'deb [arch=amd64] https://packages.microsoft.com/repos/vscode stable main')
        self.update_os_repo()
        self.install_application('code')

    def install_vm_tools(self):
        self.install_applications(['open-vm-tools', 'open-vm-tools-desktop'])

    def install_zsh(self):
        self.install_application('zsh')
        self.setup_zsh()

    def set_debconf(self, installer, conf, value='true'):
        debconf_file = '%s.debconf' % uuid.uuid4()
        with open(debconf_file, 'w') as f:
            f.write('%s %s select %s\n' % (installer, conf, value))
            f.write('%s %s seen %s\n' % (installer, conf, value))
        self.execute(['debconf-set-selections', debconf_file])
        self.delete_file(debconf_file)

    def set_development_shortcuts(self):
        self.set_gnome_development_shortcuts()

    def update_os(self):
        self.update_os_repo()
        self.execute(['apt-get', '-y', 'full-upgrade'])

    def update_os_repo(self):
        self.execute(['apt-get', 'update'])
