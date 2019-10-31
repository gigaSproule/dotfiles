import ctypes
from typing import AnyStr, List

from System import System


class Windows(System):

    def is_super_user(self):
        return ctypes.windll.shell32.IsUserAnAdmin() == 1

    def install_applications(self, applications: List[AnyStr]):
        command = ['choco', 'install', '--yes']
        command.extend(applications)
        self.execute(command, super_user=True)

    def install_curl(self):
        self.install_application('curl')

    def install_discord(self):
        self.install_application('discord')

    def install_docker(self):
        self.install_application('docker-desktop')

    def install_dropbox(self):
        self.install_application('dropbox')

    def install_eclipse(self):
        self.install_application('eclipse')

    def install_firefox(self):
        self.install_application('firefox')

    def install_git(self):
        self.install_application('git')

    def install_gpg(self):
        self.install_application('gpg4win')

    def install_gradle(self):
        self.install_application('gradle')

    def install_groovy(self):
        self.install_application('groovy')

    def install_intellij(self):
        self.install_application('intellijidea-ultimate')

    def install_jdk(self):
        self.install_application('adoptopenjdk')

    def install_keepassxc(self):
        self.install_application('keepassxc')

    def install_kubectl(self):
        self.install_application('kubernetes-cli')

    def install_maven(self):
        self.install_application('maven')

    def install_makemkv(self):
        self.install_application('makemkv')

    def install_minikube(self):
        self.install_application('minikube')

    def install_mkvtoolnix(self):
        self.install_application('mkvtoolnix')

    def install_nextcloud_client(self):
        self.install_application('nextcloud-client')

    def install_nodejs(self):
        self.install_application('nodejs')

    def install_nordvpn(self):
        self.install_application('nordvpn')

    def install_slack(self):
        self.install_application('slack')

    def install_spotify(self):
        self.install_application('spotify')

    def install_steam(self):
        self.install_application('steam')

    def install_sweet_home_3d(self):
        self.install_application('sweet-home-3d')

    def install_system_extras(self):
        self.download_file('https://chocolatey.org/install.ps1', 'install.ps1')
        self.execute(['iex', 'install.ps1'], super_user=True)

    def install_vscode(self):
        self.install_application('vscode')

    def update_os(self):
        self.update_os_repo()
        self.execute(['choco', 'upgrade', 'all'], super_user=True)

    def update_os_repo(self):
        self.execute(['choco', 'update'], super_user=True)
