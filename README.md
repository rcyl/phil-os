- Run in docker because we need to modify .cargo/config.toml files
```
docker compose run app bash(docker compose file defined the "service" as app)
docker compose build (to rebuild, docker compose run does not rebuild)
```

- To build in linux (native host archi x86, no extra target or cross compile)
```
cargo rustc -- -C link-arg=-nostartfiles (on Linux)
```
- The rust core library is distributed as a precompiled binary, so it is only
valid for the supported host triples but not a custom target

To create the *.bin file
```
cargo bootimage
```

- The bootimage tool does the following
1. Compiles kernel into ELF
1. Compiles bootloader depdency as a standalone executable
1. Links the bytes of the kernel ELF to the bootloader

- Boot with QEMU, (outside docker image)
```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin
```