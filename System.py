import os
import re
import shutil
import subprocess
import sys
import tarfile
import urllib.request
from typing import List, AnyStr

pattern = re.compile('.*([0-9]+\.[0-9]+\.[0-9]+)$')


def execute(command: List[AnyStr], directory: AnyStr = os.path.dirname(os.path.realpath(__file__)), root=False):
    current_gid = os.getgid()
    current_uid = os.getuid()
    if root:
        os.setgid(0)
        os.setuid(0)
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
    if root:
        os.setgid(current_gid)
        os.setuid(current_uid)
    return {
        'code': proc.returncode,
        'output': output
    }


def download_file(url: AnyStr, downloaded_file: AnyStr):
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


class System:
    def install_application(self, application: AnyStr):
        self.install_applications([application])

    def install_applications(self, applications: List[AnyStr]):
        pass

    def install_chromium(self):
        pass

    def install_codecs(self):
        pass

    def setup_codecs(self):
        os.makedirs(os.environ['HOME'] + '/.config/aacs')
        urllib.request.urlretrieve('http://vlc-bluray.whoknowsmy.name/files/KEYDB.cfg',
                                   os.environ['HOME'] + '/.config/aacs')

    def install_curl(self):
        pass

    def install_discord(self):
        pass

    def install_docker(self):
        pass

    def setup_docker(self):
        pass

    def install_dropbox(self):
        pass

    def install_eclipse(self):
        pass

    def install_firefox(self):
        pass

    def install_firmware_updater(self):
        pass

    def install_git(self):
        pass

    def setup_git(self):
        execute(['git', 'config', '--global', 'user.name', 'Benjamin Sproule'])
        execute(['git', 'config', '--global', 'user.email', 'benjamin@benjaminsproule.com'])
        execute(['git', 'config', '--global', 'credential.helper', 'cache --timeout=86400'])
        os.makedirs(os.environ['HOME'] + '/.git/hooks', exist_ok=True)

    def install_gpg(self):
        pass

    def install_graphic_card_tools(self):
        pass

    def install_graphic_card_tools_laptop(self):
        pass

    def install_groovy_gradle(self):
        pass

    def install_intellij(self):
        pass

    def install_jdk(self):
        pass

    def set_java_home(self, file: AnyStr, jdk_path: AnyStr):
        pass

    def install_keepassxc(self):
        pass

    def install_kubectl(self):
        pass

    def install_lutris(self):
        pass

    def install_maven(self):
        pass

    def install_makemkv(self):
        pass

    def install_microcode(self):
        pass

    def install_minikube(self):
        pass

    def install_mkvtoolnix(self):
        pass

    def install_nextcloud_client(self):
        pass

    def install_nodejs(self):
        pass

    def install_powertop(self):
        pass

    def install_simplescreenrecorder(self):
        pass

    def install_slack(self):
        pass

    def install_spotify(self):
        pass

    def install_steam(self):
        pass

    def install_sweet_home_3d(self):
        pass

    def install_system_extras(self):
        pass

    def install_tlp(self):
        pass

    def install_tmux(self):
        pass

    def install_vm_tools(self):
        pass

    def install_vscode(self):
        pass

    def install_window_manager(self):
        pass

    def install_zsh(self):
        pass

    def setup_zsh(self):
        pass

    def set_development_shortcuts(self):
        pass

    def set_development_environment_settings(self):
        pass

    def set_free_dns_cron(self):
        pass

    def setup_power_saving_tweaks(self):
        pass

    def update_os(self):
        pass

    def update_os_repo(self):
        pass
