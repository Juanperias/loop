# Loop Kernel

![loop_logo](https://raw.githubusercontent.com/Juanperias/loop/assets/assets/loop_logo.png)

Loop is a monolithic unix based kernel.
which provides full control over the system and is programmed entirely in rust.

Loop runs in protected mode, although its bootloader uses some bios interrupts.

To compile the loop you can do a

```bash
make
```

and with this you will get the loop.bin, and if you only want the kernel is in kernel.bin (inside build)
