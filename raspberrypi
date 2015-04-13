#!/bin/bash

SETUP_FREEDNS=${1:-false}
SETUP_MAIL=${2:-false}
SETUP_OWNCLOUD=${3:-false}
SETUP_NODEJS=${4:-false}
HOSTNAME=( `eval echo "$5"` )
FREEDNS_PRIVATE_KEY=( `eval echo "$6"` )

install_general() {
	sudo apt-get update
	sudo apt-get dist-upgrade -y
	sudo apt-get install -yf
	sudo apt-get install -y git vim wget dnsutils
}

install_freedns() {
	echo "#!/bin/sh" >> dynIPupdate.sh
	echo "#FreeDNS updater script" >> dynIPupdate.sh
	echo "UPDATEURL=\"http://freedns.afraid.org/dynamic/update.php?$1\"" >> dynIPupdate.sh
	echo "DOMAIN=\"$2\"" >> dynIPupdate.sh
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
}

install_postfix() {
	sudo apt-get install -y postfix libsasl2-2 sasl2-bin libsasl2-modules
	
	echo "Set the following:
	General type of mail configuration: Internet Site
	NONE doesn't appear to be requested in current config
	System mail name: server1.example.com
	Root and postmaster mail recipient: <admin_user_name>
	Other destinations for mail: server1.example.com, example.com, localhost.example.com, localhost
	Force synchronous updates on mail queue?: No
	Local networks: 127.0.0.0/8
	Yes doesn't appear to be requested in current config
	Mailbox size limit (bytes): 0
	Local address extension character: +
	Internet protocols to use: all"
	
	sudo dpkg-reconfigure postfix
	sudo postconf -e 'home_mailbox = Maildir/'
	sudo postconf -e 'mailbox_command ='
	sudo postconf -e 'smtpd_sasl_local_domain ='
	sudo postconf -e 'smtpd_sasl_auth_enable = yes'
	sudo postconf -e 'smtpd_sasl_security_options = noanonymous'
	sudo postconf -e 'broken_sasl_auth_clients = yes'
	sudo postconf -e 'smtpd_recipient_restrictions = permit_sasl_authenticated,permit_mynetworks,reject_unauth_destination'
	sudo postconf -e 'inet_interfaces = all'
	
	sudo sh -c 'echo pwcheck_method: saslauthd >> /etc/postfix/sasl/smtpd.conf'
	sudo sh -c 'echo mech_list: plain login >> /etc/postfix/sasl/smtpd.conf'
	
	touch smtpd.key
	chmod 600 smtpd.key
	openssl genrsa 1024 > smtpd.key
	openssl req -new -key smtpd.key -x509 -days 3650 -out smtpd.crt # has prompts
	openssl req -new -x509 -extensions v3_ca -keyout cakey.pem -out cacert.pem -days 3650 # has prompts
	sudo mv smtpd.key /etc/ssl/private/
	sudo mv smtpd.crt /etc/ssl/certs/
	sudo mv cakey.pem /etc/ssl/private/
	sudo mv cacert.pem /etc/ssl/certs/
	
	sudo postconf -e 'smtp_tls_security_level = may'
	sudo postconf -e 'smtpd_tls_security_level = may'
	sudo postconf -e 'smtpd_tls_auth_only = no'
	sudo postconf -e 'smtp_tls_note_starttls_offer = yes'
	sudo postconf -e 'smtpd_tls_key_file = /etc/ssl/private/smtpd.key'
	sudo postconf -e 'smtpd_tls_cert_file = /etc/ssl/certs/smtpd.crt'
	sudo postconf -e 'smtpd_tls_CAfile = /etc/ssl/certs/cacert.pem'
	sudo postconf -e 'smtpd_tls_loglevel = 1'
	sudo postconf -e 'smtpd_tls_received_header = yes'
	sudo postconf -e 'smtpd_tls_session_cache_timeout = 3600s'
	sudo postconf -e 'tls_random_source = dev:/dev/urandom'
	sudo postconf -e 'myhostname = $1'
	
	sudo /etc/init.d/postfix restart
	
	#Need to check
	sudo sed -i "/^# START=yes.*/ s/^# //" /etc/default/saslauthd
	sudo sh -c 'echo PWDIR=\"/var/spool/postfix/var/run/saslauthd\" >> /etc/default/saslauthd'
	sudo sh -c 'PARAMS=\"-m \${PWDIR}\" >> /etc/default/saslauthd'
	sudo sh -c 'PIDFILE=\"\${PWDIR}/saslauthd.pid\" >> /etc/default/saslauthd'
	sudo sh -c 'OPTIONS=\"-c -m /var/spool/postfix/var/run/saslauthd\" >> /etc/default/saslauthd'
	
	sudo dpkg-statoverride --force --update --add root sasl 755 /var/spool/postfix/var/run/saslauthd
	sudo /etc/init.d/saslauthd start
}

