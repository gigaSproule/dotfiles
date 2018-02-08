dotfiles
===================
A collection of configs and scripts to setup everything I need

Install
-------
This requires python 3 and pip (for installing distro and lxml)

1. Run

  ```sh
  cd ~
  git clone https://github.com/gigaSproule/dotfiles.git ~/dotfiles
  cd ~/dotfiles
  sudo XDG_CURRENT_DESKTOP=$XDG_CURRENT_DESKTOP su -c 'pip3 install -r requirements.txt && ./install.py [-d | --development | -p | --personal | -s | --server | -v | --vm | -h | --help]'
  ```
