FROM rust

RUN apt-get update && apt-get -y install vim qemu-system

# To change to nightly, this would be suffience once bug 143072 in rust-lang/rust fixed
# RUN rustup override set nightly

# To change to nightly
RUN rustup install nightly-2025-06-23

RUN rustup default nightly-2025-06-23

# To build core
RUN rustup component add rust-src

# To run bootimage and build the bootloader
RUN rustup component add llvm-tools-preview

RUN rustup target add thumbv7em-none-eabihf

# Set environment variables
RUN useradd -m ulap
USER ulap
RUN mkdir -p /home/ulap
RUN echo "alias cr=cargo" >> /home/ulap/.bashrc

ENV CARGO_HOME=/home/ulap/.cargo
#ENV RUSTUP_HOME=/home/ulap/.rustup

# Compiles kernel and bootloeader and links them together
# to create a bootable diskimage
RUN cargo install bootimage





