#!/usr/bin/env python3
import getopt
import glob
import os
import sys
from shutil import copyfile

import distro

from Arch import Arch
from Kubuntu import Kubuntu
from LinuxCommands import execute, LinuxCommands
from Lubuntu import Lubuntu
from Xubuntu import Xubuntu


def copy_symlink_files():
    files = glob.glob('**/*.symlink')
    for file in files:
        file_name = file.replace('.symlink', '').split('/')
        file_name = file_name[len(file_name) - 1]
        target = os.environ['HOME'] + '/.' + file_name
        copyfile(file, target)


def setup_user_bin():
    os.makedirs(os.environ['HOME'] + '/bin', exist_ok=True)
    os.makedirs(os.environ['HOME'] + '/.local/bin', exist_ok=True)


def install_required_dependencies():
    if execute(['pip3', '-V'])['code'] == 0:
        execute(['pip3', 'install', 'distro', 'lxml'])
    else:
        execute(['pip', 'install', 'distro'])


def system():
    if distro.name() == 'Ubuntu':
        current_desktop = os.environ['XDG_CURRENT_DESKTOP']
        if current_desktop == 'KDE':
            return Kubuntu()
        elif current_desktop == 'LXQt' or current_desktop == 'LXDE':
            return Lubuntu()
        elif current_desktop == 'XFCE':
            return Xubuntu()
    elif distro.name() == 'Arch':
        return Arch()
    else:
        return LinuxCommands()


def main(argv):
    development = False
    personal = False
    media = False
    server = False
    vm = False

    try:
        opts = getopt.getopt(argv, 'dpsm',
                                   ['development', 'personal', 'server', 'media'])
        for opt in opts:
            if opt in ('-h', '--help'):
                print('install.py [-d] [-p] [-r] [-s]')
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
        print('install.py [-d] [-r] [-s]')
        exit(1)

    setup_user_bin()

    install_required_dependencies()

    linux = system()
    linux.update_os()
    print('Installing Distro Specific Extras')
    linux.install_distro_extras()

    if development:
        print('Installing Git')
        linux.install_git()
        print('Installing Curl')
        linux.install_curl()

        print('Installing Java')
        linux.install_jdk()
        print('Installing Groovy & Gradle')
        linux.install_groovy_gradle()
        print('Installing NodeJS')
        linux.install_nodejs()

        print('Installing Atom')
        linux.install_atom()
        print('Installing Chromium')
        linux.install_chromium()
        print('Installing Deb')
        linux.install_deb()
        print('Installing Docker')
        linux.install_docker()
        print('Installing Eclipse')
        linux.install_eclipse()
        print('Installing ecryptfs')
        linux.install_ecryptfs()
        print('Installing IntelliJ')
        linux.install_intellij()
        print('Installing jq')
        linux.install_jq()
        print('Installing Kubectl')
        linux.install_kubectl()
        print('Installing Maven')
        linux.install_maven()
        print('Installing Minikube')
        linux.install_minikube()
        print('Installing mcollective')
        linux.install_mcollective()
        print('Installing OpenVPN')
        linux.install_openvpn()
        print('Installing SimpleScreenRecorder')
        linux.install_simplescreenrecorder()
        print('Installing RPM')
        linux.install_rpm()
        print('Installing terraform')
        linux.install_terraform()
        print('Installing tmux')
        linux.install_tmux()
        print('Installing ZSH')
        linux.install_zsh()

        print('Setting development specific shortcuts')
        linux.set_development_shortcuts()

    if personal:
        print('Installing Chromium')
        linux.install_chromium()
        print('Installing Dropbox')
        linux.install_dropbox()
        print('Installing Codecs')
        linux.install_codecs()
        print('Installing KeepassXC')
        linux.install_keepassxc()
        print('Installing Lutris')
        linux.install_lutris()
        print('Installing Nextcloud Client')
        linux.install_nextcloud_client()
        print('Installing OpenVPN')
        linux.install_openvpn()
        print('Installing Steam')
        linux.install_steam()
        print('Installing tmux')
        linux.install_tmux()
        print('Installing ZSH')
        linux.install_zsh()

    if media:
        print('Installing MakeMKV')
        linux.install_makemkv()
        print('Installing MKVToolNix')
        linux.install_mkvtoolnix()

    if server:
        print('Installing Docker')
        linux.install_docker()
        print('Installing Free DNS Cron')
        linux.set_free_dns_cron()

    if vm:
        print('Installing VM Tools')
        linux.install_vm_tools()

    copy_symlink_files()


if __name__ == '__main__':
    main(sys.argv[1:])
