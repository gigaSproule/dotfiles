import ctypes
import os
from typing import AnyStr, List

from System import System


class Windows(System):

    def execute(self, command: List[AnyStr], directory: AnyStr = os.path.dirname(os.path.realpath(__file__)),
                super_user: bool = True):
        return super().execute(command, directory, super_user)

    def execute_powershell(self, command: List[AnyStr], directory: AnyStr = os.path.dirname(os.path.realpath(__file__)),
                           super_user: bool = True):
        return self.execute(['C:\Windows\System32\WindowsPowerShell\\v1.0\powershell.exe'] + command, directory,
                            super_user)

    def is_super_user(self):
        return ctypes.windll.shell32.IsUserAnAdmin() == 1

    def install_applications(self, applications: List[AnyStr]):
        command = ['choco', 'install', '--yes']
        command.extend(applications)
        self.execute(command)

    def install_blender(self):
        self.install_application('blender')

    def install_conemu(self):
        self.install_application('conemu')

    def install_cryptomator(self):
        self.install_application('cryptomator')

    def install_curl(self):
        self.install_application('curl')

    def install_discord(self):
        self.install_application('discord')

    def install_docker(self):
        self.install_application('docker-desktop')
        self.setup_docker()

    def setup_docker(self):
        self.execute_powershell(['Install-Module', '-Name',
                                 'DockerCompletion', '-Force'])
        self.execute_powershell(['Import-Module', 'DockerCompletion'])
        with open('C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\profile.ps1', 'w') as f:
            f.writelines(['Import-Module DockerCompletion'])

    def install_dropbox(self):
        self.install_application('dropbox')

    def install_eclipse(self):
        self.install_application('eclipse')

    def install_epic_games(self):
        self.install_application('epicgameslauncher')

    def install_firefox(self):
        self.install_application('firefox')

    def install_gog_galaxy(self):
        self.install_application('goggalaxy')

    def install_google_chrome(self):
        self.install_application('googlechrome')

    def install_google_cloud_sdk(self):
        self.install_application('gcloudsdk')

    def install_google_drive(self):
        self.install_application('google-drive-file-stream')

    def install_git(self):
        self.install_application('git')
        self.setup_git()

    def setup_git(self):
        super().setup_git()
        self.execute(['git', 'config', '--system', 'core.longpaths', 'true'])
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
        self.install_application('nvm')
        with open('C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\profile.ps1', 'w') as f:
            f.writelines([
                'function callnvm() {',
                '   # Always use argument version if there is one',
                '   $versionDesired = $args[0]',
                '   if (($versionDesired -eq "" -Or $versionDesired -eq $null) -And (Test-Path .nvmrc -PathType Any)) {',
                '       # if we have an nvmrc and no argument supplied, use the version in the file',
                '       $versionDesired = $(Get-Content .nvmrc).replace( \'v\', \'\' );',
                '   }',
                '   Write-Host "Requesting version \'$($versionDesired)\'"',
                '   if ($versionDesired -eq "") {',
                '       Write-Host "A node version needs specifying as an argument if there is no .nvmrc"',
                '   } else {',
                '       $response = nvm use $versionDesired;',
                '       if ($response -match \'is not installed\') {',
                '           if ($response -match \'64-bit\') {',
                '               $response = nvm install $versionDesired x64',
                '           } else {',
                '               $response = nvm install $versionDesired x86',
                '           }',
                '           Write-Host $response',
                '           $response = nvm use $versionDesired;',
                '       }',
                '       Write-Host $response',
                '   }',
                '}',
                'Set-Alias nvmu -value "callnvm"'
            ])
        self.execute_powershell(['refreshenv'])
        self.execute(['nvm', 'install', 'latest'])
        output = self.execute(['nvm', 'list'])['output']
        for output_version in output.split('\n'):
            if output_version != '':
                self.execute(['nvm', 'use', output_version.replace(' ', '')])
                break
        self.execute_powershell(['refreshenv'])
        self.execute_powershell(['npm', 'install', '--global', 'yarn'])

    def install_nordvpn(self):
        self.install_application('nordvpn')

    def install_nvidia_tools(self):
        self.install_application('geforce-experience')

    def install_obs_studio(self):
        self.install_application('obs-studio')

    def install_origin(self):
        self.install_application('origin')

    def install_python(self):
        self.install_application('python')

    def install_rust(self):
        self.install_application('rustup.install')

    def install_slack(self):
        self.install_application('slack')

    def install_spotify(self):
        self.install_application('spotify')

    def install_steam(self):
        self.install_application('steam')

    def install_sweet_home_3d(self):
        self.install_application('sweet-home-3d')

    def install_system_extras(self):
        self.execute_powershell(['Set-ExecutionPolicy', 'Unrestricted'])
        self.download_file('https://chocolatey.org/install.ps1', 'install.ps1')
        self.execute_powershell(['iex', '.\install.ps1'])
        self.execute_powershell(['Install-PackageProvider', '-Name', 'NuGet',
                                 '-MinimumVersion', '2.8.5.201', '-Force'])
        self.execute_powershell(['Import-Module', '"$env:ProgramData\chocolatey\helpers\chocolateyInstaller.psm1";',
                                 'Update-SessionEnvironment'])
        self.execute(['REG', 'ADD', 'HKLM\SYSTEM\CurrentControlSet\Control\FileSystem',
                      '/v', 'LongPathsEnabled', '/t', 'REG_DWORD', '/d', '1', '/f'])

    def install_vim(self):
        self.install_application('vim')

    def install_vlc(self):
        self.install_application('vlc')

    def install_vscode(self):
        self.install_application('vscode')

    def install_wget(self):
        self.install_application('wget')

    def update_os(self):
        self.execute(['choco', 'upgrade', 'all'])
