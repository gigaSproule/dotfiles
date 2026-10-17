FROM ubuntu:26.04@sha256:2e27ff302056b2b96df4a524ce48e72c595abfe3fd7a3d923adf8e782e16896f

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
