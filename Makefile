ASM=nasm
BUILD_DIR=build
SRC_DIR=src

.PHONY: all bin bootloader clean always

#
# make bin file
#

bin: $(BUILD_DIR)/loop.bin

$(BUILD_DIR)/loop.bin: bootloader kernel zeroes
	cat $(BUILD_DIR)/boot.bin $(BUILD_DIR)/kernel.bin  $(BUILD_DIR)/zeroes.bin > $(BUILD_DIR)/loop.bin
#
# Making the bootloader
#

bootloader: $(BUILD_DIR)/boot.bin

$(BUILD_DIR)/boot.bin: always
	$(MAKE) -C $(SRC_DIR)/boot BUILD_DIR=$(abspath $(BUILD_DIR))

#
# Making the kernel
#

kernel: $(BUILD_DIR)/kernel.bin

$(BUILD_DIR)/kernel.bin: always
	$(MAKE) -C $(SRC_DIR)/kernel BUILD_DIR=$(abspath $(BUILD_DIR))

#
# Making zeroes
#

zeroes: $(BUILD_DIR)/zeroes.bin

$(BUILD_DIR)/zeroes.bin: always
	$(MAKE) -C $(SRC_DIR)/zeroes BUILD_DIR=$(abspath $(BUILD_DIR))

#
# Utils
#

clean:
	$(MAKE) -C $(SRC_DIR)/boot BUILD_DIR=$(abspath $(BUILD_DIR)) clean
	$(MAKE) -C $(SRC_DIR)/kernel BUILD_DIR=$(abspath $(BUILD_DIR)) clean
	$(MAKE) -C $(SRC_DIR)/zeroes BUILD_DIR=$(abspath $(BUILD_DIR)) clean

always:
	mkdir -p $(BUILD_DIR)
