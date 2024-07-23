ASM=nasm
BUILD_DIR=build
SRC_DIR=src

.PHONY: all floppy bootloader clean always

#
# Making floppy image
#

floppy: $(BUILD_DIR)/loop.img

$(BUILD_DIR)/loop.img: bootloader kernel
	dd if=/dev/zero of=$(BUILD_DIR)/loop.img bs=512 count=2880
	mkfs.fat -F 12 -n "LOOP" $(BUILD_DIR)/loop.img
	dd if=$(BUILD_DIR)/boot.bin of=$(BUILD_DIR)/loop.img conv=notrunc
	mcopy -i $(BUILD_DIR)/loop.img $(BUILD_DIR)/kernel.bin "::kernel.bin"

#
# Making the bootloader
#

bootloader: $(BUILD_DIR)/boot.bin

$(BUILD_DIR)/boot.bin: always
	$(MAKE) -C $(SRC_DIR)/boot BUILD_DIR=$(abspath $(BUILD_DIR))

kernel: $(BUILD_DIR)/kernel.bin

$(BUILD_DIR)/kernel.bin: always
	$(MAKE) -C $(SRC_DIR)/kernel BUILD_DIR=$(abspath $(BUILD_DIR))

clean:
	$(MAKE) -C $(SRC_DIR)/boot BUILD_DIR=$(abspath $(BUILD_DIR)) clean
	$(MAKE) -C $(SRC_DIR)/kernel BUILD_DIR=$(abspath $(BUILD_DIR)) clean

always:
	mkdir -p $(BUILD_DIR)
