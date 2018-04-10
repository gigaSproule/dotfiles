from System import execute, download_file
from Unix import Unix


class Mac(Unix):
    def install_application(self, application):
        self.install_applications([application])

    def install_applications(self, applications):
        command = ['brew', 'install', '-y']
        command.extend(applications)
        execute(command)

    def install_system_dependencies(self):
        download_file('https://raw.githubusercontent.com/Homebrew/install/master/install', 'brew-install')
        execute(['ruby', 'brew-install'])

    def update_os(self):
        self.update_os_repo()
        execute(['brew', '-y', 'upgrade'])

    def update_os_repo(self):
        execute(['brew', 'update'])
