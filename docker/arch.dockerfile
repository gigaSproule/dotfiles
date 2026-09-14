FROM archlinux@sha256:204e91950fd364961088a01773eee9012243b7e965fed42b1d82d12416190782

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
