
kernel = target/riscv64gc-unknown-none-elf/debug/firefly

QEMU = qemu-system-riscv64

QEMUOPTS = -machine virt -cpu rv64 -smp 2 -m 8M -drive if=none,format=raw,file=hdd.img,id=foo -device virtio-blk-device,scsi=off,drive=foo -nographic -serial mon:stdio -bios none -device virtio-rng-device -device virtio-gpu-device -device virtio-net-device -device virtio-tablet-device -device virtio-keyboard-device -kernel $(kernel)

# try to generate a unique GDB port
#GDBPORT = $(shell expr `id -u` % 5000 + 25000)

OBJDUMP = riscv64-unknown-elf-objdump
OBJCOPY = riscv64-unknown-elf-objcopy

GDBPORT = 1234
# QEMU's gdb stub command line changed in 0.11
QEMUGDB = $(shell if $(QEMU) -help | grep -q '^-gdb'; \
	then echo "-gdb tcp::$(GDBPORT)"; \
	else echo "-s -p $(GDBPORT)"; fi)

hdd:
	dd if=/dev/zero of=hdd.img count=16 bs=1M

qemu-gdb: $(kernel) hdd
	$(OBJDUMP) -S $(kernel) > target/firefly.asm
	$(OBJCOPY) --only-keep-debug $(kernel) target/firefly.sym
	@echo "*** Now run 'gdb' in another window." 1>&2
	$(QEMU) $(QEMUOPTS) -S $(QEMUGDB)
	rm target/firefly.asm target/firefly.sym

clean:
	-rm -R target hdd.img Cargo.lock dep_graph.png