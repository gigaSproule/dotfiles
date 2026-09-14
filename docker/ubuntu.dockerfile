FROM ubuntu:26.04@sha256:513c074113a871b51a8d16ab445c88779d6452d937a164fb5cc479f32668a41d

ENV TZ=Europe/London
RUN apt-get update && apt-get install -y build-essential curl libssl-dev locales pkg-config sudo
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
RUN sed -i '/en_GB.UTF-8/s/^# //g' /etc/locale.gen && locale-gen
ENV LANG en_GB.UTF-8
ENV LANGUAGE en_GB:en
ENV LC_ALL en_GB.UTF-8
RUN useradd -ms /bin/bash ubuntuuser \
    && usermod --append --groups sudo ubuntuuser \
    && echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

COPY src/ /app/src/
COPY Cargo.toml Cargo.lock /app/
RUN chown -R ubuntuuser:ubuntuuser /app

USER ubuntuuser

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y &&  \
    echo ". $HOME/.cargo/env" >> $HOME/.bashrc
WORKDIR /app

CMD [ "bash" ]
