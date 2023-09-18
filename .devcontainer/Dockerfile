FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld \
   && apt-get autoremove -y && apt-get clean -y

USER vscode
WORKDIR /tmp/cargo-bin
ADD https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-gnu.tgz binstall.tgz
RUN sudo chown vscode:vscode binstall.tgz && tar -xvf binstall.tgz && rm binstall.tgz
RUN ./cargo-binstall -y --root /usr/local/cargo cargo-binstall