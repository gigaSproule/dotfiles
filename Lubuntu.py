from Ubuntu import Ubuntu


class Lubuntu(Ubuntu):
    def __init__(self):
        super().__init__()

    def install_openvpn(self):
        self.install_applications(['openvpn', 'network-manager-openvpn'])
        super().setup_openvpn()
