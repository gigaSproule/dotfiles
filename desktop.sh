#!/bin/bash

cd $HOME
mkdir temp
cd temp

architecture=`uname -m`

detectedDistro="Unknown"
regExpLsbInfo="Description:[[:space:]]*([^ ]*)"
regExpLsbFile="/etc/(.*)[-_]"

if [ `which lsb_release 2>/dev/null` ]; then
	lsbInfo=`lsb_release -d`
	if [[ $lsbInfo =~ $regExpLsbInfo ]]; then
		detectedDistro=${BASH_REMATCH[1]}
	else
		echo "??? Should not occur: Don't find distro name in lsb_release output ???"
		exit 1
	fi
else
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

# correct repos
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	sudo sed -i "/^# deb .*partner/ s/^# //" /etc/apt/sources.list
elif [[ $detectedDistro == "fedora" ]]
then
	sudo yum localinstall --nogpgcheck http://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-stable.noarch.rpm http://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-stable.noarch.rpm
elif [[ $detectedDistro == "opensuse" ]]
then
	sudo zypper ar http://download.opensuse.org/distribution/12.2/repo/oss/ official_oss
	sudo zypper ar http://download.opensuse.org/distribution/12.2/repo/non-oss/ official_non-oss
	sudo zypper ar http://download.opensuse.org/update/12.2/ update
fi

# update system
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	sudo apt-get update
	sudo apt-get -y upgrade
	sudo apt-get -yf install
elif [[ $detectedDistro == "fedora" ]]
then
	sudo yum -y update
elif [[ $detectedDistro == "opensuse" ]]
then
	sudo zypper update
fi

# install all apps from repos
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	packages="eclipse filezilla putty openjdk-6-jdk openjdk-7-jdk icedtea-6-plugin icedtea-7-plugin subversion gcc calibre apache2 php5 libapache2-mod-php5 mysql-server php5-mysql virtualbox git-core gnupg flex bison gperf build-essential zip curl libc6-dev libncurses5-dev:i386 x11proto-core-dev libx11-dev:i386 libreadline6-dev:i386 libgl1-mesa-dev:i386 g++-multilib mingw32 tofrodos python-markdown libxml2-utils xsltproc zlib1g-dev:i386 samba libpam-smbpass unetbootin vim wget lynx cmus caca-utils finch gtk2-engines-murrine gtk2-engines-pixbuf zip unzip"
	command="sudo apt-get -yf install"
elif [[ $detectedDistro == "fedora" ]]
then
	packages="eclipse filezilla putty git gimp gparted calibre gstreamer-plugins-bad gstreamer-plugins-ugly gstreamer-ffmpeg phonon-backend-gstreamer java-*-openjdk java-*-openjdk-plugin wget php mysql httpd vim gtk-murrine-engine gtk2-engines zip unzip"
	command="sudo yum -y install"
elif [[ $detectedDistro == "opensuse" ]]
then
	## missing eclipse putty openjdk-6-jdk openjdk-7-jdk icedtea-7-plugin libapache2-mod-php5 mysql-server build-essential unetbootin
	packages="chromium filezilla gimp calibre gcc subversion git apache2 php5 php5-mysql virtualbox curl wget samba vim gtk-murrine-engine gtk2-engines zip unzip"
	command="sudo zypper install"
fi

if [[ $detectedDistro == "kubuntu" ]]
then
	packages="$packages kubuntu-restricted-extras chromium-browser gimp firefox"
elif [[ $detectedDistro == "xubuntu" ]]
then
	packages="$packages xubuntu-restricted-extras chromium-browser gimp planner gparted"
elif [[ $detectedDistro == "lubuntu" ]]
then
	packages="$packages lubuntu-restricted-extras firefox"
elif [[ $detectedDistro == "ubuntu" ]]
then
	packages="$packages ubuntu-restricted-extras gimp chromium-browser planner gparted gstreamer0.10-plugins-bad gstreamer0.10-plugins-ugly gstreamer0.10-ffmpeg gstreamer0.10-plugins-good"
