#!/bin/bash

host=$1
share=$2
backup=$3
credentials=$4

sudo mount -t cifs //$host/$share $backup -o credentials=$credentials
sudo tar -zcvf $backup/owncloud.tar.gz /var/www/owncloud
sudo umount $backup
