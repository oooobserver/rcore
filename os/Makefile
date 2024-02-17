.PHONY: gdb make_gdb run os build

run:
	qemu-system-riscv64 \
	-machine virt \
	-nographic \
	-bios ../bootloader/rustsbi-qemu.bin \
	-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \

make_gdb:
	qemu-system-riscv64 \
	-machine virt \
	-nographic \
	-bios ../bootloader/rustsbi-qemu.bin \
	-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
	-s -S

gdb:
	riscv64-unknown-elf-gdb \
	-ex 'file target/riscv64gc-unknown-none-elf/release/os' \
	-ex 'set arch riscv:rv64' \
	-ex 'target remote localhost:1234'

#  Log Level:
#	Default value is 2
# 	0: trace
#	1: debug
#	2: info
#	3: warn
#	4: error

level ?= 2
build:
	LOG_LEVEL=$(level) cargo build --release  && rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin

os:
	make build level=$(level) && make run





