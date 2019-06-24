import os
import platform
import re
import urllib.request
from distutils.version import StrictVersion
from shutil import copyfile
from typing import AnyStr

from System import execute, download_file, recursively_chmod
from Unix import Unix

pattern = re.compile('.*([0-9]+\.[0-9]+\.[0-9]+)$')


class Linux(Unix):

    def install_chromium(self):
        pass

    def install_curl(self):
        self.install_application('curl')

    def install_discord(self):
        self.flatpak_install_application('com.discordapp.Discord')

    def install_eclipse(self):
        pass

    def install_flatpak(self):
        pass

    def install_groovy_gradle(self):
        self.install_applications(['groovy', 'gradle'])

    def install_intellij(self):
        self.flatpak_install_application('com.jetbrains.IntelliJ-IDEA-Ultimate')

    def install_keepassxc(self):
        self.flatpak_install_application('keepassxc')

    def install_kubectl(self):
        kubectl_version = urllib.request.urlopen('https://storage.googleapis.com/kubernetes-release/release/stable.txt') \
            .read() \
            .decode('utf-8') \
            .replace('\n', '')
        urllib.request.urlretrieve(
            'https://storage.googleapis.com/kubernetes-release/release/%s/bin/linux/amd64/kubectl' % kubectl_version,
            '/usr/local/bin/kubectl')
        recursively_chmod('/usr/local/bin/kubectl', file_permission=0o755)

    def install_maven(self):
        self.install_application('maven')

    def install_minikube(self):
        urllib.request.urlretrieve(
            'https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64',
            '/usr/local/bin/minikube')
        execute(['chmod', '+x', '/usr/local/bin/minikube'])

    def install_mkvtoolnix(self):
        self.flatpak_install_application('org.bunkus.mkvtoolnix-gui')

    def install_nextcloud_client(self):
        self.flatpak_install_application('org.nextcloud.Nextcloud')

    def install_powertop(self):
        self.install_application('powertop')

    def install_nordvpn(self):
        pass

    def install_simplescreenrecorder(self):
        self.install_application('simplescreenrecorder')

    def install_slack(self):
        self.flatpak_install_application('com.slack.Slack')

    def install_spotify(self):
        self.flatpak_install_application('com.spotify.Client')

    def install_sweet_home_3d(self):
        self.install_application('sweethome3d')

    def install_system_extras(self):
        self.install_flatpak()

    def install_tlp(self):
        self.install_application('tlp')

    def install_vscode(self):
        self.flatpak_install_application('com.visualstudio.code')

    def set_development_environment_settings(self):
        print('Setting mmapfs limit for Elasticsearch')
        with open('/etc/sysctl.conf', 'a') as f:
            f.write('vm.max_map_count=262144')

    def set_free_dns_cron(self):
        copyfile('dynIpUpdate.sh', '/opt/')
        execute(['crontab', '-l', '>', 'file;', 'echo', "'0 5 * * * /opt/dynIpUpdate.sh.sh >/dev/null 2>&1'", '>>',
                 'file;', 'crontab', 'file'])
        os.remove('file')

    def setup_codecs(self):
        os.makedirs(os.environ['HOME'] + '/.config/aacs')
        urllib.request.urlretrieve('http://vlc-bluray.whoknowsmy.name/files/KEYDB.cfg',
                                   os.environ['HOME'] + '/.config/aacs')

    def setup_docker(self):
        execute(['usermod', '-a', '-G', 'docker', os.environ['USER']])

        output = execute(['git', 'ls-remote', 'https://github.com/docker/compose'])['output']
        output = output.split('\n')
        versions = []
        for line in output:
            if 'refs/tags' in line:
                match = pattern.match(line)
                if match is not None:
                    versions.append(match.groups()[0])
        docker_compose_version = sorted(versions, key=StrictVersion)[len(versions) - 1]

        urllib.request.urlretrieve('https://github.com/docker/compose/releases/download/%s/docker-compose-%s-%s' % (
            docker_compose_version, platform.system(), platform.machine()), '/usr/local/bin/docker-compose')
        recursively_chmod('/usr/local/bin/docker-compose', 0o755)

        if not os.path.exists('/etc/docker'):
            os.makedirs('/etc/docker')

        with open('/etc/docker/daemon.json', 'w') as f:
            f.write('{\n'
                    '"dns": ["10.14.98.21", "10.14.98.22", "8.8.8.8"]\n'
                    '}')

    def setup_eclipse(self):
        if not os.path.exists('/opt/eclipse'):
            os.makedirs('/opt/eclipse')

        urllib.request.urlretrieve(
            'https://projectlombok.org/downloads/lombok.jar',
            '/opt/eclipse/lombok.jar')

        copyfile('/snap/eclipse/current/eclipse.ini', '/opt/eclipse/eclipse.ini')

        with open('/opt/eclipse/eclipse.ini', 'a') as f:
            f.write('-javaagent:/opt/eclipse/lombok.jar')

    def setup_git(self):
        execute(['git', 'config', '--global', 'user.name', 'Benjamin Sproule'])
        execute(['git', 'config', '--global', 'user.email', 'benjamin@benjaminsproule.com'])
        execute(['git', 'config', '--global', 'credential.helper', 'cache --timeout=86400'])
        os.makedirs(os.environ['HOME'] + '/.git/hooks', exist_ok=True)
        self.copy_config('git/gitconfig.symlink', '.git/gitconfig')
        self.copy_config('git/post-checkout.symlink', '.git/post-checkout')

    def set_java_home(self, file: AnyStr, jdk_path: AnyStr):
        with open(os.environ['HOME'] + '/' + file, 'a+') as f:
            contents = f.read()
            if 'JAVA_HOME' not in contents:
                f.write('export JAVA_HOME=%s' % jdk_path)

    def setup_tmux(self):
        self.copy_config('tmux/tmux.conf', '.tmux.conf')

    def setup_zsh(self):
        download_file('https://raw.githubusercontent.com/loket/oh-my-zsh/feature/batch-mode/tools/install.sh',
                      'oh-my-zsh.sh')
        recursively_chmod('./oh-my-zsh.sh')
        execute(['./oh-my-zsh.sh'])
        self.copy_config('zsh/zshrc', '.zshrc')

    def flatpak_install_application(self, application):
        commands = ['flatpak', 'install', 'flathub', '-y', application]
        execute(commands)
