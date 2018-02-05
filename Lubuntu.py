import os

import lxml.etree as le

from Ubuntu import Ubuntu


class Lubuntu(Ubuntu):
    def __init__(self):
        super().__init__()

    def install_dropbox(self):
        super().install_dropbox()
        with open('%s/.config/autostart/dropbox.desktop' % os.environ['HOME'], 'w') as f:
            f.write('[Desktop Entry]\n'
                'Name=Dropbox\n'
                'GenericName=File Synchronizer\n'
                'Comment=Sync your files across computers and to the web\n'
                'Exec=dropbox start -i\n'
                'Terminal=false\n'
                'Type=Application\n'
                'Icon=dropbox\n'
                'Categories=Network;FileTransfer;\n'
                'StartupNotify=false\n')

    def install_lutris(self):
        self.install_application('x11-xserver-utils')
        super().install_lutris()

    def set_development_shortcuts(self):
        with open('%s/.config/openbox/lxqt-rc.xml' % os.environ['HOME'], 'r') as f:
            doc = le.parse(f)
            namespace = doc.getroot().nsmap
            namespace['default'] = namespace[None]
            namespace.pop(None)
            xpath_elems_to_remove = []
            # Allow for going back and forth in IntelliJ
            xpath_elems_to_remove.extend(doc.xpath('//default:keyboard/default:keybind[@key="C-A-Left"]',
                                                   namespaces=namespace))
            xpath_elems_to_remove.extend(doc.xpath('//default:keyboard/default:keybind[@key="C-A-Right"]',
                                                   namespaces=namespace))
            xpath_elems_to_remove.extend(doc.xpath('//default:keyboard/default:keybind[@key="C-A-Up"]',
                                                   namespaces=namespace))
            xpath_elems_to_remove.extend(doc.xpath('//default:keyboard/default:keybind[@key="C-A-Down"]',
                                                   namespaces=namespace))
            # Allow for alt + dragging in IntelliJ
            xpath_elems_to_remove.extend(
                doc.xpath(
                    '//default:mouse/default:context[@name="Frame"]/default:mousebind[@button="A-Left" and @action="Press"]',
                    namespaces=namespace))
            xpath_elems_to_remove.extend(
                doc.xpath(
                    '//default:mouse/default:context[@name="Frame"]/default:mousebind[@button="A-Left" and @action="Click"]',
                    namespaces=namespace))
            xpath_elems_to_remove.extend(
                doc.xpath(
                    '//default:mouse/default:context[@name="Frame"]/default:mousebind[@button="A-Left" and @action="Drag"]',
                    namespaces=namespace))
            xpath_elems_to_remove.extend(
                doc.xpath(
                    '//default:mouse/default:context[@name="Frame"]/default:mousebind[@button="A-Right" and @action="Press"]',
                    namespaces=namespace))
            xpath_elems_to_remove.extend(
                doc.xpath(
                    '//default:mouse/default:context[@name="Frame"]/default:mousebind[@button="A-Right" and @action="Drag"]',
                    namespaces=namespace))
            xpath_elems_to_remove.extend(
                doc.xpath(
                    '//default:mouse/default:context[@name="Frame"]/default:mousebind[@button="A-Middle" and @action="Press"]',
                    namespaces=namespace))
            xpath_elems_to_remove.extend(
                doc.xpath(
                    '//default:mouse/default:context[@name="Frame"]/default:mousebind[@button="A-Up" and @action="Click"]',
                    namespaces=namespace))
            xpath_elems_to_remove.extend(
                doc.xpath(
                    '//default:mouse/default:context[@name="Frame"]/default:mousebind[@button="A-Down" and @action="Click"]',
                    namespaces=namespace))
            for elem in xpath_elems_to_remove:
                parent = elem.getparent()
                parent.remove(elem)
            with open('%s/.config/openbox/lxqt-rc.xml' % os.environ['HOME'], 'w') as wf:
                wf.write(le.tostring(doc).decode('UTF-8'))
