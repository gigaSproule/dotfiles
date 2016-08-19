#!/bin/bash

if [[ ! -d "/media/nas" ]]
then
  sudo mkdir /media/nas
fi

if [[ ! -d "/media/nas/benjamin" ]]
then
  sudo mkdir /media/nas/benjamin
fi

if [[ ! -d "/media/nas/shared" ]]
then
  sudo mkdir /media/nas/shared
fi

sudo mount -t cifs //MYBOOKLIVEDUO/benjamin /media/nas/benjamin -o credentials=$HOME/.smbcredentials,uid=1000,gid=1000
sudo mount -t cifs //MYBOOKLIVEDUO/shared /media/nas/shared -o credentials=$HOME/.smbcredentials,uid=1000,gid=1000

