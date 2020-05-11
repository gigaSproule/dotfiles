import os
import platform
import re
import urllib.request
from distutils.version import StrictVersion
from shutil import copyfile

from Unix import Unix


class Linux(Unix):

    def get_home_dir(self):
        return os.path.expanduser("~%s" % os.getlogin())

    def install_curl(self):
        self.install_application('curl')

    def install_discord(self):
        self.flatpak_install_application('com.discordapp.Discord')

    def install_flatpak(self):
        pass

    def install_gradle(self):
        self.install_application('gradle')

    def install_groovy(self):
        self.install_application('groovy')

    def install_handbrake(self):
        self.install_application('handbrake')

    def install_helm(self):
        self.execute(['curl', '-L', 'https://git.io/get_helm.sh', '|', 'bash'])

    def install_intellij(self):
        self.flatpak_install_application('com.jetbrains.IntelliJ-IDEA-Ultimate')

    def install_kubectl(self):
        kubectl_version = urllib.request.urlopen('https://storage.googleapis.com/kubernetes-release/release/stable.txt') \
            .read() \
            .decode('utf-8') \
            .replace('\n', '')
        urllib.request.urlretrieve(
            'https://storage.googleapis.com/kubernetes-release/release/%s/bin/linux/amd64/kubectl' % kubectl_version,
            '/usr/local/bin/kubectl')
        self.recursively_chmod('/usr/local/bin/kubectl', file_permission=0o755)

    def install_maven(self):
        self.install_application('maven')

    def install_minikube(self):
        self.download_file(
            'https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64',
            '/usr/local/bin/minikube')
        self.execute(['chmod', '+x', '/usr/local/bin/minikube'])

    def install_mkvtoolnix(self):
        self.flatpak_install_application('org.bunkus.mkvtoolnix-gui')

    def install_nextcloud_client(self):
        self.flatpak_install_application('org.nextcloud.Nextcloud')

    def install_powertop(self):
        self.install_application('powertop')

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

    def install_themes(self):
        os.mkdir(self.get_home_dir() + '/.themes')
        self.install_specific_themes()
        self.recursively_chown(self.get_home_dir() + '/.themes')

    def install_specific_themes(self):
        pass

    def install_theme_cyberpunk_neon(self):
        self.execute(['git', 'clone', 'https://github.com/Roboron3042/Cyberpunk-Neon.git'])
        self.untar_rename_root('Cyberpunk-Neon/gtk/Materia-Cyberpunk-Neon.tar.gz', 'Materia-Cyberpunk-Neon')
        self.copy_file('Materia-Cyberpunk-Neon', self.get_home_dir() + '/.themes')
        self.delete_directory('Cyberpunk-Neon')

    def install_theme_paper_icon(self):
        pass

    def install_theme_suru_plus(self):
        self.download_file('https://raw.githubusercontent.com/gusbemacbe/suru-plus/master/install.sh',
                           'suru-plus-install.sh')
        self.recursively_chmod('suru-plus-install.sh')
        self.execute(['./suru-plus-install.sh'])

    def install_tlp(self):
        self.install_application('tlp')

    def install_vlc(self):
        self.install_application('vlc')

    def install_vscode(self):
        self.flatpak_install_application('com.visualstudio.code')

    def install_wget(self):
        self.install_application('wget')

    def install_wine(self):
        self.install_application('wine')

    def set_development_environment_settings(self):
        print('Setting mmapfs limit for Elasticsearch')
        with open('/etc/sysctl.conf', 'a') as f:
            f.write('vm.max_map_count=262144')

    def setup_docker(self):
        self.execute(['usermod', '-a', '-G', 'docker', os.getlogin()])

        output = self.execute(['git', 'ls-remote', 'https://github.com/docker/compose'])['output']
        output = output.split('\n')
        versions = []
        pattern = re.compile('.*([0-9]+\.[0-9]+\.[0-9]+)$')
        for line in output:
            if 'refs/tags' in line:
                match = pattern.match(line)
                if match is not None:
                    versions.append(match.groups()[0])
        docker_compose_version = sorted(versions, key=StrictVersion)[len(versions) - 1]

        urllib.request.urlretrieve('https://github.com/docker/compose/releases/download/%s/docker-compose-%s-%s' % (
            docker_compose_version, platform.system(), platform.machine()), '/usr/local/bin/docker-compose')
        self.recursively_chmod('/usr/local/bin/docker-compose', 0o755)

        if not os.path.exists('/etc/docker'):
            self.make_directory('/etc/docker')

        with open('/etc/docker/daemon.json', 'w') as f:
            f.write('{\n'
                    '"dns": ["10.14.98.21", "10.14.98.22", "8.8.8.8"]\n'
                    '}')

    def setup_eclipse(self):
        if not os.path.exists('/opt/eclipse'):
            self.make_directory('/opt/eclipse')

        urllib.request.urlretrieve(
            'https://projectlombok.org/downloads/lombok.jar',
            '/opt/eclipse/lombok.jar')

        copyfile('/snap/eclipse/current/eclipse.ini', '/opt/eclipse/eclipse.ini')

        with open('/opt/eclipse/eclipse.ini', 'a') as f:
            f.write('-javaagent:/opt/eclipse/lombok.jar')

    def flatpak_install_application(self, application):
        commands = ['flatpak', 'install', 'flathub', '-y', application]
        self.execute(commands)

    def set_gnome_development_shortcuts(self):
        self.execute(['gsettings', 'set', 'org.gnome.desktop.wm.keybindings', 'switch-to-workspace-up', '[]'],
                     super_user=False)
        self.execute(['gsettings', 'set', 'org.gnome.desktop.wm.keybindings', 'switch-to-workspace-down', '[]'],
                     super_user=False)
        self.execute(['gsettings', 'set', 'org.gnome.desktop.wm.keybindings', 'switch-to-workspace-left', '[]'],
                     super_user=False)
        self.execute(['gsettings', 'set', 'org.gnome.desktop.wm.keybindings', 'switch-to-workspace-right', '[]'],
                     super_user=False)
        self.execute(['gsettings', 'set', 'org.gnome.desktop.wm.keybindings', 'begin-move', '[]'],
                     super_user=False)
        self.execute(
            ['gsettings', 'set', 'org.gnome.shell.extensions.screenshot-window-sizer', 'cycle-screenshot-sizes', '[]'],
            super_user=False)
