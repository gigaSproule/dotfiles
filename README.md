# dotfiles
A collection of configs and scripts to setup everything I need

## Install
This requires python 3 and pip (for installing distro and lxml)

1. Run

  ```sh
  cd ~
  git clone https://github.com/gigaSproule/dotfiles.git ~/dotfiles
  cd ~/dotfiles
  sudo XDG_CURRENT_DESKTOP=$XDG_CURRENT_DESKTOP su -c 'pip3 install -r requirements.txt && ./install.py [-d | --development | -p | --personal | -s | --server | -v | --vm | -h | --help]'
  ```

## Install Root CA
### Ubuntu based OS
1. Visit the website with the required root certificate
2. `view certificate > details > select intermediate root CA > export` save as `~/Downloads/${CERT_NAME}.pem`

```bash
cd /usr/share/ca-certificates
sudo mkdir extra
sudo cp ~/Downloads/${CERT_NAME}.pem ./extra
sudo openssl x509 -in root-ca.pem -inform PEM -out ${CERT_NAME}.crt
sudo dpkg-reconfigure ca-certificates
sudo keytool -keystore ${JAVA_HOME}/jre/lib/security/cacerts -importcert -alias ${CERT_NAME} -file /usr/share/ca-certificates/extra/${CERT_NAME}.crt
sudo keytool -list -keystore ${JAVA_HOME}/jre/lib/security/cacerts | grep ${CERT_NAME} 
```

### Firefox
1. Go to `Preferences/Options > Privacy & Security > View Certificates > Authorities`
2. Click on `Import`
3. Navigate to `/usr/share/ca-certificates/extra`
4. Select `${CERT_NAME}.crt`
5. Check `Trust this CA to identify websites`
6. Click `OK`
7. Click `OK`

### Chrome/Chromium
1. Go to `Settings > Advanced > Manage certificates > Authorities`
2. Click on `Import`
3. Navigate to `/usr/share/ca-certificates/extra`
4. Select `${CERT_NAME}.crt`
5. Check `Trust this certificate for identifying websites`
6. Click `OK`

### IntelliJ
1. Go to `File > Settings > Tools > Server Certificates > Accpeted Certificates`
2. Click on the `+`
3. Navigate to `/usr/share/ca-certificates/extra`
4. Select `${CERT_NAME}.crt`
5. Click `OK`

### AWS CLI
```bash
export REQUESTS_CA_BUNDLE=/etc/ssl/certs/ca-certificates.crt
```

### Azure CLI
```bash
export AZURE_CLI_DISABLE_CONNECTION_VERIFICATION=1
```
