# Base image
ARG VARIANT=bookworm-slim
FROM debian:${VARIANT}
ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8

# Arguments
ARG CONTAINER_USER=milkv
ARG CONTAINER_GROUP=milkv
ARG GITHUB_TOKEN

# Install dependencies
RUN apt-get update \
    && apt-get install -y git curl llvm-dev libclang-dev clang unzip gdb-multiarch sudo \
    libssl-dev pkg-config qemu-user file cpio git ssh make wget binutils-riscv64-unknown-elf sshpass iputils-ping \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts

# Set users
RUN adduser --disabled-password --gecos "" ${CONTAINER_USER}
RUN adduser ${CONTAINER_USER} sudo
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}

# Install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain none -y --profile minimal

# Update envs
ENV PATH=${PATH}:/home/${CONTAINER_USER}/.cargo/bin

# Set default toolchain
RUN rustup toolchain install nightly
RUN rustup default nightly
#RUN rustup target add --toolchain nightly armv7-unknown-linux-gnueabihf
#RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN rustup component add clippy
RUN rustup component add rustfmt

RUN cargo install svd2rust form

# Install debugging server (see https://community.milkv.io/t/jtag-jtag-debug-guide-for-duo/1138)
RUN wget https://occ-oss-prod.oss-cn-hangzhou.aliyuncs.com/resource//1698310757460/T-Head-DebugServer-linux-x86_64-V5.18.0-20230926.sh.tar.gz
RUN tar xf T-Head-DebugServer-linux-x86_64-V5.18.0-20230926.sh.tar.gz
RUN printf 'yes\n\nyes\n' | sudo ./T-Head-DebugServer-linux-x86_64-V5.18.0-20230926.sh -i
RUN rm T-Head-DebugServer-linux-x86_64-V5.18.0-20230926.*

CMD [ "/bin/bash" ]