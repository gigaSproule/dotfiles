#!/bin/bash

detectedDistro="Unknown"
regExpLsbInfo="Description:[[:space:]]*([^ ]*)"
regExpLsbFile="/etc/(.*)[-_]"

if [ `which lsb_release 2>/dev/null` ]; then       # lsb_release available
   lsbInfo=`lsb_release -d`
   if [[ $lsbInfo =~ $regExpLsbInfo ]]; then
      detectedDistro=${BASH_REMATCH[1]}
   else
      echo "??? Should not occur: Don't find distro name in lsb_release output ???"
      exit 1
   fi

else                                               # lsb_release not available
   etcFiles=`ls /etc/*[-_]{release,version} 2>/dev/null`
   for file in $etcFiles; do
      if [[ $file =~ $regExpLsbFile ]]; then
         detectedDistro=${BASH_REMATCH[1]}
         break
      else
         echo "??? Should not occur: Don't find any etcFiles ???"
         exit 1
      fi
   done
fi

detectedDistro=`echo $detectedDistro | tr "[:upper:]" "[:lower:]"`

case $detectedDistro in
	suse) 	detectedDistro="opensuse" ;;
        linux)	detectedDistro="linuxmint" ;;
esac

# make sure all programs are installed from repos
if [[ $detectedDistro == "ubuntu" ]]
then
	sudo apt-get update
	sudo apt-get -y install openjdk-6-jre php5 apache2 libapache2-mod-php5 mysql-server php5-mysql openssh-server vsftpd xbmc xorg perl rsync screen wget linux-sound-base alsa-base alsa-utils alsa-tools lynx cmus caca-utils finch
elif [[ $detectedDistro == "debian" ]]
then
	sudo aptitude update
	sudo aptitude install -y vim vsftpd openssh-* wget perl rsync screen openjdk-6-jre
elif [[ $detectedDistro == "fedora" ]]
then
	sudo yum install -y vim vsftpd openssh-* wget perl rsync screen java-1.7.0-openjdk
fi

# setup apache
if [[ $detectedDistro == "ubuntu" ]]
then
	wget -O default http://www.britintel.co.uk/files/default_server
	sudo mv default /etc/apache2/sites-available/
	sudo /etc/init.d/apache2 restart
fi

# setup portfolio site
if [[ $detectedDistro == "ubuntu" ]]
then
	mkdir public_html
	cd public_html
	sudo wget -r -l1 --no-parent http://bsproule.britintel.co.uk
	cd bsproule.britintel.co.uk
	sudo mv * ..
	cd ..
	sudo rm -rf bsproule.britintel.co.uk
	sudo chown benjamin:benjamin *
	cd $HOME
	sudo /etc/init.d/apache2 restart
fi

# install nodejs
if [[ $detectedDistro == "ubuntu" ]]
then
	sudo apt-get install python-software-properties
	sudo add-apt-repository ppa:chris-lea/node.js
	sudo apt-get update
	sudo apt-get install -y nodejs npm

	sudo sh -c "echo description \"node.js server\"" >> /etc/init/nodejs.conf
	sudo sh -c "echo author      \"Benjamin Sproule\"" >> /etc/init/nodejs.conf
	sudo sh -c "echo 	start on started network-interface INTERFACE=eth0" >> /etc/init/nodejs.conf
	sudo sh -c "echo stop on shutdown" >> /etc/init/nodejs.conf
	sudo sh -c "echo script" >> /etc/init/nodejs.conf
	sudo sh -c "echo     \# We found $HOME is needed. Without it, we ran into problems" >> /etc/init/nodejs.conf
	sudo sh -c "echo     export HOME=\"/root\"" >> /etc/init/nodejs.conf
	sudo sh -c "echo     exec /usr/bin/node $HOME/node/app.js 2>&1 >> /var/log/node.log" >> /etc/init/nodejs.conf
	sudo sh -c "echo end script" >> /etc/init/nodejs.conf
	sudo chmod u+x /etc/init/nodejs.conf
	# use 'sudo start nodejs' to start and 'sudo stop nodejs' to stop
