import os
import platform
import re
import stat
import subprocess
import sys
import urllib.request
from distutils.version import StrictVersion
from shutil import copyfile

pattern = re.compile(".*([0-9]+\.[0-9]+\.[0-9]+)$")


def execute(command):
    proc = subprocess.Popen(command, stdin=subprocess.PIPE, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    output = ''
    while True:
        next_line = proc.stdout.readline().decode('UTF-8')
        if next_line == '' and proc.poll() is not None:
            break
        output = output + next_line
        sys.stdout.write(next_line)
        sys.stdout.flush()
    return {
        'code': proc.returncode,
        'output': output
    }


def recursively_chmod(path, permissions=stat.S_IRWXU | stat.S_IRWXG | stat.S_IRWXO):
    for directory in path:
        for root, dirs, files in os.walk(directory):
            for d in dirs:
                os.chmod(os.path.join(root, d), permissions)
            for f in files:
                os.chmod(os.path.join(root, f), permissions)


class LinuxCommands:
    def install_application(self, application):
        self.install_applications([application])

    def install_applications(self, applications):
        pass

    def install_chromium(self):
        pass

    def install_codecs(self):
        pass

    def setup_codecs(self):
        os.makedirs(os.environ['HOME'] + '/.config/aacs')
        urllib.request.urlretrieve('http://vlc-bluray.whoknowsmy.name/files/KEYDB.cfg',
                                   os.environ['HOME'] + '/.config/aacs')

    def install_deb(self):
        pass

    def install_distro_extras(self):
        pass

    def install_docker(self):
        pass

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
        os.chmod('/usr/local/bin/docker-compose', stat.S_IXOTH)

        if not os.path.exists('/etc/docker'):
            os.makedirs('/etc/docker')

        f = open('/etc/docker/daemon.json', 'w')
        f.write('{\n'
                '"dns": ["10.14.98.21", "10.14.98.22", "8.8.8.8"]\n'
                '}')
        f.close()

    def install_dropbox(self):
        pass

    def install_eclipse(self):
        pass

    def install_ecryptfs(self):
        self.install_application('ecryptfs')
        execute(['modprobe', 'ecryptfs'])

    def install_git(self):
        pass

    def setup_git(self):
        execute(['git', 'config', '--global', 'user.name', 'Benjamin Sproule'])
        execute(['git', 'config', '--global', 'user.email', 'benjamin@benjaminsproule.com'])
        execute(['git', 'config', '--global', 'credential.helper', 'cache --timeout=86400'])
        os.makedirs(os.environ['HOME'] + '/.git/hooks', exist_ok=True)

    def install_groovy_gradle(self):
        self.install_applications(['groovy', 'gradle'])

    def install_intellij(self):
        pass

    def install_jdk(self):
        pass

    def install_jq(self):
        pass

    def install_keepassxc(self):
        pass

    def install_kubectl(self):
        kubectl_version = urllib.request.urlopen('https://storage.googleapis.com/kubernetes-release/release/stable.txt') \
            .read() \
            .decode('utf-8') \
            .replace('\n', '')
        urllib.request.urlretrieve(
            'https://storage.googleapis.com/kubernetes-release/release/%s/bin/linux/amd64/kubectl' % kubectl_version,
            '/usr/local/bin/kubectl')

    def install_maven(self):
        self.install_application('maven')

    def install_minikube(self):
        urllib.request.urlretrieve(
            'https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64',
            '/usr/local/bin/minikube')
        execute(['chmod', '+x', '/usr/local/bin/minikube'])

    def install_mcollective(self):
        pass

    def install_nextcloud_client(self):
        pass

    def install_nodejs(self):
        pass

    def install_openvpn(self):
        pass

    def setup_openvpn(self):
        os.makedirs(os.environ['HOME'] + '/.openvpn')

    def install_simplescreenrecorder(self):
        self.install_application('simplescreenrecorder')

    def install_steam(self):
        pass

    def install_rpm(self):
        pass

    def install_terraform(self):
        pass

    def install_tmux(self):
        pass

    def install_vm_tools(self):
        pass

    def install_zsh(self):
        pass

    def setup_zsh(self):
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
        pass

    def update_os_repo(self):
        pass
