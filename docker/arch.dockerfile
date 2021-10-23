FROM archlinux

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && echo ". $HOME/.cargo/env" >> $HOME/.bashrc && pacman -Sy --noconfirm gcc openssl pkgconf

WORKDIR /app

CMD [ "bash" ]