else
	echo "Could not install distro specific packages"
fi

command="$command $packages"
$command

# install specific repos
if [[ $detectedDistro == "fedora" ]]
then
	wget http://repos.fedorapeople.org/repos/spot/chromium-stable/fedora-chromium-stable.repo
	sudo mv fedora-chromium-stable.repo /etc/yum.repos.d/fedora-chromium-stable.repo
	wget http://download.virtualbox.org/virtualbox/rpm/fedora/virtualbox.repo
	sudo mv virtualbox.repo /etc/yum.repos.d/virtualbox.repo
	
	sudo yum -y update
	sudo yum -y install chromium VirtualBox-4.2
	
	service vboxdrv setup
	usermod -a -G vboxusers $USER
fi
	

# install dvd decoders
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	sudo apt-get -yf install libdvdread4
	sudo /usr/share/doc/libdvdread4/install-css.sh
elif [[ $detectedDistro == "fedora" ]]
then
	echo "[atrpms]" >> atrpms.repo
	echo "name=Fedora Core $releasever - $basearch - ATrpms" >> atrpms.repo
	echo "baseurl=http://dl.atrpms.net/f$releasever-$basearch/atrpms/stable" >> atrpms.repo
	echo "gpgkey=http://ATrpms.net/RPM-GPG-KEY.atrpms" >> atrpms.repo
	echo "enabled=0" >> atrpms.repo
	echo "gpgcheck=1" >> atrpms.repo
	sudo mv atrpms.repo /etc/yum.repos.d/atrpms.repo
	sudo rpm --import http://packages.atrpms.net/RPM-GPG-KEY.atrpms
	sudo yum --enablerepo=atrpms install libdvdcss
fi


if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	sudo /etc/init.d/apache2 restart
elif [[ $detectedDistro == "fedora" ]]
then
	sudo chkconfig httpd on
	sudo /etc/init.d/httpd restart
fi

# install DropBox
if [[ $detectedDistro == "ubuntu" ]]
then
	sudo apt-get -yf install nautilus-dropbox
