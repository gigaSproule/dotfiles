# dotfiles
A collection of configs and scripts to setup everything I need

## Install
This requires python 3 and pip (for installing distro and lxml)

1. Run
```bash
cd ~
git clone https://github.com/gigaSproule/dotfiles.git ~/dotfiles
cd ~/dotfiles
sudo XDG_CURRENT_DESKTOP=${XDG_CURRENT_DESKTOP} su -c 'pip3 install -r requirements.txt && ./install.py [-d | --development | -p | --personal | -s | --server | -v | --vm | -h | --help]'
```

## Install Root CA Certificate
### Get Root CA Certificate
1. Visit the website with the required root certificate
2. Firefox: `Advanced > Add Exception > View Certificate > Details > Select intermediate root CA > Export` save as `~/Downloads/${CERT_NAME}.pem`
3. Chrome: `Toolbar > View Certificate > Details > Select intermediate root CA > Export` save as `~/Downloads/${CERT_NAME}.pem`

### Convert to .crt from .pem
```bash
sudo openssl x509 -in ~/Downloads/${CERT_NAME}.pem -inform PEM -out ~/Downloads/${CERT_NAME}.crt
```

### Arch Linux
```bash
sudo trust anchor --store ~/Downloads/${CERT_NAME}.crt
```

### Ubuntu based OS
```bash
sudo cp ~/Downloads/${CERT_NAME}.crt /usr/local/share/ca-certificates/
sudo update-ca-certificates
```

### Java
```bash
sudo keytool -keystore ${JAVA_HOME}/lib/security/cacerts -importcert -alias ${CERT_NAME} -file /usr/local/share/ca-certificates/${CERT_NAME}.crt
sudo keytool -list -keystore ${JAVA_HOME}/lib/security/cacerts | grep ${CERT_NAME}
```

### Firefox
#### Using certutil
```bash
certutil -d "$HOME/.mozilla/firefox/*.default" -A -i ~/Downloads/${CERT_NAME}.crt -n "${NICKNAME}" -t C,,
```

#### Manual
1. Go to `Preferences/Options > Privacy & Security > View Certificates > Authorities`
2. Click on `Import`
3. Navigate to `~/Downloads`
4. Select `${CERT_NAME}.crt`
5. Check `Trust this CA to identify websites`
6. Click `OK`
7. Click `OK`

### Chrome/Chromium
#### Using certutil
```bash
certutil -d "sql:$HOME/.pki/nssdb" -A -i ~/Downloads/${CERT_NAME}.crt -n "${NICKNAME}" -t C,,
```

#### Manual
1. Go to `Settings > Advanced > Manage certificates > Authorities`
2. Click on `Import`
3. Navigate to `~/Downloads`
4. Select `${CERT_NAME}.crt`
5. Check `Trust this certificate for identifying websites`
6. Click `OK`

### IntelliJ
1. Go to `File > Settings > Tools > Server Certificates > Accpeted Certificates`
2. Click on the `+`
3. Navigate to `~/Downloads`
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
