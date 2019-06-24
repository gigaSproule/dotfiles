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
        execute(['pip', 'install', 'distro', 'lxml'])


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
    personal = False
    docker = False
    laptop = False
    server = False
    vm = False

    try:
        opts, args = getopt.getopt(argv, 'hd:p:s:v:',
                                   ['help', 'docker', 'personal', 'server', 'vm'])
        for opt, arg in opts:
            if opt in ('-h', '--help'):
                print('install.py [-d] [-p] [-r] [-s] [-v]')
                exit(0)
            elif opt in ('-d', '--docker'):
                docker = True
            elif opt in ('-p', '--personal'):
                personal = True
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
    print('Installing Distro Specific Extras')
    system.install_system_extras()
    system.update_os()

    if personal:
        print('Installing GNOME')
        system.install_window_manager()
        print('Installing Graphic Card Tools')
        system.install_graphic_card_tools()

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

        print('Installing Chromium')
        system.install_chromium()
        print('Installing Codecs')
        system.install_codecs()
        print('Installing Discord')
        system.install_discord()
        print('Installing Dropbox')
        system.install_dropbox()
        print('Installing Eclipse')
        system.install_eclipse()
        print('Installing Firefox')
        system.install_firefox()
        print('Installing GPG')
        system.install_gpg()
        print('Installing IntelliJ')
        system.install_intellij()
        print('Installing KeepassXC')
        system.install_keepassxc()
        print('Installing Lutris')
        system.install_lutris()
        print('Installing MakeMKV')
        system.install_makemkv()
        print('Installing Maven')
        system.install_maven()
        print('Installing MKVToolNix')
        system.install_mkvtoolnix()
        print('Installing Nextcloud Client')
        system.install_nextcloud_client()
        print('Installing NordVPN')
        system.install_nordvpn()
        print('Installing SimpleScreenRecorder')
        system.install_simplescreenrecorder()
        print('Installing Slack')
        system.install_slack()
        print('Installing Spotify')
        system.install_spotify()
        print('Installing Steam')
        system.install_steam()
        print('Installing SweetHome3D')
        system.install_sweet_home_3d()
        print('Installing tmux')
        system.install_tmux()
        print('Installing VSCode')
        system.install_vscode()
        print('Installing ZSH')
        system.install_zsh()

        print('Setting development specific shortcuts')
        system.set_development_shortcuts()

        print('Setting development environment settings')
        system.set_development_environment_settings()

    if docker:
        print('Installing Docker')
        system.install_docker()
        print('Installing Kubectl')
        system.install_kubectl()
        print('Installing Minikube')
        system.install_minikube()

    if laptop:
        print('Installing Graphic Card Tools')
        system.install_graphic_card_tools()
        print('Installing Graphics Card Tools for Laptop')
        system.install_graphic_card_tools_laptop()
        print('Installing Microcode')
        system.install_microcode()
        print('Installing Powertop')
        system.install_powertop()
        print('Installing TLP')
        system.install_tlp()
        print('Setup power saving tweaks')
        system.setup_power_saving_tweaks()
        print('Installing FWUPD')
        system.install_firmware_updater()

    if server:
        print('Installing Docker')
        system.install_docker()

    if vm:
        print('Installing VM Tools')
        system.install_vm_tools()


if __name__ == '__main__':
    main(sys.argv[1:])
