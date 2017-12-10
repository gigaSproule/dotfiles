dotfiles
===================
A collection of configs and scripts to setup everything I need

Install
-------

1. Run

  ```sh
  cd ~
  git clone https://github.com/gigaSproule/dotfiles.git ~/dotfiles
  cd ~/dotfiles
  sudo XDG_CURRENT_DESKTOP=$XDG_CURRENT_DESKTOP su -c 'pip3 install distro lxml && ./install.py'
  sudo ./install.py
  ```

