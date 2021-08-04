OBJCOPY=arm-none-eabi-objcopy
OBJDUMP=arm-none-eabi-objdump

TARGET=arm-none-eabihf

SOURCES := $(shell find src -name '*.rs')

# Files

.PHONY: build clean listing $(OUT_FILE)

all: clean kernel.img

libmagpie.rlib:
	xargo build --target arm-none-eabihf

kernel.elf: libmagpie.rlib linker.ld boot.o
	arm-none-eabi-gcc -T linker.ld -o kernel.elf -ffreestanding -O2 -nostdlib boot.o target/arm-none-eabihf/debug/libmagpie.rlib

kernel.img: kernel.elf
	$(OBJCOPY) kernel.elf -O binary kernel.img

kernel.list: kernel.img
	$(OBJDUMP) -d kernel.elf > kernel.list

# kernel.elf: src/start.o src/interrupts_asm.s src/main.o
# 	arm-none-eabi-gcc -O0 -g -Wl,-gc-sections -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -nostdlib $^ -o $@

# %.o: %.rs $(SOURCES)
# rustc --target arm-none-eabihf -g -L /Users/mary/.cargo/ --crate-type="staticlib" $< -o $@
# cargo build  --target arm-unknown-linux-gnueabihf -Z unstable-options --out-dir build && cp build/rpi-kernel src/main.o

qemu: clean kernel.elf
	qemu-system-arm -M raspi2 -kernel kernel.elf -serial stdio

%.o: %.s
	arm-none-eabi-as $< -o $@

install: clean kernel.img
	sh install.sh

install-screen: install
	sleep 5
	screen /dev/tty.SLAB_USBtoUART 115200

clean:
	rm -f kernel.img
	rm -f kernel.elf
	rm -f src/*.o