elif [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" ]]
then
	if [[ $architecture == "x86_64" ]]
	then
		wget -O dropbox.deb https://www.dropbox.com/download?dl=packages/ubuntu/dropbox_1.6.0_amd64.deb
	else
		wget -O dropbox.deb https://www.dropbox.com/download?dl=packages/ubuntu/dropbox_1.6.0_i386.deb
	fi
	sudo dpkg -i dropbox.deb
elif [[ $detectedDistro == "fedora" ]]
then
	if [[ $architecture == "x86_64" ]]
	then
		wget -O dropbox.rpm https://www.dropbox.com/download?dl=packages/fedora/nautilus-dropbox-1.6.0-1.fedora.x86_64.rpm
	else
		wget -O dropbox.rpm https://www.dropbox.com/download?dl=packages/fedora/nautilus-dropbox-1.6.0-1.fedora.i386.rpm
	fi
	sudo rpm -i dropbox.rpm
else
	if [[ $architecture == "x86_64" ]]
	then
		wget -O - "https://www.dropbox.com/download?plat=lnx.x86" | tar xzf -
	else
		wget -O - "https://www.dropbox.com/download?plat=lnx.x86_64" | tar xzf -
	fi
	mv .dropbox-dist $HOME
	$HOME/.dropbox-dist/dropboxd
fi

# setup Apache
wget -O default http://www.britintel.co.uk/files/default_desktop
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	sudo mv default /etc/apache2/sites-available/
	sudo /etc/init.d/apache2 restart
elif [[ $detectedDistro == "fedora" ]]
then
	sudo mv default /etc/httpd/conf/httpd.conf
	sudo /etc/init.d/httpd restart
fi

# install Spotify
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	sudo echo "deb http://repository.spotify.com stable non-free" | sudo tee -a /etc/apt/sources.list
	sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 94558F59
	sudo apt-get update
	sudo apt-get -yf install spotify-client-qt
else
	wget -O spotify.rpm https://www.dropbox.com/s/4mi31ngl7v38iq6/spotify-client-0.8.3.278.g21c7566.632-2.x86_64.rpm
	sudo rpm -i spotify.rpm
fi

# install Skype
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	sudo apt-get -yf install skype
elif [[ $detectedDistro == "fedora" ]]
then
	wget -O skype.rpm http://www.skype.com/intl/en-us/get-skype/on-your-computer/linux/downloading.fedora
	sudo yum -i skype.rpm
fi

# install MS fonts
if [ ! -d "/usr/share/fonts/vista" ]
then
	wget http://www.britintel.co.uk/files/fonts.tar.gz
	sudo tar -C /usr/share/fonts/ -xvf fonts.tar.gz
fi

# install CrossOver
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	if [[ $architecture == "x86_64" ]]
	then
		wget -O crossover.deb http://media.codeweavers.com/pub/crossover/cxlinux/demo/ia32-crossover_11.2.0-1_amd64.deb
	else
		wget -O crossover.deb http://media.codeweavers.com/pub/crossover/cxlinux/demo/crossover_11.2.0-1_i386.deb
	fi
	sudo dpkg -i crossover.deb
elif [[ $detectedDistro == "fedora" ]]
then
	wget -O crossover.rpm http://media.codeweavers.com/pub/crossover/cxlinux/demo/crossover-11.0.3-1.i386.rpm
	sudo yum -i crossover.rpm
fi

# install Humble Bundle games
# install Gish
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	wget http://www.britintel.co.uk/files/gish.deb
	sudo dpkg -i gish.deb
elif [[ $detectedDistro == "fedora" ]]
then
	wget http://www.britintel.co.uk/files/gish.rpm
	sudo rpm -i gish.rpm
fi

# install Night Sky
if [ ! -d "$HOME/NightSky" ]
then
	wget http://www.britintel.co.uk/files/nightsky.tar.gz
	tar -C $HOME -xvf nightsky.tar.gz

	echo "[Desktop Entry]" >> NightSky.desktop
	echo "Name=NightSky" >> NightSky.desktop
	echo "Type=Application" >> NightSky.desktop
	if [[ $architecture == "x86_64" ]]
	then
		echo "Exec=$HOME/NightSky/NightSkyHD_64" >> NightSky.desktop
	else
		echo "Exec=$HOME/NightSky/NightSkyHD" >> NightSky.desktop
	fi
	echo "Categories=Game;" >> NightSky.desktop
	echo "Terminal=false" >> NightSky.desktop
	echo "Comment=NightSky Game" >> NightSky.desktop
	echo "Icon=$HOME/Dropbox/Photos/Icons/nightsky.png" >> NightSky.desktop
	chmod +x NightSky.desktop

	mkdir $HOME/.local/share/applications/
	mv NightSky.desktop $HOME/.local/share/applications/
fi

# install Minecraft
if [ ! -d "$HOME/minecraft" ]
then
	cd $HOME
	mkdir minecraft
	cd minecraft
	wget https://s3.amazonaws.com/MinecraftDownload/launcher/minecraft.jar
	chmod +x minecraft.jar
	wget https://s3.amazonaws.com/MinecraftDownload/launcher/minecraft_server.jar
	chmod +x minecraft_server.jar

	echo "[Desktop Entry]" >> Minecraft.desktop
	echo "Name=Minecraft" >> Minecraft.desktop
	echo "Type=Application" >> Minecraft.desktop
	echo "Exec=optirun java -jar $HOME/minecraft/minecraft.jar" >> Minecraft.desktop
	echo "Categories=Game;" >> Minecraft.desktop
	echo "Terminal=false" >> Minecraft.desktop
	echo "Comment=Minecraft Client" >> Minecraft.desktop
	echo "Icon=$HOME/Dropbox/Photos/Icons/minecraft.png" >> Minecraft.desktop
	chmod +x Minecraft.desktop
	mv Minecraft.desktop $HOME/.local/share/applications/

	echo "[Desktop Entry]" >> MinecraftServer.desktop
	echo "Name=Minecraft Server" >> MinecraftServer.desktop
	echo "Type=Application" >> MinecraftServer.desktop
	echo "Exec=java -Xmx1024M -Xms1024M -jar $HOME/minecraft/minecraft_server.jar" >> MinecraftServer.desktop
	echo "Categories=Game;" >> MinecraftServer.desktop
	echo "Terminal=false" >> MinecraftServer.desktop
	echo "Comment=Minecraft Server" >> MinecraftServer.desktop
	echo "Icon=$HOME/Dropbox/Photos/Icons/minecraft.png" >> MinecraftServer.desktop
	chmod +x MinecraftServer.desktop
	mv MinecraftServer.desktop $HOME/.local/share/applications/

	cd $HOME/temp
fi

# install Opera
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	if [[ $architecture == "x86_64" ]]
	then
		wget -O opera.deb http://www.britintel.co.uk/files/operax64.deb
	else
		wget -O opera.deb http://www.britintel.co.uk/files/operax86.deb
	fi
	sudo dpkg -i opera.deb
elif [[ $detectedDistro == "fedora" ]]
then
	if [[ $architecture == "x86_64" ]]
	then
		wget -O opera.rpm http://www.britintel.co.uk/files/operax64.rpm
	else
		wget -O opera.rpm http://www.britintel.co.uk/files/operax86.rpm
	fi
	sudo rpm -i opera.rpm
fi

## install Altitude
#wget http://www.britintel.co.uk/files/altitude.sh
#chmod +x altitude.sh
#./altitude.sh

# install Steam
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	wget http://media.steampowered.com/client/installer/steam.deb
	sudo dpkg -i steam.deb
elif [[ $detectedDistro == "fedora" ]]
then
	wget http://spot.fedorapeople.org/steam/steam.repo
	sudo mv steam.repo /etc/yum.repos.d/steam.repo
fi

# install Data Modeler
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	wget http://www.britintel.co.uk/files/datamodeler.deb
	sudo dpkg -i datamodeler.deb
elif [[ $detectedDistro == "fedora" ]]
then
	wget http://www.britintel.co.uk/files/datamodeler.rpm
	sudo rpm -i datamodeler.rpm
fi

# install SQL Developer
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	wget http://www.britintel.co.uk/files/sqldeveloper.deb
	sudo dpkg -i sqldeveloper.deb
elif [[ $detectedDistro == "fedora" ]]
then
	wget http://www.britintel.co.uk/files/sqldeveloper.rpm
	sudo rpm -i sqldeveloper.rpm
fi

# install IntelliJ Ultimate
if [ ! -d "/opt/intellij" ]
then
	wget -O intellij.tar.gz http://download.jetbrains.com/idea/ideaIU-12.0.4.tar.gz
	tar -zxof intellij.tar.gz -C $HOME
	sudo mv $HOME/idea-IU-* /opt/intellij

	echo "[Desktop Entry]" >> IntelliJ.desktop
	echo "Name=IntelliJ" >> IntelliJ.desktop
	echo "Type=Application" >> IntelliJ.desktop
	echo "Exec=/opt/intellij/bin/idea.sh" >> IntelliJ.desktop
	echo "Categories=Development;IDE;Java;" >> IntelliJ.desktop
	echo "Terminal=false" >> IntelliJ.desktop
	echo "Comment=IntelliJ Ulitmate" >> IntelliJ.desktop
	echo "Icon=/opt/intellij/bin/idea.png" >> IntelliJ.desktop
	chmod +x IntelliJ.desktop
	sudo mv IntelliJ.desktop /usr/local/share/applications/
fi

# install Android sdk
if [ ! -d "/opt/android-sdk" ]
then
	wget -O androidsdk.tgz http://dl.google.com/android/android-sdk_r18-linux.tgz
	tar -zxof androidsdk.tgz -C /opt
	sudo mv $HOME/android-sdk-linux /opt/android-sdk

	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"0bb4\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"0e79\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"0502\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"0b05\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"413c\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"0489\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"091e\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"18d1\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"0bb4\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"12d1\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"24e3\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"2116\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"0482\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"17ef\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"1004\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"22b8\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"0409\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"2080\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"0955\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"2257\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"10a9\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"1d4d\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"0471\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"04da\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"05c6\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"1f53\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"04e8\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"04dd\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"0fce\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"0930\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo sh -c 'echo SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"19d2\", MODE=\"0666\" >> /etc/udev/rules.d/51-android.rules'
	sudo chmod a+r /etc/udev/rules.d/51-android.rules
fi

# setup git
git config --global user.name "Benjamin Sproule"
git config --global user.email benjaminsproule@gmail.com

# install GoLang
if [[ ! -d "/opt/go-lang" ]]
then
	wget -O golang.tar.gz http://go.googlecode.com/files/go1.0.1.linux-amd64.tar.gz
	sudo tar -C /opt -xzf golang.tar.gz
	
fi

# install nodejs
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	sudo apt-get -yf install python-software-properties
	sudo add-apt-repository ppa:chris-lea/node.js
	sudo apt-get update
	sudo apt-get -yf install nodejs npm
fi

# install ruby
curl -L https://get.rvm.io | bash -s stable --ruby
echo "source $HOME/.rvm/scripts/rvm" >> $HOME/.bashrc

# install grails
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	sudo add-apt-repository ppa:groovy-dev/grails
	sudo apt-get update
	sudo apt-get -yf install grails-ppa
	echo export JAVA_HOME=/usr/lib/jvm/java-7-openjdk-amd64/ >> $HOME/.bashrc
fi

# install Google Music
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	wget -O google-music.deb https://dl.google.com/linux/direct/google-musicmanager-beta_current_amd64.deb
	sudo dpkg -i google-music.deb
fi

# install hal for DRM Flash videos
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	sudo apt-get install hal
	sudo mkdir /etc/hal/fdi/preprobe
	sudo mkdir /etc/hal/fdi/information
	/usr/sbin/hald --daemon=yes --verbose=yes
	rm -rf ~/.adobe
elif [[ $detectedDistro == "fedora" ]]
then
	wget http://linuxdownload.adobe.com/adobe-release/adobe-release-x86_64-1.0-1.noarch.rpm
	sudo yum install -y adobe-release-x86_64-1.0-1.noarch.rpm
	sudo yum install -y flash-plugin
	sudo yum install -y policycoreutils-devel
	wget http://togami.com/~warren/archive/2012/adobedrm.te
	checkmodule -M -m -o adobedrm.mod adobedrm.te
	sudo semodule_package -o adobedrm.pp -m adobedrm.mod
	wget http://thinkingconcurrently.com/files/f19_flash/fakehal-0.5.14-7.fc19.x86_64.rpm
	wget http://thinkingconcurrently.com/files/f19_flash/fakehal-libs-0.5.14-7.fc19.x86_64.rpm
	sudo yum install -y fakehal-0.5.14-7.fc19.x86_64.rpm fakehal-libs-0.5.14-7.fc19.x86_64.rpm
	rm -rf ~/.adobe/Flash_Player/
	sudo mkdir -p /usr/share/hal/fdi/preprobe /usr/share/hal/fdi/information /usr/share/hal/fdi/policy/20thirdparty /var/cache/hald/
	sudo ln -s /usr/share/hal /etc/hal
	sudo touch /var/cache/hald/fdi-cache
	sudo systemctl start haldaemon.service
fi

wget -O Zukitwo.zip http://gnome-look.org/CONTENT/content-files/140562-Zukitwo.zip
unzip -tq Zukitwo.zip
sudo mv Zukitwo/ /usr/share/themes/
sudo mv Zukitwo-Shell/ /usr/share/themes/

# install linux counter script
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	sudo echo "deb http://ppa.launchpad.net/alex-mieland/ppa/ubuntu oneiric main" | sudo tee -a /etc/apt/sources.list
	sudo echo "deb-src http://ppa.launchpad.net/alex-mieland/ppa/ubuntu oneiric main" | sudo tee -a /etc/apt/sources.list
	sudo apt-get update
	sudo apt-get -yf install lico-update
	lico-update.sh -ci
else
	wget -N --no-cache http://linuxcounter.net/script/lico-update.sh
	chmod +x lico-update.sh
	mv lico-update.sh $HOME
	$HOME/lico-update.sh -ci
fi

# install wireless drivers
if [[ `lspci|grep Network` == *BCM43* ]]
then
	if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
	then
		sudo apt-get -y remove bcmwl-kernel-source
		sudo apt-get -yf install firmware-b43-installer
	elif [[ $detectedDistro == "fedora" ]]
	then
		sudo yum -y install b43-fwcutter
		wget http://downloads.openwrt.org/sources/broadcom-wl-4.150.10.5.tar.bz2
		tar xjf broadcom-wl-4.150.10.5.tar.bz2
		cd broadcom-wl-4.150.10.5/driver
		sudo b43-fwcutter -w /lib/firmware wl_apsta_mimo.o
	fi
fi

# install Optimus drivers
if [[ `lspci | grep VGA` == *Intel* && `lspci | grep VGA` == *NVIDIA* ]]
then
	if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
	then
		sudo add-apt-repository ppa:bumblebee/stable
		sudo apt-get update
		sudo apt-get -yf install bumblebee bumblebee-nvidia
	elif [[ $detectedDistro == "fedora" ]]
	then
		sudo yum -y --nogpgcheck install http://install.linux.ncsu.edu/pub/yum/itecs/public/bumblebee/fedora19/noarch/bumblebee-release-1.1-1.noarch.rpm
		sudo yum -y --nogpgcheck install http://install.linux.ncsu.edu/pub/yum/itecs/public/bumblebee-nonfree/fedora19/noarch/bumblebee-nonfree-release-1.1-1.noarch.rpm
		sudo yum -y install libbsd-devel libbsd glibc-devel libX11-devel help2man autoconf git tar glib2 glib2-devel kernel-devel kernel-headers automake gcc gtk2-devel VirtualGL VirtualGL.i686 glibc-devel bbswitch bumblebee bumblebee-nvidia primus primus.i686
	fi
	sudo usermod -a -G bumblebee $USER
fi

# update system again
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	sudo apt-get update
	sudo apt-get -y upgrade
	sudo apt-get -yf install
	sudo apt-get -y autoremove
elif [[ $detectedDistro == "fedora" ]]
then
	sudo yum -y update
fi

# exports
echo "export PATH=$PATH:/opt/android-sdk/platform-tools:/opt/go/bin:/opt/android/bin:$HOME/.rvm/bin" >> $HOME/.bashrc
echo "export USE_CCACHE=1" >> $HOME/.bashrc

# setup miscellanious
if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	sudo cp /usr/share/applications/skype.desktop $HOME/.config/autostart/
	sudo cp /usr/share/applications/google-musicmanager.desktop $HOME/.config/autostart/
fi

echo "Everything should be installed"

if [[ $detectedDistro == "kubuntu" || $detectedDistro == "xubuntu" || $detectedDistro == "lubuntu" || $detectedDistro == "ubuntu" ]]
then
	echo "To setup mySQL, follow this tutorial http://www.debuntu.org/how-to-create-a-mysql-database-and-set-privileges-to-a-user"
fi

# remove temporary files/folders
cd $HOME
rm -rf temp
