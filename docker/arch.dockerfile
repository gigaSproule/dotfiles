FROM archlinux@sha256:63c7b061c0c001cb7ce4f8d11b63d351c23e7f97121bc5c8bd5d9f431e615d7d

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
