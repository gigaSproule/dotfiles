import ctypes
from typing import AnyStr, List

from System import System


class Windows(System):

    def is_super_user(self):
        return ctypes.windll.shell32.IsUserAnAdmin() == 1

    def install_applications(self, applications: List[AnyStr]):
        command = ['choco', 'install', '--yes']
        command.extend(applications)
        self.execute(command)

    def install_curl(self):
        self.install_application('curl')

    def install_discord(self):
        self.install_application('discord')

    def install_docker(self):
        self.install_application('docker-desktop')
        self.setup_docker()

    def setup_docker(self):
        self.execute(['Install-Module', '-Name', 'DockerCompletion', '-Confirm'])
        self.execute(['Import-Module', 'DockerCompletion'])
        with open('C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\profile.ps1', 'w') as f:
            f.writelines(['Import-Module DockerCompletion'])

    def install_dropbox(self):
        self.install_application('dropbox')

    def install_eclipse(self):
        self.install_application('eclipse')

    def install_firefox(self):
        self.install_application('firefox')

    def install_google_cloud_sdk(self):
        self.install_application('gcloudsdk')

    def install_git(self):
        self.install_application('git')

    def setup_git(self):
        super().setup_git()
        self.install_application('poshgit')

    def install_gimp(self):
        self.install_application('gimp')

    def install_gpg(self):
        self.install_application('gpg4win')

    def install_gradle(self):
        self.install_application('gradle')

    def install_groovy(self):
        self.install_application('groovy')

    def install_handbrake(self):
        self.install_application('handbrake')

    def install_inkscape(self):
        self.install_application('inkscape')

    def install_intellij(self):
        self.install_application('intellijidea-ultimate')

    def install_jdk(self):
        self.install_application('adoptopenjdk')

    def install_keepassxc(self):
        self.install_application('keepassxc')

    def install_kubectl(self):
        self.install_application('kubernetes-cli')

    def install_helm(self):
        self.install_application('kubernetes-helm')

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
        self.install_applications(['nodejs', 'yarn'])

    def install_nordvpn(self):
        self.install_application('nordvpn')

    def install_nvidia_tools(self):
        self.install_application('nvidia-display-driver')

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
        self.execute(['iex', 'install.ps1'])
        self.execute(['Install-PackageProvider', '-Name', 'NuGet', '-MinimumVersion', '2.8.5.201', '-Force'])

    def install_vim(self):
        self.install_application('vim')

    def install_vlc(self):
        self.install_application('vlc')

    def install_vscode(self):
        self.install_application('vscode')

    def update_os(self):
        self.execute(['choco', 'upgrade', 'all'])