elif [[ $detectedDistro == "debian" ]]
then
	sudo apt-get install -y nodejs npm
	
	sudo sh -c "echo \#! /bin/bash" >> /etc/init.d/nodejs
	sudo sh -c "echo \# /etc/init.d/nodejs" >> /etc/init.d/nodejs
	sudo sh -c "echo \#" >> /etc/init.d/nodejs
	sudo sh -c "echo export PATH=$PATH:/opt/node/bin" >> /etc/init.d/nodejs
	sudo sh -c "echo NODEJS_PID=/var/run/nodejs.pid" >> /etc/init.d/nodejs
	sudo sh -c "echo start() {" >> /etc/init.d/nodejs
	sudo sh -c "echo 	if [ -f \$NODEJS_PID ]; then" >> /etc/init.d/nodejs
	sudo sh -c "echo 		rm -f \$NODEJS_PID" >> /etc/init.d/nodejs
	sudo sh -c "echo 	fi" >> /etc/init.d/nodejs
	sudo sh -c "echo 	node $HOME/node/app.js > $HOME/node/output.log &" >> /etc/init.d/nodejs
	sudo sh -c "echo 	echo \$! >> \$NODEJS_PID" >> /etc/init.d/nodejs
	sudo sh -c "echo }" >> /etc/init.d/nodejs
	sudo sh -c "echo stop() {" >> /etc/init.d/nodejs
	sudo sh -c "echo 	kill -9 `cat \$NODEJS_PID`" >> /etc/init.d/nodejs
	sudo sh -c "echo }" >> /etc/init.d/nodejs
	sudo sh -c "echo case \"\$1\" in" >> /etc/init.d/nodejs
	sudo sh -c "echo     start\)" >> /etc/init.d/nodejs
	sudo sh -c "echo         start" >> /etc/init.d/nodejs
	sudo sh -c "echo         ;;" >> /etc/init.d/nodejs
	sudo sh -c "echo     stop\)" >> /etc/init.d/nodejs
	sudo sh -c "echo         stop" >> /etc/init.d/nodejs
	sudo sh -c "echo         ;;" >> /etc/init.d/nodejs
	sudo sh -c "echo     restart)" >> /etc/init.d/nodejs
	sudo sh -c "echo         start" >> /etc/init.d/nodejs
	sudo sh -c "echo         stop" >> /etc/init.d/nodejs
	sudo sh -c "echo         ;;" >> /etc/init.d/nodejs
	sudo sh -c "echo     *\)" >> /etc/init.d/nodejs
	sudo sh -c "echo         echo \"Usage: /etc/init.d/nodejs {start|stop}\"" >> /etc/init.d/nodejs
	sudo sh -c "echo         exit 1" >> /etc/init.d/nodejs
	sudo sh -c "echo         ;;" >> /etc/init.d/nodejs
	sudo sh -c "echo esac" >> /etc/init.d/nodejs
	sudo sh -c "echo exit 0" >> /etc/init.d/nodejs
	sudo chmod u+x /etc/init.d/nodejs
elif [[ $detectedDistro == "fedora" ]]
then
	sudo yum install -y nodejs npm
fi

# setup minecraft server
if [[ $detectedDistro == "ubuntu" ]]
then
	cd $HOME

	if [ ! -d "$HOME/minecraft" ]
	then
		mkdir minecraft
	fi
	
	sudo useradd minecraft
	wget -O minecraft "http://minecraft.gamepedia.com/Tutorials/Server_startup_script/Script?action=raw"
	sudo mv minecraft /etc/init.d/minecraft
	sudo chmod 755 /etc/init.d/minecraft
	sudo update-rc.d minecraft defaults
	/etc/init.d/minecraft start
	/etc/init.d/minecraft stop
	/etc/init.d/minecraft update
	
	echo "gigaSproule" >> ops.txt
	mv ops.txt $HOME/minecraft/

	sudo crontab -u minecraft -l > file; echo '0 4 * * * /etc/init.d/minecraft_server backup' >> file; crontab file
	rm file
	
	echo "To change memory values, change MAXHEAP and MINHEAP in /etc/init.d/minecraft"
fi

# setup dynamic dns
private_key=
echo "#!/bin/sh" >> dynIPupdate.sh
echo "#FreeDNS updater script" >> dynIPupdate.sh
echo "UPDATEURL=\"http://freedns.afraid.org/dynamic/update.php?$private_key\"" >> dynIPupdate.sh
echo "DOMAIN=\"bensproule.co.uk\"" >> dynIPupdate.sh
echo "registered=\$(nslookup \$DOMAIN|tail -n2|grep A|sed s/[^0-9.]//g)" >> dynIPupdate.sh
echo "current=\$(wget -q -O - http://checkip.dyndns.org|sed s/[^0-9.]//g)" >> dynIPupdate.sh
echo "[ \"\$current\" != \"\$registered\" ] && {" >> dynIPupdate.sh
echo "wget -q -O /dev/null \$UPDATEURL" >> dynIPupdate.sh
echo "echo \"DNS updated on:\"; date" >> dynIPupdate.sh
echo "}" >> dynIPupdate.sh
chmod +x dynIPupdate.sh
sudo mv dynIPupdate.sh /opt/

