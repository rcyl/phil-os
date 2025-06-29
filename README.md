# README
- RiscV 
1. https://blog.henrygressmann.de/rust-os/1-hello-riscv/
1. https://osblog.stephenmarz.com/

- Run in docker because we need to modify .cargo/config.toml files
```
MYID="$(id -u):$(id -g)" docker compose run app bash(docker compose file defined the "service" as app)
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

- isa-debug-exit device uses port mapped IO
```
[package.metadata.bootimage]
test-args = ["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"]
```
0xf4 is generally unused on x86 IO bus and iosize specifices port size (4 bytes)

- print function name
```
core::any::type_name::<T>()
```

```
cargo test --lib
```
## Interrupt Descriptor Table

- Specifies a handler for each CPU exception

## Interrupt Calling Convention
- A function call is invoked voluntarily by a compiler inserted call instruction,
while an exception might occur at any instruction

### System V ABI (for C functions)
1. the first six integer arguments are passed in registers rdi, rsi, rdx, rcx, r8, r9
1. additional arguments are passed on the stack
1. results are returned in rax and rdx

#### Preserved and Scratch Registers
- Values of **preserved registers** must remain unchanged across function calls
- So a called function , aka **callee** is only allowed to overwrite these register
if it restores their original values before returning 
- A common patten is to save these registers to the stack at the functions
beginning and restore them just before returning. 
- A called function is allowed to ovewrite **scratch registers** without restrictions
- If the caller wants to preserve the value of a scratch register across a function call, 
it needs to backup and restore it before the function call (e.g., by pushing it to the stack)
- Scratch register are **caller-saved**.

- On x86_64, C calling convetion specifies the following:

| preserved registers | scratch registers |
| --------- | ------- | 
| rbp, rbx, rsp, r12, r13, r14, r15 | rax, rcx, rdx, rsi, rdi, r8, r9, r10, r11 |  
| calle-saved                       | caller-saved |

#### Preserving all registers
- For interrups, we need a calling convention that preserves all registers
- **x86-interrupt** calling convention is such a convention as it guarantees
that all register values are restored to the original values on function return
- But this does not mean all registers are saved to the stack, the compiler
only backs up the registers that are overwritten by the function