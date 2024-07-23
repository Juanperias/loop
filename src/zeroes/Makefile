ASM?=nasm
BUILD_DIR?=build
ASM_FLAGS?=-f bin

.PHONY: all zeroes clean

all: zeroes

zeroes: $(BUILD_DIR)/zeroes.bin

$(BUILD_DIR)/zeroes.bin:
	$(ASM) $(ASM_FLAGS) zeroes.asm -o $(BUILD_DIR)/zeroes.bin

clean:
	rm -f $(BUILD_DIR)/zeroes.bin
