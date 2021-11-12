#!/bin/bash

if [ ! -f "$HOME/.smbcredentials" ]; then
    echo "username=" >>$HOME/.smbcredentials
    echo "password=" >>$HOME/.smbcredentials
fi

sudo mkdir -p /media/nas/benjamin
sudo mkdir -p /media/nas/shared

sudo mount -t cifs //192.168.1.200/benjamin /media/nas/benjamin -o credentials=$HOME/.smbcredentials,uid=1000,gid=1000,vers=2.0
sudo mount -t cifs //192.168.1.200/shared /media/nas/shared -o credentials=$HOME/.smbcredentials,uid=1000,gid=1000,vers=2.0
