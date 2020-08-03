#!/usr/bin/env python3
import getopt
import os
import sys

from Arch import Arch
from Kubuntu import Kubuntu
from Linux import Linux
from Lubuntu import Lubuntu
from Mac import Mac
from System import System
from Ubuntu import Ubuntu
from Windows import Windows
from Xubuntu import Xubuntu


def install_required_dependencies(application):
    if System().execute(['pip3', '-V'])['code'] == 0:
        System().execute(['pip3', 'install', application])
    else:
        System().execute(['pip', 'install', application])


def get_system():
    if sys.platform == 'linux':
        install_required_dependencies('distro')
        import distro
        if distro.name() == 'Ubuntu':
            current_desktop = os.environ['XDG_CURRENT_DESKTOP']
            if current_desktop == 'KDE':
                print('Detected Kubuntu')
                return Kubuntu()
            elif current_desktop == 'LXQt' or current_desktop == 'LXDE':
                print('Detected Lubuntu')
                return Lubuntu()
            elif current_desktop == 'XFCE':
                print('Detected Xubuntu')
                return Xubuntu()
            else:
                print('Detected Ubuntu')
                return Ubuntu()
        elif distro.name() == 'Arch Linux':
            print('Detected Arch')
            return Arch()
        else:
            return Linux()
    elif sys.platform == 'darwin':
        print('Detected Mac')
        return Mac()
    elif sys.platform == 'win32' or sys.platform == 'cygwin':
        print('Detected Windows')
        return Windows()
    else:
        EnvironmentError('Unknown operating system')


def main(argv):
    browsers = development = docker = gaming = gcp = laptop = personal = recording = ripping = video = vm = vpn = False

    try:
        opts, args = getopt.getopt(argv, 'hbdcgpv',
                                   ['help', 'browsers', 'development', 'docker', 'gaming', 'gcp', 'personal',
                                    'recording', 'ripping', 'video', 'vm', 'vpn'])
        if len(opts) == 0:
            print('No options provided')
            print_help()
            exit(1)
        for opt, arg in opts:
            if opt in ('-h', '--help'):
                print_help()
                exit(0)
            elif opt in ('-b', '--browsers'):
                browsers = True
            elif opt in ('-d', '--development'):
                development = True
            elif opt in ('-c', '--docker'):
                docker = True
            elif opt in ('-g', '--gaming'):
                gaming = True
            elif opt in ('-g', '--gcp'):
                gcp = True
            elif opt in ('-p', '--personal'):
                personal = True
            elif opt in ('-r', '--recording'):
                recording = True
            elif opt in ('-r', '--ripping'):
                ripping = True
            elif opt in ('-v', '--video'):
                video = True
            elif opt in ('-v', '--vm'):
                vm = True
            elif opt in ('-v', '--vpn'):
                vpn = True
    except getopt.GetoptError as error:
        print(str(error))
        print_help()
        exit(1)

    system = get_system()
    if not system.is_super_user():
        print('This needs to be run as root, so the applications can be installed.')
        exit(1)

    system.setup_user_bin()

    print('Installing Distro Specific Extras')
    system.install_system_extras()
    system.update_os()

    print('Installing Window Manager')
    system.install_window_manager()
    print('Installing Graphic Card Tools')
    system.install_graphic_card_tools()

    print('Installing Curl')
    system.install_curl()

    print('Installing KeepassXC')
    system.install_keepassxc()

    print('Installing tmux')
    system.install_tmux()
    print('Installing Vim')
    system.install_vim()
    print('Installing Wget')
    system.install_wget()
    print('Installing ZSH')
    system.install_zsh()

    if browsers:
        print('Installing Chrome')
        system.install_chrome()
        print('Installing Firefox')
        system.install_firefox()

    if development:
        print('Installing Android Studio')
        system.install_android_studio()
        print('Installing Eclipse')
        # system.install_eclipse()
        print('Installing Gradle')
        system.install_gradle()
        print('Installing Git')
        system.install_git()
        print('Installing Groovy')
        system.install_groovy()
        print('Installing IntelliJ')
        system.install_intellij()
        print('Installing Java')
        system.install_jdk()
        print('Installing Maven')
        system.install_maven()
        print('Installing NodeJS')
        system.install_nodejs()
        print('Installing Python')
        system.install_python()
        print('Installing Rust')
        system.install_rust()
        print('Installing Slack')
        system.install_slack()
        print('Installing VSCode')
        system.install_vscode()
        print('Installing Xcode')
        system.install_xcode()
        print('Setting development specific shortcuts')
        system.set_development_shortcuts()
        print('Setting development environment settings')
        system.set_development_environment_settings()

    if docker:
        print('Installing Docker')
        system.install_docker()
        print('Installing Kubectl')
        system.install_kubectl()
        print('Installing Helm')
        system.install_helm()
        print('Installing Minikube')
        system.install_minikube()

    if gaming:
        print('Installing Discord')
        system.install_discord()
        print('Installing Lutris')
        system.install_lutris()
        print('Installing Steam')
        system.install_steam()
        print('Installing Wine')
        system.install_wine()

    if gcp:
        print('Installing Google Cloud SDK')
        system.install_google_cloud_sdk()

    if laptop:
        print('Installing Bluetooth')
        system.install_bluetooth()
        print('Installing FWUPD')
        system.install_firmware_updater()
        print('Installing Graphic Card Tools')
        system.install_graphic_card_tools()
        print('Installing Graphics Card Tools for Laptop')
        system.install_graphic_card_laptop_tools()
        print('Installing Microcode')
        system.install_microcode()
        print('Installing Powertop')
        system.install_powertop()
        print('Installing TLP')
        system.install_tlp()
        print('Install WiFi')
        system.install_wifi()
        print('Setup power saving tweaks')
        system.setup_power_saving_tweaks()

    if personal:
        print('Installing Dropbox')
        system.install_dropbox()
        print('Installing GPG')
        system.install_gpg()
        print('Installing Nextcloud Client')
        system.install_nextcloud_client()
        print('Installing Spotify')
        system.install_spotify()
        print('Installing SweetHome3D')
        system.install_sweet_home_3d()
        print('Installing themes')
        system.install_themes()

    if recording:
        print('Installing OBS Studio')
        system.install_obs_studio()

    if ripping:
        print('Installing Handbrake')
        system.install_handbrake()
        print('Installing MakeMKV')
        system.install_makemkv()
        print('Installing MKVToolNix')
        system.install_mkvtoolnix()

    if video:
        print('Installing Codecs')
        system.install_codecs()
        print('Installing VLC')
        system.install_vlc()

    if vm:
        print('Installing VM Tools')
        system.install_vm_tools()

    if vpn:
        print('Installing NordVPN')
        system.install_nordvpn()


def print_help():
    print('install.py [-c] [-d] [-g] [-r] [-s] [-m] [-v]')


if __name__ == '__main__':
    main(sys.argv[1:])
