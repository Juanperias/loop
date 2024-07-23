ASM?=nasm
ASM_FLAGS?=-f bin
BUILD_DIR?=build

.PHONY: all bootloader clean

all: bootloader

bootloader: $(BUILD_DIR)/boot.bin

$(BUILD_DIR)/boot.bin:
	$(ASM) $(ASM_FLAGS) boot.asm -o $(BUILD_DIR)/boot.bin

clean:
	rm -f $(BUILD_DIR)/boot.bin
