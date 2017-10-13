from LinuxCommands import execute


class DistroInstaller:
    def __init__(self):
        pass

    def install(self):
        if execute(['pip3', '-V']) == 0:
            execute(['pip3', 'install', 'distro'])
        else:
            execute(['pip', 'install', 'distro'])
