FROM rust

RUN apt-get update && apt-get -y install vim qemu-system

RUN useradd -m ulap
USER ulap

# Set environment variables
ENV CARGO_HOME=/home/ulap/.cargo
ENV RUSTUP_HOME=/home/ulap/.rustup

RUN mkdir -p /home/ulap
RUN echo "alias cr=cargo" >> /home/ulap/.bashrc

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



