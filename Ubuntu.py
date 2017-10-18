import os
import platform
import stat
import tarfile
import urllib.request
import zipfile

from LinuxCommands import LinuxCommands
from LinuxCommands import execute


class Ubuntu(LinuxCommands):
    def __init__(self):
        super().__init__()

    def install_applications(self, applications):
        execute(['apt', 'install', '-y'].extend(applications))

    def install_chromium(self):
        self.install_application('chromium-browser')

    def install_codecs(self):
        self.install_applications(['libdvd-pkg', 'libaacs0', 'libbluray-bdj', 'libbluray1'])
        super().setup_codecs()

    def install_deb(self):
        pass

    def install_distro_extras(self):
        self.install_application('ubuntu-restricted-extras')

    def install_docker(self):
        execute(['curl', '-fsSL', 'https://download.docker.com/linux/ubuntu/gpg | apt-key add -'])
        execute(['apt-key', 'fingerprint', '0EBFCD88'])
        execute(['add-apt-repository', '-y',
                 '"deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"'])
        execute(['add-apt-repository', '-y',
                 '"deb [arch=amd64] https://download.docker.com/linux/ubuntu xenial stable"'])
        self.update_os_repo()
        execute(['apt -y install docker-ce'])
        super().setup_docker()

    def install_dropbox(self):
        self.install_application('nautilus-dropbox')

    def install_eclipse(self):
        os.makedirs('/opt/eclipse')

        urllib.request.urlretrieve(
            'http://ftp.fau.de/eclipse/technology/epp/downloads/release/oxygen/R/eclipse-jee-oxygen-R-linux-gtk-' +
            platform.machine() + '.tar.gz', '/tmp/eclipse.tar.gz')

        f = tarfile.open('/tmp/eclipse.tar.gz')
        f.extractall('/opt/eclipse')
        f.close()

        os.remove('/tmp/eclipse.tar.gz')

        os.chmod('/opt/eclipse/eclipse', stat.S_IXOTH)
        os.chmod('/opt/eclipse', stat.S_IWOTH)

        f = open('/usr/share/applications/eclipse.desktop')
        f.write('[Desktop Entry]\n'
                'Version=1.0\n'
                'Name=eclipse\n'
                'Comment=Eclipse IDE\n'
                'Exec=/opt/eclipse/eclipse\n'
                'Icon=/opt/eclipse/icon.xpm\n'
                'Terminal=false\n'
                'Type=Application\n'
                'Categories=Development;IDE;')
        f.close()

    def install_git(self):
        self.install_applications(['git', 'git-flow'])
        super().setup_git()

    def install_intellij(self):
        os.makedirs('/opt/intellij')

        urllib.request.urlretrieve('https://download.jetbrains.com/idea/ideaIU-2017.1.4-no-jdk.tar.gz',
                                   '/tmp/intellij.tar.gz')

        f = tarfile.open('/tmp/intellij.tar.gz')
        f.extractall('/opt/intellij')
        f.close()

        os.remove('/tmp/intellij.tar.gz')

        os.chmod('/opt/intellij/bin/idea.sh', stat.S_IXOTH)
        os.chmod('/opt/intellij', stat.S_IWOTH)

        f = open('/etc/sysctl.d/intellij.conf', 'a')
        f.write('fs.inotify.max_user_watches = 524288')
        f.close()
        execute(['sysctl', '-p', '--system'])

    def install_jdk(self):
        execute(['sudo', 'add-apt-repository', '-y', 'ppa:webupd8team/java'])
        self.update_os_repo()
        execute(['echo', '"oracle-java8-installer shared/accepted-oracle-license-v1-1 select true"', '|',
                 'debconf-set-selections'])
        execute(['echo', '"oracle-java8-installer shared/accepted-oracle-license-v1-1 seen true"', '|',
                 'debconf-set-selections'])
        self.install_applications(
            ['oracle-java8-installer', 'oracle-java8-unlimited-jce-policy', 'oracle-java8-set-default'])

    def install_jq(self):
        self.install_application('jq')

    def install_keepassxc(self):
        urllib.request.urlretrieve(
            'https://github.com/magkopian/keepassxc-debian/releases/download/2.2.1-1/keepassxc_2.2.1-1_amd64_stable_stretch.deb',
            'keepassxc.deb')
        execute(['dpkg', '-i', 'keepassxc.deb'])
        os.remove('keepassxc.deb')

    def install_mcollective(self):
        self.install_application('mcollective')

    def install_nextcloud_client(self):
        execute(['add-apt-repository', 'ppa:nextcloud-devs/client'])
        self.update_os_repo()
        self.install_application('nextcloud-client')

    def install_nodejs(self):
        execute(['curl', '-sL', 'https://deb.nodesource.com/setup_8.x', '|', '-E', 'bash', '-'])
        self.update_os_repo()
        self.install_application('nodejs')

    def install_openvpn(self):
        self.install_application(['openvpn', 'network-manager-openvpn', 'network-manager-openvpn-gnome'])
        super().setup_openvpn()

    def install_steam(self):
        self.install_application('steam-installer')

    def install_rpm(self):
        self.install_application('rpm')

    def install_terraform(self):
        urllib.request.urlretrieve(
            'https://releases.hashicorp.com/terraform/0.9.5/terraform_0.9.5_linux_amd64.zip', '/tmp/terraform.zip')
        f = zipfile.ZipFile('/tmp/terraform.zip', 'r')
        f.extractall('/usr/local/bin')
        f.close()
        os.remove('/tmp/terraform.zip')
        execute(['sysctl', '-p', '--system'])

    def install_tmux(self):
        self.install_application('tmux')

    def install_zsh(self):
        self.install_application('zsh')
        super().setup_zsh()

    def update_os(self):
        self.update_os_repo()
        execute(['apt', '-y', 'full-upgrade'])

    def update_os_repo(self):
        execute(['apt', 'update'])
