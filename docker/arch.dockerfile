FROM archlinux

RUN useradd -ms /bin/bash archuser \
    && echo -e "password\npassword" | passwd archuser \
    && groupadd sudo \
    && usermod --append --groups wheel,sudo archuser \
    && usermod --append --groups sudo root \
    && echo '%sudo ALL=(ALL:ALL) ALL' >> /etc/sudoers
RUN pacman -Sy --noconfirm gcc openssl pkgconf sudo
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && echo ". $HOME/.cargo/env" >> $HOME/.bashrc
WORKDIR /app

CMD [ "bash" ]