crontab -l > file; echo '0 5 * * * /opt/dynIPupdate.sh >/dev/null 2>&1' >> file; crontab file
rm file
/opt/dynIPupdate.sh

# setup ftp
wget http://www.britintel.co.uk/files/vsftpd.conf
sudo mv vsftpd.conf /etc/
if [[ $detectedDistro == "ubuntu" ]]
then
	sudo restart vsftpd
elif [[ $detectedDistro == "debian" ]]
then
	sudo /etc/init.d/vsftpd restart
elif [[ $detectedDistro == "fedora" ]]
then
	sudo systemctl restart vsftpd
fi

# setup xbmc/auto login
if [[ $detectedDistro == "ubuntu" ]]
then
	sudo sed '$d' < /etc/init/tty1.conf > file2 ; sudo mv file2 /etc/init/tty1.conf
	sudo sh -c "echo exec /bin/login -f benjamin < /dev/tty1 > /dev/tty1 2>&1 >> /etc/init/tty1.conf"
	echo "#!/bin/bash" >> .xinitrc
	echo "exec /usr/bin/xbmc-standalone" >> .xinitrc
	chmod +x .xinitrc
	rm .Xauthority*
	sudo cp /etc/X11/xorg.conf /etc/X11/xorg.conf.backup
	sudo dpkg-reconfigure xserver-xorg
	cp /etc/skel/.profile ~/
	echo "# startx automatically" >> .profile
	echo "if [ $(tty) == \"/dev/tty1\" ]; then" >> .profile
	echo "startx" >> .profile
	echo "fi" >> .profile
fi

# install graphics drivers for HD resolutions
if [[ $detectedDistro == "ubuntu" ]]
then
	sudo add-apt-repository ppa:ubuntu-x-swat/x-updates
	sudo apt-get update && sudo apt-get dist-upgrade
	if [[ `lspci | grep VGA` == *NVIDIA* ]]
	then
		sudo apt-get install nvidia-current nvidia-settings
	elif [[ `lspci | grep VGA` == *ATI* ]]
	then
		sudo apt-get install fglrx fglrx-amdcccle
	fi
	sudo cp /etc/X11/xorg.conf /usr/share/X11/xorg.conf.d/xorg.conf
fi

# setup samba with Media hard drive
if [[ $detectedDistro == "fedora" ]]
then
	sudo systemctl start smb.service
	sudo systemctl start nmb.service
	sudo systemctl enable smb.service
	sudo systemctl enable nmb.service
fi

sudo mkdir /media/Media
sudo sh -c "echo [Media Share] >> /etc/samba/smb.conf"
sudo sh -c "echo comment = Media share on server >> /etc/samba/smb.conf"
sudo sh -c "echo read only = no >> /etc/samba/smb.conf"
sudo sh -c "echo locking = no >> sudo /etc/samba/smb.conf"
sudo sh -c "echo path = /media/Media >> /etc/samba/smb.conf"
sudo sh -c "echo guest ok = yes >> /etc/samba/smb.conf"

# auto mount extra hard drives
sudo sh -c "echo UUID=3026518E0BD8945B	/media/Media	ntfsdefaults	0	0 >> /etc/fstab"
echo "Create directory in /media and add line to /etc/fstab using 'sudo blkid' to get UUID (eg. UUID=insert_uuid /media/Media ntfs defaults 0 0)"

# install linux counter script
if [[ $detectedDistro == "ubuntu" ]]
then
	sudo echo "deb http://ppa.launchpad.net/alex-mieland/ppa/ubuntu oneiric main" | sudo tee -a /etc/apt/sources.list
	sudo echo "deb-src http://ppa.launchpad.net/alex-mieland/ppa/ubuntu oneiric main" | sudo tee -a /etc/apt/sources.list
	sudo apt-get update
	sudo apt-get install lico-update
	lico-update.sh -ci
fi

sudo shutdown -r now
