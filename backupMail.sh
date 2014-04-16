#!/bin/bash

host=$1
share=$2

sudo mount -t cifs //$host/$share /mnt/backup -o credentials=$HOME/passwd
sudo tar -zcvf /mnt/backup/Maildir.tar.gz $HOME/Maildir
sudo umount /mnt/backup
