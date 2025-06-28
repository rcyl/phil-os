FROM rust

RUN echo "alias cr=cargo" >> /root/.bashrc

RUN apt-get update && apt-get -y install vim qemu-system

# To change to nightly
RUN rustup override set nightly

# To build core
RUN rustup component add rust-src

# To run bootimage and build the bootloader
RUN rustup component add llvm-tools-preview

RUN rustup target add thumbv7em-none-eabihf

# Compiles kernel and bootloeader and links them together
# to create a bootable diskimage
RUN cargo install bootimage



