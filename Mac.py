from typing import AnyStr, List

from Unix import Unix


class Mac(Unix):

    def install_applications(self, applications: List[AnyStr]):
        command = ['brew', 'install', '-y']
        command.extend(applications)
        self.execute(command, super_user=True)

    def install_system_extras(self):
        self.download_file('https://raw.githubusercontent.com/Homebrew/install/master/install', 'brew-install')
        self.execute(['ruby', 'brew-install'])

    def update_os(self):
        self.update_os_repo()
        self.execute(['brew', '-y', 'upgrade'], super_user=True)

    def update_os_repo(self):
        self.execute(['brew', 'update'], super_user=True)
