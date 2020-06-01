from typing import AnyStr, List

from Unix import Unix


class Mac(Unix):
    def app_store_install_application(self, application_id: AnyStr):
        command = ['mas', 'install', application_id]
        self.execute(command, super_user=False)

    def cask_install_application(self, application: AnyStr):
        self.cask_install_applications([application])

    def cask_install_applications(self, applications: List[AnyStr]):
        command = ['brew', 'cask', 'install']
        command.extend(applications)
        self.execute(command, super_user=False)

    def install_applications(self, applications: List[AnyStr]):
        command = ['brew', 'install']
        command.extend(applications)
        self.execute(command, super_user=False)

    def install_android_studio(self):
        self.cask_install_application('android-studio')
        with open(self.get_home_dir() + '/.custom.sh', 'a+') as f:
            f.write('alias studio="open -a /Applications/Android\\ Studio.app"')

    def install_chrome(self):
        self.cask_install_application('google-chrome')

    def install_docker(self):
        self.cask_install_application('docker')

    def install_dropbox(self):
        self.cask_install_application('dropbox')

    def install_eclipse(self):
        self.cask_install_application('eclipse-java')

    def install_firefox(self):
        self.cask_install_application('firefox')

    def install_gradle(self):
        self.install_applications(['gradle', 'gradle-completion'])

    def install_git(self):
        self.install_application('git')

    def install_gpg(self):
        self.cask_install_application('gpg-suite')

    def install_google_cloud_sdk(self):
        self.cask_install_application('google-cloud-sdk')
        # source "/usr/local/Caskroom/google-cloud-sdk/latest/google-cloud-sdk/path.zsh.inc"
        # source "/usr/local/Caskroom/google-cloud-sdk/latest/google-cloud-sdk/completion.zsh.inc"

    def install_groovy(self):
        self.install_application('groovy')

    def install_helm(self):
        self.install_application('helm')

    def install_intellij(self):
        self.cask_install_application('intellij-idea')

    def install_java(self):
        self.install_application('openjdk')
        self.symlink('$(brew --prefix)/opt/openjdk/libexec/openjdk.jdk',
                     '/Library/Java/JavaVirtualMachines/openjdk.jdk')

    def install_keepassxc(self):
        self.cask_install_application('keepassxc')

    def install_kubectl(self):
        self.install_application('kubernetes-cli')

    def install_maven(self):
        self.install_application('maven')

    def install_minikube(self):
        self.install_application('minikube')

    def install_nextcloud_client(self):
        self.cask_install_application('nextcloud')

    def install_nodejs(self):
        self.install_applications(['node', 'yarn'])

    def install_nordvpn(self):
        self.app_store_install_application('1116599239')

    def install_obs_studio(self):
        self.cask_install_application('obs')

    def install_python(self):
        self.install_application('python')

    def install_slack(self):
        self.app_store_install_application('803453959')

    def install_spotify(self):
        self.cask_install_application('spotify')

    def install_sweet_home_3d(self):
        self.cask_install_application('sweet-home3d')

    def install_tmux(self):
        self.install_applications(['tmux', 'reattach-to-user-namespace'])
        self.setup_tmux()

    def install_vlc(self):
        self.cask_install_application('vlc')

    def install_vscode(self):
        self.cask_install_application('visual-studio-code')

    def install_wget(self):
        self.install_application('wget')

    def install_xcode(self):
        self.app_store_install_application('497799835')

    def install_zsh(self):
        self.install_applications(['zsh', 'zsh-autosuggestions'])
        self.setup_zsh('/usr/local/bin/zsh')

    def setup_tmux(self):
        super().setup_tmux()
        with open(self.get_home_dir() + '/.tmux.conf.custom', 'a+') as f:
            f.write('bind -T copy-mode-vi y send-keys -X copy-pipe-and-cancel \'reattach-to-user-namespace pbcopy\'')

    def install_system_extras(self):
        import ssl
        ssl._create_default_https_context = ssl._create_unverified_context
        self.download_file('https://raw.githubusercontent.com/Homebrew/install/master/install.sh', 'brew-install')
        self.execute(['chmod', '+x', 'brew-install'])
        self.execute(['./brew-install'], super_user=False)
        self.delete_file('brew-install')

        self.install_application('mas')

    def update_os(self):
        self.update_os_repo()
        self.execute(['brew', '-y', 'upgrade'])

    def update_os_repo(self):
        self.execute(['brew', 'update'])
