#!/bin/bash

if [ ! -d "$HOME/.smbcredentials" ]
then
	echo "username=" >> $HOME/.smbcredentials
	echo "password=" >> $HOME/.smbcredentials
fi

sudo mkdir -p /media/nas/benjamin
sudo mkdir -p /media/nas/shared

sudo mount -t cifs //MYBOOKLIVEDUO/benjamin /media/nas/benjamin -o credentials=$HOME/.smbcredentials,uid=1000,gid=1000
sudo mount -t cifs //MYBOOKLIVEDUO/shared /media/nas/shared -o credentials=$HOME/.smbcredentials,uid=1000,gid=1000
