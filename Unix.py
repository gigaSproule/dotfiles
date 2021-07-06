import os
import shutil
from typing import AnyStr

from System import System


class Unix(System):
    def copy_config(self, src, dst):
        actual_src = '%s/%s' % (os.path.dirname(os.path.realpath(__file__)), src)
        actual_dst = '%s/%s' % (self.get_home_dir(), dst)
        self.make_directory(os.path.dirname(actual_dst))
        shutil.copyfile(actual_src, actual_dst)
        self.recursively_chown(actual_dst)

    def is_super_user(self):
        return os.getuid() == 0

    def add_to_path(self, file: AnyStr, path: AnyStr):
        with open(self.get_home_dir() + '/' + file, 'a+') as f:
            f.write('export PATH=$PATH:%s\n' % path)

    def install_rust(self):
        self.download_file('https://sh.rustup.rs', 'rustup-install')
        self.recursively_chmod('rustup-install')
        self.execute(['./rustup-install', '-y'], super_user=False)
        self.delete_file('rustup-install')
        self.add_to_path('.zshrc.custom', '$HOME/.cargo/bin')
        self.add_to_path('.bashrc.custom', '$HOME/.cargo/bin')

    def set_java_home(self, file: AnyStr, jdk_path: AnyStr):
        with open(self.get_home_dir() + '/' + file, 'a+') as f:
            f.seek(0)
            contents = f.read()
            if 'JAVA_HOME' not in contents:
                f.write('export JAVA_HOME=%s\n' % jdk_path)

    def setup_tmux(self):
        self.copy_config('tmux/tmux.conf', '.tmux.conf')

    def setup_zsh(self, zsh_bin: AnyStr = '/usr/bin/zsh'):
        self.download_file('https://raw.githubusercontent.com/loket/oh-my-zsh/feature/batch-mode/tools/install.sh',
                           'oh-my-zsh.sh')
        self.recursively_chmod('./oh-my-zsh.sh')
        self.execute(['./oh-my-zsh.sh'], super_user=False)
        self.copy_config('zsh/zshrc', '.zshrc')
        self.execute(['chsh', '-s', zsh_bin])
        self.execute(['chsh', '-s', zsh_bin, os.getlogin()])
        self.delete_file('oh-my-zsh.sh')

    def symlink(self, source: AnyStr, destination: AnyStr):
        self.execute(['ln', '-sfn', source, destination])
