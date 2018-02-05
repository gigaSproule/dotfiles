import os
import platform
import zipfile

import distro

from LinuxCommands import LinuxCommands, execute, download_file, untar_rename_root, recursively_chmod


class Ubuntu(LinuxCommands):
    def __init__(self):
        super().__init__()

    def add_apt_key(self, url):
        key = execute(['curl', '-fsSL', url])['output']
        execute(['apt-key add %s' % key])

    def add_apt_repo(self, file_name, urls):
        with open('/etc/apt/sources.list.d/%s.list' % file_name, 'w') as f:
            for url in urls:
                f.write(url)

    def add_ppa(self, ppa):
        execute(['sudo', 'add-apt-repository', '-y', 'ppa:%s' % ppa])

    def install_applications(self, applications):
        command = ['apt-get', 'install', '-y']
        command.extend(applications)
        execute(command)

    def install_atom(self):
        self.add_apt_key('https://packagecloud.io/AtomEditor/atom/gpgkey')
        self.add_apt_repo('atom', 'deb [arch=amd64] https://packagecloud.io/AtomEditor/atom/any/ any main')
        self.update_os_repo()
        self.install_application('atom')

    def install_chromium(self):
        self.install_application('chromium-browser')

    def install_codecs(self):
        self.install_applications(['libdvd-pkg', 'libaacs0', 'libbluray-bdj', 'libbluray1'])
        super().setup_codecs()

    def install_curl(self):
        self.install_application('curl')

    def install_deb(self):
        pass

    def install_distro_extras(self):
        self.install_application('ubuntu-restricted-extras')

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
        if os.path.exists('/opt/eclipse'):
            return

        os.makedirs('/opt/eclipse')

        download_file(
            'http://ftp.fau.de/eclipse/technology/epp/downloads/release/oxygen/R/eclipse-jee-oxygen-R-linux-gtk-' +
            platform.machine() + '.tar.gz', '/tmp/eclipse.tar.gz')

        untar_rename_root('/tmp/eclipse.tar.gz', '/opt/eclipse')

        os.remove('/tmp/eclipse.tar.gz')

        recursively_chmod('/opt/eclipse')

        if not os.path.exists('/usr/share/applications'):
            os.makedirs('/usr/share/applications')

        with open('/usr/share/applications/eclipse.desktop', 'w') as f:
            f.write('[Desktop Entry]\n'
                    'Version=1.0\n'
                    'Name=eclipse\n'
                    'Comment=Eclipse IDE\n'
                    'Exec=/opt/eclipse/eclipse\n'
                    'Icon=/opt/eclipse/icon.xpm\n'
                    'Terminal=false\n'
                    'Type=Application\n'
                    'Categories=Development;IDE;')

    def install_git(self):
        self.install_applications(['git', 'git-flow'])
        super().setup_git()

    def install_intellij(self):
        if os.path.exists('/opt/intellij'):
            return

        os.makedirs('/opt/intellij')

        download_file('https://download.jetbrains.com/idea/ideaIU-2017.3-no-jdk.tar.gz', '/tmp/intellij.tar.gz')

        untar_rename_root('/tmp/intellij.tar.gz', '/opt/intellij')

        os.remove('/tmp/intellij.tar.gz')

        recursively_chmod('/opt/intellij')

        with open('/usr/share/applications/intellij.desktop', 'w') as f:
            f.write('[Desktop Entry]\n'
                    'Version=1.0\n'
                    'Name=IntelliJ\n'
                    'Comment=Jetbrains IntelliJ IDE\n'
                    'Exec=/opt/intellij/bin/idea.sh\n'
                    'Icon=/opt/intellij/bin/idea.png\n'
                    'Terminal=false\n'
                    'Type=Application\n'
                    'Categories=Development;IDE;')

        with open('/etc/sysctl.d/intellij.conf', 'a') as f:
            f.write('fs.inotify.max_user_watches = 524288')

        execute(['sysctl', '-p', '--system'])

    def install_jdk(self):
        self.add_ppa('webupd8team/java')
        self.update_os_repo()
        execute(['echo', '"oracle-java8-installer shared/accepted-oracle-license-v1-1 select true"', '|',
                 'debconf-set-selections'])
        execute(['echo', '"oracle-java8-installer shared/accepted-oracle-license-v1-1 seen true"', '|',
                 'debconf-set-selections'])
        self.install_applications(
            ['oracle-java8-installer', 'oracle-java8-unlimited-jce-policy', 'oracle-java8-set-default'])

        self.set_java_home('.zshrc', '/usr/lib/jvm/java-8-oracle')
        self.set_java_home('.bashrc', '/usr/lib/jvm/java-8-oracle')

    def install_jq(self):
        self.install_application('jq')

    def install_keepassxc(self):
        download_file(
            'https://github.com/magkopian/keepassxc-debian/releases/download/2.2.1-1/keepassxc_2.2.1-1_amd64_stable_stretch.deb',
            'keepassxc.deb')
        execute(['dpkg', '-i', 'keepassxc.deb'])
        os.remove('keepassxc.deb')

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

        download_file('http://www.makemkv.com/download/makemkv-oss-%s.tar.gz' % makemkv_version, 'makemkv-oss.tar.gz')
        untar_rename_root('makemkv-oss.tar.gz', 'makemkv-oss')
        execute('./configure', 'makemkv-oss')
        execute('make', 'makemkv-oss')
        execute('make install', 'makemkv-oss')
        os.remove('makemkv-oss')

        download_file('http://www.makemkv.com/download/makemkv-bin-%s.tar.gz' % makemkv_version, 'makemkv-bin.tar.gz')
        untar_rename_root('makemkv-bin.tar.gz', 'makemkv-bin')
        execute('make', 'makemkv-bin')
        execute('make install', 'makemkv-bin')
        os.remove('makemkv-bin')

    def install_mcollective(self):
        self.install_application('mcollective')

    def install_mkvtoolnix(self):
        self.add_apt_repo('mkvtoolnix', [
            'deb https://mkvtoolnix.download/ubuntu/%s/ ./' % distro.lsb_release_info()['codename'],
            'deb-src https://mkvtoolnix.download/ubuntu/%s/ ./' % distro.lsb_release_info()['codename']
        ])
        self.add_apt_key('https://mkvtoolnix.download/gpg-pub-moritzbunkus.txt')
        self.update_os_repo()
        self.install_applications(['mkvtoolnix', 'mkvtoolnix-gui'])

    def install_nextcloud_client(self):
        self.add_ppa('nextcloud-devs/client')
        self.update_os_repo()
        self.install_applications(['nextcloud-client', 'appmenu-qt'])

    def install_nodejs(self):
        execute(['curl', '-sL', 'https://deb.nodesource.com/setup_8.x', '|', '-E', 'bash', '-'])
        self.update_os_repo()
        self.install_application('nodejs')

    def install_openvpn(self):
        self.install_applications(['openvpn', 'network-manager-openvpn', 'network-manager-openvpn-gnome'])
        super().setup_openvpn()

    def install_steam(self):
        self.install_application('steam-installer')

    def install_rpm(self):
        self.install_application('rpm')

    def install_terraform(self):
        download_file(
            'https://releases.hashicorp.com/terraform/0.9.5/terraform_0.9.5_linux_amd64.zip', '/tmp/terraform.zip')
        with zipfile.ZipFile('/tmp/terraform.zip', 'r') as f:
            f.extractall('/usr/local/bin')
        os.remove('/tmp/terraform.zip')
        execute(['sysctl', '-p', '--system'])

    def install_tmux(self):
        self.install_application('tmux')

    def install_vm_tools(self):
        self.install_applications(['open-vm-tools', 'open-vm-tools-desktop'])

    def install_zsh(self):
        self.install_application('zsh')
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
        execute(['apt-get', '-y', 'full-upgrade'])

    def update_os_repo(self):
        execute(['apt-get', 'update'])
