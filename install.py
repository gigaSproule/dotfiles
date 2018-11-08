#!/usr/bin/env python3
import getopt
import os
import sys

from Arch import Arch
from Kubuntu import Kubuntu
from Linux import execute, Linux
from Lubuntu import Lubuntu
from Mac import Mac
from Ubuntu import Ubuntu
from Windows import Windows
from Xubuntu import Xubuntu


def setup_user_bin():
    os.makedirs(os.environ['HOME'] + '/bin', exist_ok=True)
    os.makedirs(os.environ['HOME'] + '/.local/bin', exist_ok=True)


def install_required_dependencies():
    if execute(['pip3', '-V'])['code'] == 0:
        execute(['pip3', 'install', 'distro', 'lxml'])
    else:
        execute(['pip', 'install', 'distro'])


def get_system():
    if sys.platform == 'linux' or sys.platform == 'linux2':
        import distro
        if distro.name() == 'Ubuntu':
            current_desktop = os.environ['XDG_CURRENT_DESKTOP']
            if current_desktop == 'KDE':
                return Kubuntu()
            elif current_desktop == 'LXQt' or current_desktop == 'LXDE':
                return Lubuntu()
            elif current_desktop == 'XFCE':
                return Xubuntu()
            else:
                return Ubuntu()
        elif distro.name() == 'Arch':
            return Arch()
        else:
            return Linux()
    elif sys.platform == 'darwin':
        return Mac()
    elif sys.platform == 'win32':
        return Windows()
    else:
        EnvironmentError('Unknown operating system')


def main(argv):
    development = False
    personal = False
    media = False
    server = False
    vm = False

    try:
        opts, args = getopt.getopt(argv, 'hd:p:s:m:v:',
                                   ['help', 'development', 'personal', 'server', 'media', 'vm'])
        for opt, arg in opts:
            if opt in ('-h', '--help'):
                print('install.py [-d] [-p] [-r] [-s] [-m] [-v]')
                exit(0)
            elif opt in ('-d', '--development'):
                development = True
            elif opt in ('-p', '--personal'):
                personal = True
            elif opt in ('-m', '--media'):
                media = True
            elif opt in ('-s', '--server'):
                server = True
            elif opt in ('-v', '--vm'):
                vm = True
    except getopt.GetoptError:
        print('install.py [-d] [-r] [-s] [-m] [-v]')
        exit(1)

    setup_user_bin()

    install_required_dependencies()

    system = get_system()
    system.install_system_dependencies()
    system.update_os()
    print('Installing Distro Specific Extras')
    system.install_system_extras()

    if development:
        print('Installing Git')
        system.install_git()
        print('Installing Curl')
        system.install_curl()

        print('Installing Java')
        system.install_jdk()
        print('Installing Groovy & Gradle')
        system.install_groovy_gradle()
        print('Installing NodeJS')
        system.install_nodejs()

        print('Installing Atom')
        system.install_atom()
        print('Installing Chromium')
        system.install_chromium()
        print('Installing Deb')
        system.install_deb()
        print('Installing Docker')
        system.install_docker()
        print('Installing Eclipse')
        system.install_eclipse()
        print('Installing ecryptfs')
        system.install_ecryptfs()
        print('Installing GPG')
        system.install_gpg()
        print('Installing IntelliJ')
        system.install_intellij()
        print('Installing jq')
        system.install_jq()
        print('Installing KeepassXC')
        system.install_keepassxc()
        print('Installing Kubectl')
        system.install_kubectl()
        print('Installing Maven')
        system.install_maven()
        print('Installing Minikube')
        system.install_minikube()
        print('Installing mcollective')
        system.install_mcollective()
        print('Install NSS')
        system.install_nss()
        print('Installing OpenVPN')
        system.install_openvpn()
        print('Installing NordVPN')
        system.install_nordvpn()
        print('Installing RPM')
        system.install_rpm()
        print('Installing SimpleScreenRecorder')
        system.install_simplescreenrecorder()
        print('Installing Slack')
        system.install_slack()
        print('Installing terraform')
        system.install_terraform()
        print('Installing tmux')
        system.install_tmux()
        print('Installing ZSH')
        system.install_zsh()

        print('Setting development specific shortcuts')
        system.set_development_shortcuts()

        print('Setting development environment settings')
        system.set_development_environment_settings()

    if personal:
        print('Installing Chromium')
        system.install_chromium()
        print('Installing Discord')
        system.install_discord()
        print('Installing Dropbox')
        system.install_dropbox()
        print('Installing Codecs')
        system.install_codecs()
        print('Installing GPG')
        system.install_gpg()
        print('Installing KeepassXC')
        system.install_keepassxc()
        print('Installing Lutris')
        system.install_lutris()
        print('Installing Nextcloud Client')
        system.install_nextcloud_client()
        print('Installing OpenVPN')
        system.install_openvpn()
        print('Installing NordVPN')
        system.install_nordvpn()
        print('Installing Retroarch')
        system.install_retroarch()
        print('Installing Spotify')
        system.install_spotify()
        print('Installing Steam')
        system.install_steam()
        print('Installing SweetHome3D')
        system.install_sweet_home_3d()
        print('Installing tmux')
        system.install_tmux()
        print('Installing ZSH')
        system.install_zsh()

    if media:
        print('Installing MakeMKV')
        system.install_makemkv()
        print('Installing MKVToolNix')
        system.install_mkvtoolnix()

    if server:
        print('Installing Docker')
        system.install_docker()
        print('Installing Free DNS Cron')
        system.set_free_dns_cron()

    if vm:
        print('Installing VM Tools')
        system.install_vm_tools()


if __name__ == '__main__':
    main(sys.argv[1:])
