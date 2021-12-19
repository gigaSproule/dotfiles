#!/bin/bash
sudo mount -t cifs -o rw,uid=$(id -u),gid=$(id -g),credentials=/home/benjamin/.smbcredentials,vers=1.0 //192.168.1.200/benjamin /mnt/benjamin
sudo mount -t cifs -o rw,uid=$(id -u),gid=$(id -g),credentials=/home/benjamin/.smbcredentials,vers=1.0 //192.168.1.200/shared /mnt/shared
