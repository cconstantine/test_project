FROM rust:1.86-bookworm AS dev

RUN rustup component add rustfmt
RUN rustup target add wasm32-unknown-unknown

RUN mkdir /app
ENV HOME="/app"
WORKDIR /app


#This user schenanigans allows for local development
ARG USER=app
ARG USER_ID=1000
ARG GROUP_ID=1000

RUN groupadd -g ${GROUP_ID} ${USER} && \
    useradd -l -u ${USER_ID} -g ${USER} -s /bin/bash ${USER}

RUN chown ${USER}:${USER} /app
USER ${USER}
