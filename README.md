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

## Double faults

```
unsafe {
    *(0xdeadbeef as *mut u8) = 42;
};
```
1. CPU tries to write 0xdeadbeef which causes a page fault.
1. CPU looks at the entry in IDT and sees no handler function is specified.
It cannot call the page fault handler and a double fault occurs. 
1. CPU looks at the IDT entry of double fault handler, but this does not 
specify a handler also, so a triple fault occurs
1. A triple fault is fatal, so QEMU reacts like a real hardware and issues a
system reset. QEMU enters boot loop

## Kernel Stack Overflow

### Guard page
Special memory page at the bottom of the stack. Not mapped to any physical
frame, so accessing it causes a page fault instead of corrupting other memory.
Bootloader sets up a guard page for our kernel stack so a stack overflow causes
a page fault. 

## Switching stacks
X86_64 is able to switch to a predefined known good stack when an exception occurs. 
This happens at the hardware level. 
The switching is done as an **Interrupt Stack Table (IST)**. The  IST is a table of
7 pointers to known good statkcs. 

### **Task State Segment (TSS)** 
TSS is used to hold various bits of about a task in 32 bit mode
and was used for hardware context switching. 
But in x86_64, it no longer holds any task specific information but instead
it holds 2 stack tables (IST) is one of them. 

## Global Descriptor Table GDT

Segmentation is no longer supported in 64 bit mode, but GDT still exists.
GDT is used for switching between kernel and user space and loading a TSS structure

## Page tables
- A pointer to the currently active table is stored in a special CPU register
called CR3 (in x86)
- CR2 register is automatically set by the CPU on page fault