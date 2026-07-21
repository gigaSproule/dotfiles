FROM archlinux@sha256:592e11bd99ab579f933a0cb77a8f66e2f3ae57f5eafacf13aea44a6e98ef21ae

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
