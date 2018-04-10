from System import System, execute, download_file


class Windows(System):
    def install_application(self, application):
        self.install_applications([application])

    def install_applications(self, applications):
        command = ['choco', 'install', '-y']
        command.extend(applications)
        execute(command)

    def install_system_dependencies(self):
        download_file('https://chocolatey.org/install.ps1', 'install.ps1')
        execute(['iex', 'install.ps1'])

    def update_os(self):
        self.update_os_repo()
        execute(['choco', 'upgrade', 'chocolatey'])

    def update_os_repo(self):
        execute(['choco', 'update'])