install_dovecot() {
	sudo apt-get install -y dovecot-imapd dovecot-pop3d

	#Need to check
	sudo sed -i "/^protocols =.*/ s/^protocols = pop3 pop3s imap imaps//" /etc/dovecot/dovecot.conf

	sudo sh -c 'echo home_mailbox = Maildir/ >> /etc/postfix/main.cf'

	sudo sed -i "/^mail_location =.*/ s/^mail_location = maildir:/home/%u/Maildir//" /etc/dovecot/dovecot.conf

	sudo maildirmake.dovecot /etc/skel/Maildir
	sudo maildirmake.dovecot /etc/skel/Maildir/.Drafts
	sudo maildirmake.dovecot /etc/skel/Maildir/.Sent
	sudo maildirmake.dovecot /etc/skel/Maildir/.Trash
	sudo maildirmake.dovecot /etc/skel/Maildir/.Templates

	sudo cp -r /etc/skel/Maildir /home/$USER/
	sudo chown -R myuser:usergroup /home/$USER/Maildir
	sudo chmod -R 700 /home/$USER/Maildir

	#Need to check
	sudo sed -i "/^ssl =.*/ s/^ssl = yes//" /etc/dovecot/dovecot.conf
	sudo sed -i "/^ssl_cert_file =.*/ s/^ssl_cert_file = /etc/ssl/certs/ssl-cert-snakeoil.pem//" /etc/dovecot/dovecot.conf
	sudo sed -i "/^ssl_key_file =.*/ s/^ssl_key_file = /etc/ssl/private/ssl-cert-snakeoil.key//" /etc/dovecot/dovecot.conf

	sudo sed -i "/^# listen =.*/ s/^# //" /etc/dovecot/dovecot.conf

	# add below to /etc/dovecot/dovecot.conf instead of above line!
	#protocol imap {
	#	listen = *:143
	#	ssl_listen = *:993
	#	login_greeting_capability = yes
	#	imap_client_workarounds = tb-extra-mailbox-sep
	#}
	
	sudo /etc/init.d/dovecot restart
}

install_owncloud() {
	echo "Make the interfaces file look as follows (with correct IP addresses):
	auto eth0
	iface eth0 inet static
		address 192.168.1.118
		gateway 192.168.1.1
		netmask 255.255.255.0
		network 192.168.1.0
		broadcast 192.168.1.255"
	sudo vim /etc/network/interfaces
	
	sudo /etc/init.d/networking restart
	
	sudo apt-get install -y apache2 php5 php5-json php5-gd php5-sqlite curl libcurl3 libcurl4-openssl-dev php5-curl php5-gd php5-cgi php-pear php5-dev build-essential libpcre3-dev php5 libapache2-mod-php5 php-apc
	
	sudo pecl install apc

	sudo sh -c 'echo extension=apc.so' >> /etc/php5/mods-available/apc.ini
	sudo sh -c 'echo apc.enabled=1' >> /etc/php5/mods-available/apc.ini
	sudo sh -c 'echo apc.shm_size=30' >> /etc/php5/mods-available/apc.ini
	
	sudo sed -i "/^upload_max_filesize =.*/ s/^upload_max_filesize = 1024M//" /etc/php5/apache2/php.ini
	sudo sed -i "/^post_max_size =.*/ s/^post_max_size = 1200M//" /etc/php5/apache2/php.ini
	sudo sed -i "/^extension =.*/ s/^extension=apc.so//" /etc/php5/apache2/php.ini

	sudo sed -i "/^AllowOverride .*/ s/^AllowOverride All//" /etc/apache2/sites-enabled/000-default
	
	sudo a2enmod rewrite
	sudo a2enmod headers
	
	sudo openssl genrsa -des3 -out server.key 1024; sudo openssl rsa -in server.key -out server.key.insecure;sudo openssl req -new -key server.key -out server.csr;sudo openssl x509 -req -days 365 -in server.csr -signkey server.key -out server.crt;sudo cp server.crt /etc/ssl/certs;sudo cp server.key /etc/ssl/private;sudo a2enmod ssl;sudo a2ensite default-ssl
	
	sudo service apache2 restart
	
	wget -O owncloud.tar.bz2 http://download.owncloud.org/community/owncloud-5.0.7.tar.bz2
	sudo tar -xjf owncloud.tar.bz2
	sudo cp -r owncloud /var/www
	sudo chown -R www-data:www-data /var/www/owncloud/

	sudo sed -i "/^php_value upload_max_filesize .*/ s/^php_value upload_max_filesize 1024M//" /var/www/owncloud/.htaccess
	sudo sed -i "/^php_value post_max_size .*/ s/^php_value post_max_size 1200M//" /var/www/owncloud/.htaccess
}

