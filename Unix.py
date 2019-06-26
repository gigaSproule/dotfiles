import os
import shutil
from pwd import getpwnam
from typing import AnyStr

from System import System


class Unix(System):
    def copy_config(self, src, dst):
        self.make_directory(os.path.dirname(dst))
        shutil.copyfile('%s/%s' % (os.path.dirname(os.path.realpath(__file__)), src),
                        '%s/%s' % (os.environ['HOME'], dst))

    def is_super_user(self):
        return os.getuid() == 0

    def set_java_home(self, file: AnyStr, jdk_path: AnyStr):
        with open(os.environ['HOME'] + '/' + file, 'a+') as f:
            contents = f.read()
            if 'JAVA_HOME' not in contents:
                f.write('export JAVA_HOME=%s' % jdk_path)

    def setup_git(self):
        self.execute(['git', 'config', '--global', 'user.name', 'Benjamin Sproule'])
        self.execute(['git', 'config', '--global', 'user.email', 'benjamin@benjaminsproule.com'])
        self.execute(['git', 'config', '--global', 'credential.helper', 'cache --timeout=86400'])
        self.make_directory(os.environ['HOME'] + '/.git/hooks')

    def set_user_as_super_user(self):
        os.setuid(0)
        os.setgid(0)

    def set_user_as_normal_user(self):
        current_user = getpwnam(os.getlogin())
        os.setuid(current_user.pw_uid)
        os.setgid(current_user.pw_gid)

    def setup_zsh(self):
        self.execute(['sh', '-c',
                      '"$(curl -fsSL https://raw.githubusercontent.com/loket/oh-my-zsh/feature/batch-mode/tools/install.sh)"',
                      '-s', '--batch', '||', '{', 'echo', '"Could not install Oh My Zsh"', '>/dev/stderr', 'exit', '1',
                      '}'])
        self.execute(['chsh', '-s', '/usr/bin/zsh'])
