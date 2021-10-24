FROM archlinux

RUN useradd -ms /bin/bash archuser \
    && echo -e "password\npassword" | passwd archuser \
    && groupadd sudo \
    && usermod --append --groups archuser,sudo archuser \
    && echo '%sudo ALL=(ALL:ALL) ALL' >> /etc/sudoers
RUN pacman -Sy --noconfirm gcc openssl pkgconf sudo
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && echo ". $HOME/.cargo/env" >> $HOME/.bashrc
COPY src/ /app/src/
COPY Cargo.toml Cargo.lock /app/
RUN chown -R archuser:archuser /app
WORKDIR /app

CMD [ "bash" ]
