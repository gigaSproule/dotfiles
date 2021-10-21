FROM archlinux

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && source $HOME/.cargo/env && pacman -Sy gcc openssl pkg-conf

CMD [ "bash" ]
