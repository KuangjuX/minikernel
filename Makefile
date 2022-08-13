TARGET      := riscv64imac-unknown-none-elf
MODE        := debug
KERNEL_FILE := target/$(TARGET)/$(MODE)/minikernel
BIN_FILE    := target/$(TARGET)/$(MODE)/minikernel.bin
CPUS		:= 3

FS_IMG		:= ../fs.img
KERNEL_ASM	:= kernel.S

OBJCOPY = riscv64-unknown-elf-objcopy
OBJDUMP = riscv64-unknown-elf-objdump

QEMU 		:= qemu-system-riscv64

FWDPORT = $(shell expr `id -u` % 5000 + 25999)

QEMUOPTS     = -machine virt -bios none -kernel $(KERNEL_FILE) -m 3G -smp $(CPUS) -nographic
QEMUOPTS    += -drive file=${FS_IMG},if=none,format=raw,id=x0 
QEMUOPTS	+= -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0
QEMUOPTS 	+= -netdev user,id=net0,hostfwd=udp::$(FWDPORT)-:2000 -object filter-dump,id=net0,netdev=net0,file=packets.pcap
QEMUOPTS 	+= -device e1000,netdev=net0,bus=pcie.0


QEMUGDB 	:= -gdb tcp::26000

GDB         := ~/riscv/riscv64-unknown-elf-gcc-8.3.0-2020.04.1-x86_64-linux-ubuntu14/bin/riscv64-unknown-elf-gdb

.PHONY: doc kernel build clean qemu run test

# 默认 build 为输出二进制文件
build: $(BIN_FILE) 

# 通过 Rust 文件中的注释生成 os 的文档
doc:
	@cargo doc --document-private-items

# 编译 kernel
kernel:
	@cargo build


$(BIN_FILE): kernel
	@$(OBJCOPY) $(KERNEL_FILE) --strip-all -O binary $@


asm: kernel
	@$(OBJDUMP) -d $(KERNEL_FILE) > $(KERNEL_ASM)

clean:
	@cargo clean
	@rm -rf kernel.S


qemu: build
	$(QEMU) $(QEMUOPTS)

qemu-gdb:
	@cargo build
	@echo "*** Now run 'gdb' in another window." 1>&2
	$(QEMU) $(QEMUOPTS) -S $(QEMUGDB)

# 一键运行
run: build qemu

test: build
	@cargo test


debug: build
	@tmux new-session -d \
		"$(QEMU) $(QEMUOPTS) -s -S" && \
		tmux split-window -h "$(GDB) -ex 'file $(KERNEL_FILE)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d