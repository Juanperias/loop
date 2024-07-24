# Loop Kernel

![Rust](https://camo.githubusercontent.com/8e31ce4df532515ac9a1c0418c03b7793471ff9e282dfc28e6473b65334fbac9/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f727573742d2532333030303030302e7376673f7374796c653d666f722d7468652d6261646765266c6f676f3d72757374266c6f676f436f6c6f723d7768697465)

Loop is a monolithic unix based kernel.
which provides full control over the system and is programmed entirely in rust.

Loop runs in protected mode, although its bootloader uses some bios interrupts.

To compile the loop you can do a

```bash
make
```

and with this you will get the loop.bin, and if you only want the kernel is in kernel.bin (inside build)
