FROM archlinux@sha256:82b1b08faae9d61e3e7e13d562f4d09114d939105b0d59ff34140f3bd418593a

RUN pacman -Sy --noconfirm gcc openssl pkgconf sudo
RUN useradd -ms /bin/bash archuser \
    && groupadd sudo \
    && usermod --append --groups archuser,sudo archuser \
    && echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

COPY src/ /app/src/
COPY Cargo.toml Cargo.lock /app/
RUN chown -R archuser:archuser /app

USER archuser

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    echo ". $HOME/.cargo/env" >> $HOME/.bashrc
WORKDIR /app

CMD [ "bash" ]
