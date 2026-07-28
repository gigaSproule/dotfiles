FROM archlinux@sha256:3406a568f45d68f0bef35dc80b3eacec8bda59b0292b2e50d5932ba1667f20cf

RUN pacman -Sy --noconfirm gcc openssl pkgconf sudo
RUN useradd -ms /bin/bash archuser \
    && groupadd sudo \
    && usermod --append --groups archuser,sudo archuser \
    && echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && echo ". $HOME/.cargo/env" >> $HOME/.bashrc
COPY src/ /app/src/
COPY Cargo.toml Cargo.lock /app/
RUN chown -R archuser:archuser /app
WORKDIR /app

USER archuser

CMD [ "bash" ]
