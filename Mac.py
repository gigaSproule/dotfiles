from typing import AnyStr, List

from System import execute, download_file
from Unix import Unix


class Mac(Unix):

    def install_applications(self, applications: List[AnyStr]):
        command = ['brew', 'install', '-y']
        command.extend(applications)
        execute(command, root=True)

    def install_system_extras(self):
        download_file('https://raw.githubusercontent.com/Homebrew/install/master/install', 'brew-install')
        execute(['ruby', 'brew-install'])

    def update_os(self):
        self.update_os_repo()
        execute(['brew', '-y', 'upgrade'], root=True)

    def update_os_repo(self):
        execute(['brew', 'update'], root=True)
