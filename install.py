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
        target = os.environ['HOME'] + "/." + file_name
        copyfile(file, target)


def setup_user_bin():
    os.makedirs(os.environ['HOME'] + '/bin', exist_ok=True)
    os.makedirs(os.environ['HOME'] + '/.local/bin', exist_ok=True)


def install_distro():
    if execute(['pip3', '-V'])['code'] == 0:
        execute(['pip3', 'install', 'distro'])
    else:
        execute(['pip', 'install', 'distro'])


def system():
    if distro.name() == 'Ubuntu':
        current_desktop = os.environ['XDG_CURRENT_DESKTOP']
        if current_desktop == 'Plasma' or current_desktop == 'KDE':
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
    server = False
    vm = False

    try:
        opts, args = getopt.getopt(argv, 'dps',
                                   ['development', 'personal', 'server'])
        for opt, arg in opts:
            if opt in ('-h', '--help'):
                print('install.py [-d] [-p] [-r] [-s]')
                exit(0)
            elif opt in ('-d', '--development'):
                development = True
            elif opt in ('-p', '--personal'):
                personal = True
            elif opt in ('-s', '--server'):
                server = True
            elif opt in ('-v', '--vm'):
                vm = True
    except getopt.GetoptError:
        print('install.py [-d] [-r] [-s]')
        exit(1)

    setup_user_bin()

    install_distro()

    linux = system()
    linux.update_os()
    linux.install_distro_extras()

    if development:
        linux.install_git()
        linux.install_curl()

        linux.install_jdk()
        linux.install_groovy_gradle()
        linux.install_nodejs()

        linux.install_chromium()
        linux.install_deb()
        linux.install_docker()
        linux.install_eclipse()
        linux.install_ecryptfs()
        linux.install_intellij()
        linux.install_jq()
        linux.install_keepassxc()
        linux.install_kubectl()
        linux.install_maven()
        linux.install_minikube()
        linux.install_mcollective()
        linux.install_openvpn()
        linux.install_simplescreenrecorder()
        linux.install_rpm()
        linux.install_terraform()
        linux.install_tmux()
        linux.install_zsh()

        linux.set_development_shortcuts()

    if personal:
        linux.install_chromium()
        linux.install_dropbox()
        linux.install_codecs()
        linux.install_keepassxc()
        linux.install_nextcloud_client()
        linux.install_openvpn()
        linux.install_steam()
        linux.install_tmux()
        linux.install_zsh()

    if server:
        linux.install_docker()
        linux.set_free_dns_cron()

    if vm:
        linux.install_vm_tools()

    copy_symlink_files()


if __name__ == '__main__':
    main(sys.argv[1:])
