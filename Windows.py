import ctypes
from typing import AnyStr, List

from System import System


class Windows(System):

    def is_super_user(self):
        return ctypes.windll.shell32.IsUserAnAdmin() == 1

    def install_applications(self, applications: List[AnyStr]):
        command = ['choco', 'install', '-y']
        command.extend(applications)
        self.execute(command, super_user=True)

    def install_system_extras(self):
        self.download_file('https://chocolatey.org/install.ps1', 'install.ps1')
        self.execute(['iex', 'install.ps1'], super_user=True)

    def update_os(self):
        self.update_os_repo()
        self.execute(['choco', 'upgrade', 'chocolatey'], super_user=True)

    def update_os_repo(self):
        self.execute(['choco', 'update'], super_user=True)
