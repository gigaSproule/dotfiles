#!/usr/bin/env python3
import getopt
import glob
import os
import sys
from shutil import copyfile

from LinuxCommands import LinuxCommands
from distroinstaller import DistroInstaller


def copy_symlink_files():
    files = glob.glob('**/*.symlink')
    for file in files:
        file_name = file.replace('.symlink', '').split('/')
        file_name = file_name[len(file_name) - 1]
        target = os.environ['HOME'] + "/." + file_name
        copyfile(file, target)


def create_home_bin():
    os.makedirs(os.environ['HOME'] + '/bin')


def main(argv):
    development = False
    personal = False
    server = False

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
    except getopt.GetoptError:
        print('install.py [-d] [-r] [-s]')
        exit(1)

    create_home_bin()

    distro_installer = DistroInstaller()
    distro_installer.install()

    linux = LinuxCommands()
    linux.update_os()
    linux.install_distro_extras()

    if development:
        linux.install_git()

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
        linux.install_maven()
        linux.install_mcollective()
        linux.install_openvpn()
        linux.install_simplescreenrecorder()
        linux.install_rpm()
        linux.install_terraform()
        linux.install_tmux()
        linux.install_zsh()

        linux.set_development_shortcuts()

        print('Remove keyboard shortcuts under Navigation for ctrl + alt + left/right')
        print('Remove keyboard shortcut under System for ctrl + alt + l')
        print('Remove keyboard shortcuts under Windows for ctrl + alt + s, alt + f7')

    if personal:
        linux.install_chromium()
        linux.install_dropbox()
        linux.install_dvd_decoders()
        linux.install_keepassxc()
        linux.install_nextcloud_client()
        linux.install_openvpn()
        linux.install_steam()
        linux.install_tmux()
        linux.install_zsh()

    if server:
        linux.install_docker()
        linux.set_free_dns_cron()

    copy_symlink_files()


if __name__ == '__main__':
    main(sys.argv[1:])
