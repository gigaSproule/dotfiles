FROM ubuntu:21.10

ENV DEBIAN_FRONTEND noninteractive
RUN useradd -ms /bin/bash ubuntuuser \
    && echo -e "password\npassword" | passwd ubuntuuser \
    && usermod --append --groups sudo ubuntuuser \
    && echo '%sudo ALL=(ALL:ALL) ALL' >> /etc/sudoers
RUN apt-get update && apt-get install -y build-essential curl libssl-dev sudo
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && echo ". $HOME/.cargo/env" >> $HOME/.bashrc
COPY src/ /app/src/
COPY Cargo.toml Cargo.lock /app/
# RUN chown -R ubuntuuser:ubuntuuser /app
WORKDIR /app

CMD [ "bash" ]
