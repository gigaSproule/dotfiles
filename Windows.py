from typing import AnyStr, List

from System import System, execute, download_file


class Windows(System):

    def install_applications(self, applications: List[AnyStr]):
        command = ['choco', 'install', '-y']
        command.extend(applications)
        execute(command, root=True)

    def install_system_extras(self):
        download_file('https://chocolatey.org/install.ps1', 'install.ps1')
        execute(['iex', 'install.ps1'])

    def update_os(self):
        self.update_os_repo()
        execute(['choco', 'upgrade', 'chocolatey'], root=True)

    def update_os_repo(self):
        execute(['choco', 'update'], root=True)
