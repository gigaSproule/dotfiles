import os
import shutil
import subprocess
import sys
import tarfile
import urllib.request
from typing import List, AnyStr, Callable, Any


class System:

    def copy_file(self, source: AnyStr, destination: AnyStr, super_user: bool = False):
        if super_user:
            return self.run_as_super_user(shutil.copyfile, source, destination)
        else:
            return shutil.copyfile(source, destination)

    def delete_directory(self, directory: AnyStr, super_user: bool = False):
        if super_user:
            return self.run_as_super_user(shutil.rmtree, directory)
        else:
            return shutil.rmtree(directory)

    def delete_file(self, file: AnyStr, super_user: bool = False):
        if super_user:
            return self.run_as_super_user(os.remove, file)
        else:
            return os.remove(file)

    def download_file(self, url: AnyStr, downloaded_file: AnyStr, super_user: bool = False):
        if super_user:
            self.run_as_super_user(self._download_file, url, downloaded_file)
        else:
            self._download_file(url, downloaded_file)

    def _download_file(self, url: AnyStr, downloaded_file: AnyStr):
        req = urllib.request.Request(
            url,
            data=None,
            headers={
                'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/35.0.1916.47 Safari/537.36'
            }
        )
        with urllib.request.urlopen(req) as response, open(downloaded_file, 'wb') as out_file:
            shutil.copyfileobj(response, out_file)

    def execute(self, command: List[AnyStr], directory: AnyStr = os.path.dirname(os.path.realpath(__file__)),
                super_user: bool = False):
        if super_user:
            return self.run_as_super_user(self._execute, command, directory)
        else:
            return self._execute(command, directory)

    def _execute(self, command: List[AnyStr], directory: AnyStr):
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

    def is_super_user(self):
        pass

    def make_directory(self, directory: AnyStr, super_user=True):
        if super_user:
            self.run_as_super_user(os.makedirs, directory, exist_ok=True)
        else:
            os.makedirs(directory, exist_ok=True)

    def recursively_chmod(self, path, directory_permission=0o777, file_permission=0o777, super_user: bool = False):
        if super_user:
            self.run_as_super_user(self._recursively_chmod, path, directory_permission, file_permission)
        else:
            self._recursively_chmod(path, directory_permission, file_permission)

    def _recursively_chmod(self, path: AnyStr, directory_permission: int, file_permission: int):
        os.chmod(path, directory_permission)
        for dirname, subdirs, files in os.walk(path):
            os.chmod(dirname, directory_permission)
            for f in files:
                os.chmod(os.path.join(dirname, f), file_permission)

    def run_as_super_user(self, fun: Callable[[Any], Any], *args, **kwargs):
        self.set_user_as_super_user()
        output = fun(*args, **kwargs)
        self.set_user_as_normal_user()
        return output

    def set_user_as_super_user(self):
        pass

    def set_user_as_normal_user(self):
        pass

    def untar_rename_root(self, src: AnyStr, dest: AnyStr):
        def members(tf):
            for member in tf.getmembers():
                if member.isreg():
                    file_name = member.name.split('/')
                    del file_name[0]
                    file_name = '/'.join(file_name)
                    member.name = file_name
                    yield member

        with tarfile.open(src) as tar_file:
            tar_file.extractall(dest, members(tar_file))

    def install_application(self, application: AnyStr):
        self.install_applications([application])

    def install_applications(self, applications: List[AnyStr]):
        pass

    def install_bluetooth(self):
        pass

    def install_chrome(self):
        pass

    def install_codecs(self):
        self.setup_codecs()

    def setup_codecs(self):
        self.make_directory(os.environ['HOME'] + '/.config/aacs')
        urllib.request.urlretrieve('http://vlc-bluray.whoknowsmy.name/files/KEYDB.cfg',
                                   os.environ['HOME'] + '/.config/aacs')

    def install_curl(self):
        pass

    def install_discord(self):
        pass

    def install_docker(self):
        self.setup_docker()

    def setup_docker(self):
        pass

    def install_dropbox(self):
        pass

    def install_eclipse(self):
        self.setup_eclipse()

    def setup_eclipse(self):
        pass

    def install_firefox(self):
        pass

    def install_firmware_updater(self):
        pass

    def install_google_cloud_sdk(self):
        pass

    def install_git(self):
        self.install_git()

    def setup_git(self):
        self.execute(['git', 'config', '--global', 'user.name', 'Benjamin Sproule'])
        self.execute(['git', 'config', '--global', 'user.email', 'benjamin@benjaminsproule.com'])
        self.execute(['git', 'config', '--global', 'credential.helper', 'cache --timeout=86400'])
        self.make_directory(os.environ['HOME'] + '/.git/hooks')

    def install_gpg(self):
        pass

    def install_gradle(self):
        pass

    def install_graphic_card_tools(self):
        # if nvidia
        self.install_nvidia_tools()
        # else

    def install_graphic_card_laptop_tools(self):
        # if nvidia
        self.install_nvidia_laptop_tools()
        # else

    def install_groovy(self):
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

    def install_helm(self):
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

    def install_nordvpn(self):
        pass

    def install_nvidia_tools(self):
        pass

    def install_nvidia_laptop_tools(self):
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

    def install_vim(self):
        pass

    def install_vm_tools(self):
        pass

    def install_vscode(self):
        pass

    def install_wifi(self):
        pass

    def install_window_manager(self):
        pass

    def install_wine(self):
        pass

    def install_zsh(self):
        self.setup_zsh()

    def setup_zsh(self):
        pass

    def set_development_shortcuts(self):
        pass

    def set_development_environment_settings(self):
        pass

    def setup_power_saving_tweaks(self):
        pass

    def update_os(self):
        pass

    def update_os_repo(self):
        pass
