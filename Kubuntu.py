from Ubuntu import Ubuntu


class Kubuntu(Ubuntu):
    def __init__(self):
        super().__init__()

    def install_system_extras(self):
        self.set_debconf('ttf-mscorefonts-installer', 'msttcorefonts/accepted-mscorefonts-eula')
        self.install_applications('ubuntu-restricted-extras')

    def set_development_shortcuts(self):
        print('Remove setting for alt + mouse dragging moving window')
        print('Remove keyboard shortcuts under Navigation for ctrl + alt + left/right')
        print('Remove keyboard shortcut under System for ctrl + alt + l')
        print('Remove keyboard shortcuts under Windows for ctrl + alt + s, alt + f7')
