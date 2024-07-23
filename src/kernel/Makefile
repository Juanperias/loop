BUILD_DIR?=build
ASM?=nasm
ASMFLAGS?=-f elf32
ELF_LD?=/usr/local/i386elfgcc/bin/i386-elf-ld
RUST?=rustc
RUST_FLAGS?=-O --target i686-unknown-linux-gnu --crate-type lib --emit=obj -C link-args=-no-pie -C relocation-model=static


SOURCES_RUST=$(wildcard *.rs)
SOURCES_ASM=$(wildcard *.asm)
OBJECTS_RUST=$(patsubst %.rs, $(BUILD_DIR)/kernel/rust/%.o, $(SOURCES_RUST))
OBJECTS_ASM=$(patsubst %.asm, $(BUILD_DIR)/kernel/asm/%.o, $(SOURCES_ASM))

all: kernel

kernel: $(BUILD_DIR)/kernel.bin

$(BUILD_DIR)/kernel.bin: $(OBJECTS_RUST) $(OBJECTS_ASM)
	$(ELF_LD) -o $(BUILD_DIR)/kernel.bin -Ttext 0x0 --entry=entry $(OBJECTS_ASM) $(OBJECTS_RUST) --oformat binary

$(BUILD_DIR)/kernel/rust/%.o: %.rs always
	$(RUST) $(RUST_FLAGS) $< -o $@

$(BUILD_DIR)/kernel/asm/%.o: %.asm always
	$(ASM) $(ASMFLAGS) $< -o $@

.PHONY: all kernel clean



always:
	mkdir -p $(BUILD_DIR)/kernel/asm
	mkdir -p $(BUILD_DIR)/kernel/rust

clean:
	rm -f $(BUILD_DIR)/kernel.bin
