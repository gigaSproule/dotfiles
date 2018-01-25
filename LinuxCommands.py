import os
import platform
import re
import shutil
import stat
import subprocess
import sys
import tarfile
import urllib.request
from distutils.version import StrictVersion
from shutil import copyfile

pattern = re.compile('.*([0-9]+\.[0-9]+\.[0-9]+)$')


def execute(command, directory='.'):
    proc = subprocess.Popen(command, stdin=subprocess.PIPE, stderr=subprocess.PIPE, stdout=subprocess.PIPE,
                            cwd=directory)
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


def download_file(url, downloaded_file):
    req = urllib.request.Request(
        url,
        data=None,
        headers={
            'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/35.0.1916.47 Safari/537.36'
        }
    )
    with urllib.request.urlopen(req) as response, open(downloaded_file, 'wb') as out_file:
        shutil.copyfileobj(response, out_file)


def untar_rename_root(src, dest):
    def members(tf):
        for member in tf.getmembers():
            if member.isreg():
                file_name = member.name.split('/')
                del file_name[0]
                file_name = '/'.join(file_name)
                member.name = file_name
                yield member

    with tarfile.open(src) as tar:
        tar.extractall(dest, members(tar))


def recursively_chmod(path, directory_permission=0o777, file_permission=0o777):
    os.chmod(path, directory_permission)
    for dirname, subdirs, files in os.walk(path):
        os.chmod(dirname, directory_permission)
        for f in files:
            os.chmod(os.path.join(dirname, f), file_permission)


class LinuxCommands:
    def install_application(self, application):
        self.install_applications([application])

    def install_applications(self, applications):
        pass

    def install_atom(self):
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

        with open('/etc/docker/daemon.json', 'w') as f:
            f.write('{\n'
                    '"dns": ["10.14.98.21", "10.14.98.22", "8.8.8.8"]\n'
                    '}')

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

    def set_java_home(self, file, jdk_path):
        with open(os.environ['HOME'] + '/' + file, 'a+') as f:
            contents = f.read()
            if 'JAVA_HOME' not in contents:
                f.write('export JAVA_HOME=%s' % jdk_path)

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

    def install_lutris(self):
        pass

    def install_maven(self):
        self.install_application('maven')

    def install_makemkv(self):
        pass

    def install_mcollective(self):
        pass

    def install_minikube(self):
        urllib.request.urlretrieve(
            'https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64',
            '/usr/local/bin/minikube')
        execute(['chmod', '+x', '/usr/local/bin/minikube'])

    def install_mkvtoolnix(self):
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
        pass

    def set_free_dns_cron(self):
        copyfile('dynIpUpdate.sh', '/opt/')
        execute(['crontab', '-l', '>', 'file;', 'echo', "'0 5 * * * /opt/dynIpUpdate.sh.sh >/dev/null 2>&1'", '>>',
                 'file;', 'crontab', 'file'])
        os.remove('file')

    def update_os(self):
        pass

    def update_os_repo(self):
        pass
