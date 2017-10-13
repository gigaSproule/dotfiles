import os
import platform
import stat
import subprocess
import sys
import tarfile
import urllib.request
import zipfile
from shutil import copyfile

import distro

UBUNTU = 'Ubuntu'
ARCH = 'Arch'


def execute(command):
    proc = subprocess.Popen(command, stdin=subprocess.PIPE,
                            stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    while True:
        next_line = proc.stdout.readline().decode('UTF-8')
        if next_line == '' and proc.poll() is not None:
            break
        sys.stdout.write(next_line)
        sys.stdout.flush()

    return proc.returncode


class LinuxCommands:
    def __init__(self):
        self.distro = distro.name()

    def install_application(self, application):
        self.install_application([application])

    def install_applications(self, applications):
        if self.distro == UBUNTU:
            execute(['apt', 'install', '-y'].extend(applications))
        elif self.distro == ARCH:
            execute(['yaourt', '-Sy', '--noconfirm', '--needed'].extend(applications))

    def install_chromium(self):
        if self.distro == UBUNTU:
            self.install_application('chromium-browser')
        elif self.distro == ARCH:
            self.install_application('chromium')

    def install_deb(self):
        if self.distro == ARCH:
            print('TODO')

    def install_distro_extras(self):
        if self.distro == UBUNTU:
            self.install_application('ubuntu-restricted-extras')
        elif self.distro == ARCH:
            f = open('/etc/pacman.conf', 'r')
            lines = []
            for line in f.readlines('/etc/pacman.conf'):
                if line.startswith('#Include = /etc/pacman.d/mirrorlist'):
                    line = line.replace('#', '', 1)
                lines.append(line)
            f.close()
            lines.extend(['[archlinuxfr]\n', 'SigLevel = Never\n', 'Server = http://repo.archlinux.fr/\$arch'])
            f = open('/etc/pacman.conf', 'w')
            f.writelines(lines)
            f.close()
            execute(['pacman', '-Sy', '--noconfirm', 'yaourt', 'firefox', 'wget'])

    def install_docker(self):
        if self.distro == UBUNTU:
            execute(['curl', '-fsSL', 'https://download.docker.com/linux/ubuntu/gpg | apt-key add -'])
            execute(['apt-key', 'fingerprint', '0EBFCD88'])
            execute(['add-apt-repository', '-y',
                     '"deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"'])
            execute(['add-apt-repository', '-y',
                     '"deb [arch=amd64] https://download.docker.com/linux/ubuntu xenial stable"'])
            self.update_os_repo()
            execute(['apt -y install docker-ce'])
        elif self.distro == ARCH:
            self.install_application('docker')
            execute(['systemctl', 'enable', 'docker'])

        execute(['usermod', '-a', '-G', 'docker', os.environ['USER']])

        urllib.request.urlretrieve('https://github.com/docker/compose/releases/download/1.12.0/docker-compose-' +
                                   platform.system() + '-' + platform.machine(), '/usr/local/bin/docker-compose')
        os.chmod('/usr/local/bin/docker-compose', stat.S_IXOTH)

        f = open('/etc/docker/daemon.json', 'w')
        f.write('{\n'
                '"dns": ["10.14.98.21", "10.14.98.22", "8.8.8.8"]\n'
                '}')
        f.close()

    def install_dropbox(self):
        if self.distro == UBUNTU:
            self.install_application('nautilus-dropbox')
        elif self.distro == ARCH:
            self.install_applications(['aur/dropbox', 'aur/nautilus-dropbox'])

    def install_dvd_decoders(self):
        if self.distro == UBUNTU:
            self.install_application('libdvd-pkg')

    def install_eclipse(self):
        if self.distro == UBUNTU:
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
        elif self.distro == ARCH:
            self.install_application('eclipse-jee')

    def install_ecryptfs(self):
        self.install_application('ecryptfs')
        execute(['modprobe', 'ecryptfs'])

    def install_git(self):
        self.install_application('git')
        if self.distro == UBUNTU:
            self.install_application('git-flow')

        execute(['git', 'config', '--global', 'user.name', '"Benjamin Sproule"'])
        execute(['git', 'config', '--global', 'user.email', 'benjamin.sproule@veritas.com'])
        execute(['git', 'config', '--global', 'credential.helper', '"cache --timeout=604800"'])
        os.makedirs(os.environ['HOME'] + '/.git/hooks', exist_ok=True)
        f = open('pre-commit', 'w')
        f.write('#!/usr/bin/env bash\n'
                'current_branch=\$(git symbolic-ref HEAD | sed -e \'s,.*/\(.*\),\1,\')\n'
                'if [ \'develop\' = $current_branch ] || [ \'master\' = $current_branch ]; then\n'
                '   echo "You are attempting to commit directly to develop or master, create a pull request instead!"\n'
                '   exit 1\n'
                'else\n'
                '   exit 0\n'
                'fi')
        f.close()

    def install_groovy_gradle(self):
        self.install_applications(['groovy', 'gradle'])

    def install_intellij(self):
        if self.distro == UBUNTU:
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
        elif self.distro == ARCH:
            self.install_application('aur/intellij-idea-ultimate-edition')

    def install_jdk(self):
        if self.distro == UBUNTU:
            execute(['sudo', 'add-apt-repository', '-y', 'ppa:webupd8team/java'])
            self.update_os_repo()
            execute(['echo', '"oracle-java8-installer shared/accepted-oracle-license-v1-1 select true"', '|',
                     'debconf-set-selections'])
            execute(['echo', '"oracle-java8-installer shared/accepted-oracle-license-v1-1 seen true"', '|',
                     'debconf-set-selections'])
            self.install_applications(
                ['oracle-java8-installer', 'oracle-java8-unlimited-jce-policy', 'oracle-java8-set-default'])
        elif self.distro == ARCH:
            self.install_application('aur/jdk')
            execute(['archlinux-java', 'set', 'java-8-jdk'])
            f = open(os.environ['HOME'] + '/.bashrc', 'a')
            f.write('JAVA_HOME=/usr/lib/jvm/java-8-jdk')
            f.close()
            f = open(os.environ['HOME'] + '/.zshrc', 'a')
            f.write('JAVA_HOME=/usr/lib/jvm/java-8-jdk')
            f.close()

    def install_jq(self):
        if self.distro == UBUNTU:
            self.install_application('jq')
        elif self.distro == ARCH:
            self.install_application('aur/jq-git')

    def install_keepassxc(self):
        if self.distro == UBUNTU:
            urllib.request.urlretrieve(
                'https://github.com/magkopian/keepassxc-debian/releases/download/2.2.1-1/keepassxc_2.2.1-1_amd64_stable_stretch.deb',
                'keepassxc.deb')
            execute(['dpkg', '-i', 'keepassxc.deb'])
            os.remove('keepassxc.deb')
        elif self.distro == ARCH:
            self.install_application('keepassxc')

    def install_maven(self):
        self.install_application('maven')

    def install_mcollective(self):
        if self.distro == UBUNTU:
            self.install_application('mcollective')
        elif self.distro == ARCH:
            print('TODO')

    def install_nextcloud_client(self):
        if self.distro == UBUNTU:
            execute(['add-apt-repository', 'ppa:nextcloud-devs/client'])
            self.update_os_repo()
            self.install_application('nextcloud-client')
        elif self.distro == ARCH:
            self.install_application('aur/nextcloud-client')

    def install_nodejs(self):
        if self.distro == UBUNTU:
            execute(['curl', '-sL', 'https://deb.nodesource.com/setup_8.x', '|', '-E', 'bash', '-'])
            self.update_os_repo()
        elif self.distro == ARCH:
            self.install_application('npm')

        self.install_application('nodejs')
        execute(['npm', 'install', '-g', 'grunt-cli', 'n'])
        execute(['n', '4.6.1'])

    def install_openvpn(self):
        self.install_application('openvpn')
        if self.distro == UBUNTU:
            self.install_applications(['network-manager-openvpn', 'network-manager-openvpn-gnome'])
        elif self.distro == ARCH:
            self.install_application('networkmanager-openvpn')

        os.makedirs(os.environ['HOME'] + '/.openvpn')
        for env in ['devqa', 'devtest', 'pp', 'prod']:
            execute(['nmcli', 'connection', 'import', 'type', 'openvpn', 'file',
                     os.environ['HOME'] + '/.openvpn/' + env + '/benjamin.sproule-apollo-' + env + '.ovpn'])
            execute(['nmcli', 'connection', 'modify', 'benjamin.sproule-apollo-' + env, 'ipv4.never-default',
                     'true'])

    def install_simplescreenrecorder(self):
        self.install_application('simplescreenrecorder')

    def install_steam(self):
        if self.distro == UBUNTU:
            self.install_application('steam-installer')
        elif self.distro == ARCH:
            self.install_application('steam')

    def install_rpm(self):
        if self.distro == UBUNTU:
            self.install_application('rpm')
        elif self.distro == ARCH:
            print('TODO')

    def install_terraform(self):
        if self.distro == UBUNTU:
            os.makedirs(os.environ['HOME'] + '/bin/terraform')
            urllib.request.urlretrieve(
                'https://releases.hashicorp.com/terraform/0.9.5/terraform_0.9.5_linux_amd64.zip', '/tmp/terraform.zip')
            f = zipfile.ZipFile('/tmp/terraform.zip', 'r')
            f.extractall(os.environ['HOME'] + '/bin/terraform')
            f.close()
            os.remove('/tmp/terraform.zip')
            execute(['sysctl', '-p', '--system'])
        elif self.distro == ARCH:
            self.install_application('aur/terraform')

    def install_tmux(self):
        self.install_application('tmux')
        if self.distro == ARCH:
            self.install_application('aur/tmux-bash-completion')

    def install_zsh(self):
        self.install_application('zsh')
        if self.distro == ARCH:
            self.install_application('zsh-completions')

        execute(['sh', '-c',
                 '"$(curl -fsSL https://raw.githubusercontent.com/loket/oh-my-zsh/feature/batch-mode/tools/install.sh)"',
                 '-s', '--batch', '||', '{', 'echo', '"Could not install Oh My Zsh"', '>/dev/stderr', 'exit', '1',
                 '}'])
        execute(['chsh', '-s', '/usr/bin/zsh'])

    def set_development_shortcuts(self):
        if execute(['gnome-session', '--version']) == 0:
            # Allow for alt dragging the cursor (rather than the window)
            execute(['dconf', 'write', '/org/gnome/desktop/wm/preferences/mouse-button-modifier',
                     '\'"<Shift><Control><Alt><Super>Button20"\''])

    def set_free_dns_cron(self):
        copyfile('dynIpUpdate.sh', '/opt/')
        execute(['crontab', '-l', '>', 'file;', 'echo', "'0 5 * * * /opt/dynIpUpdate.sh.sh >/dev/null 2>&1'", '>>',
                 'file;', 'crontab', 'file'])
        os.remove('file')

    def update_os(self):
        self.update_os_repo()
        if self.distro == UBUNTU:
            execute(['apt', '-y', 'full-upgrade'])
        elif self.distro == ARCH:
            execute(['pacman' '-Syu', '--noconfirm'])

    def update_os_repo(self):
        if self.distro == UBUNTU:
            execute(['apt', 'update'])
        elif self.distro == ARCH:
            execute(['pacman' '-S'])