install_nodejs() {
	wget http://node-arm.herokuapp.com/node_latest_armhf.deb
	sudo dpkg -i node_latest_armhf.deb

	#Need to check
	sudo sh -c "echo \#! /bin/bash >> /etc/init.d/nodejs"
	sudo sh -c "echo \# /etc/init.d/nodejs >> /etc/init.d/nodejs"
	sudo sh -c "echo \# >> /etc/init.d/nodejs"
	sudo sh -c "echo NODEJS_PID=/var/run/nodejs.pid >> /etc/init.d/nodejs"
	sudo sh -c "echo start() { >> /etc/init.d/nodejs"
	sudo sh -c "echo 	if [ -f \$NODEJS_PID ]; then >> /etc/init.d/nodejs"
	sudo sh -c "echo 		rm -f \$NODEJS_PID >> /etc/init.d/nodejs"
	sudo sh -c "echo 	fi >> /etc/init.d/nodejs"
	sudo sh -c "echo 	node $HOME/node/app.js > $HOME/node/output.log & >> /etc/init.d/nodejs"
	sudo sh -c "echo 	echo \$! >> \$NODEJS_PID >> /etc/init.d/nodejs"
	sudo sh -c "echo } >> /etc/init.d/nodejs"
	sudo sh -c "echo stop() { >> /etc/init.d/nodejs"
	sudo sh -c "echo 	kill -9 `cat \$NODEJS_PID` >> /etc/init.d/nodejs"
	sudo sh -c "echo } >> /etc/init.d/nodejs"
	sudo sh -c "echo case \"\$1\" in >> /etc/init.d/nodejs"
	sudo sh -c "echo 	start\) >> /etc/init.d/nodejs"
	sudo sh -c "echo 		start >> /etc/init.d/nodejs"
	sudo sh -c "echo 		;; >> /etc/init.d/nodejs"
	sudo sh -c "echo 	stop\) >> /etc/init.d/nodejs"
	sudo sh -c "echo 		stop >> /etc/init.d/nodejs"
	sudo sh -c "echo 		;; >> /etc/init.d/nodejs"
	sudo sh -c "echo 	restart) >> /etc/init.d/nodejs"
	sudo sh -c "echo 		start >> /etc/init.d/nodejs"
	sudo sh -c "echo 		stop >> /etc/init.d/nodejs"
	sudo sh -c "echo 		;; >> /etc/init.d/nodejs"
	sudo sh -c "echo 	*\) >> /etc/init.d/nodejs"
	sudo sh -c "echo 		echo \"Usage: /etc/init.d/nodejs {start|stop|restart}\" >> /etc/init.d/nodejs"
	sudo sh -c "echo 		exit 1 >> /etc/init.d/nodejs"
	sudo sh -c "echo 		;; >> /etc/init.d/nodejs"
	sudo sh -c "echo esac >> /etc/init.d/nodejs"
	sudo sh -c "echo exit 0 >> /etc/init.d/nodejs"
	sudo chmod u+x /etc/init.d/nodejs
}

install_general

if $SETUP_FREEDNS; then
	install_freedns $FREEDNS_PRIVATE_KEY $HOSTNAME
fi

if $SETUP_MAIL; then
	install_postfix $HOSTNAME
	install_dovecot
fi

if $SETUP_OWNCLOUD; then
	install_owncloud
fi

if $SETUP_NODEJS; then
	install_nodejs
fi
