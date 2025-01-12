#x86_64

default: run-x86_64-debug

install-requirements:
	chmod +x ./tools/install-requirements.sh
	./tools/install-requirements.sh

limine:
	@if [ ! -d "limine" ]; then \
		git clone https://github.com/limine-bootloader/limine.git --branch=v8.x-binary --depth=1;\
	fi
	make -C limine

ovmf-x86_64:
	mkdir -p ovmf-x86_64
	cd ovmf-x86_64 && curl -o OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASEX64_OVMF.fd

build-x86_64-debug: limine
	cd nova && cargo build --target x86_64-unknown-none
	rm -rf iso_root
	mkdir -p iso_root
	cp -v nova/target/x86_64-unknown-none/debug/nova \
		limine.conf limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v limine/BOOTX64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o nova-x86_64-debug.iso
	rm -rf iso_root

run-x86_64-debug: ovmf-x86_64 build-x86_64-debug
	qemu-system-x86_64 -enable-kvm -cpu host -M q35 -m 2G -bios ovmf-x86_64/OVMF.fd -cdrom nova-x86_64-debug.iso -boot d -serial stdio

run-x86_64-debug-multicore: ovmf-x86_64 build-x86_64-debug
	qemu-system-x86_64 -enable-kvm -M q35 -smp 8 -cpu host -m 2G -bios ovmf-x86_64/OVMF.fd -cdrom nova-x86_64-debug.iso -boot d -serial stdio

run-x86_64-debug-numa: ovmf-x86_64 build-x86_64-debug
	qemu-system-x86_64 -enable-kvm -M q35 -cpu host -m 8G -bios ovmf-x86_64/OVMF.fd -cdrom nova-x86_64-debug.iso -boot d -serial stdio -smp 4 -object memory-backend-ram,size=4G,id=m0 -object memory-backend-ram,size=4G,id=m1 -numa node,memdev=m0,cpus=0-1,nodeid=0 -numa node,memdev=m1,cpus=2-3,nodeid=1

run-x86_64-extdb: ovmf-x86_64 build-x86_64-debug
	qemu-system-x86_64 -s -S -M q35 -m 2G -bios ovmf-x86_64/OVMF.fd -cdrom nova-x86_64-debug.iso -boot d -serial stdio -d int -D log.txt -M smm=off

run-x86_64-qdblog: ovmf-x86_64 build-x86_64-debug
	qemu-system-x86_64 -M q35 -m 2G -bios ovmf-x86_64/OVMF.fd -cdrom nova-x86_64-debug.iso -boot d -serial stdio -d int -D log.txt -M smm=off

run-x86_64-log: ovmf-x86_64 build-x86_64-debug
	qemu-system-x86_64 -enable-kvm -M q35 -cpu host -m 12G -bios ovmf-x86_64/OVMF.fd -cdrom nova-x86_64-debug.iso -boot d -serial file:log_x86_64.txt

build-x86_64-release: limine
	cd nova && cargo build --target x86_64-unknown-none --release
	rm -rf iso_root
	mkdir -p iso_root
	cp -v nova/target/x86_64-unknown-none/release/nova \
		limine.conf limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v limine/BOOTX64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o nova-x86_64-release.iso
	rm -rf iso_root

run-x86_64-release: ovmf-x86_64 build-x86_64-release
	qemu-system-x86_64 -enable-kvm -M q35 -cpu host -m 12G -bios ovmf-x86_64/OVMF.fd -cdrom nova-x86_64-release.iso -boot d

check-x86_64:
	cd nova && cargo check --target x86_64-unknown-none

# aarch64

ovmf-aarch64:
	mkdir -p ovmf-aarch64
	cd ovmf-aarch64 && curl -o OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASEAARCH64_QEMU_EFI.fd
build-aarch64-debug: limine
	cd nova && cargo build --target aarch64-unknown-none
nova-aarch64-debug.iso: build-aarch64-debug
	rm -rf iso_root
	mkdir -p iso_root
	cp -v nova/target/aarch64-unknown-none/debug/nova \
		limine.conf limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v limine/BOOTAA64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o nova-aarch64-debug.iso
	rm -rf iso_root
run-aarch64-debug: ovmf-aarch64 nova-aarch64-debug.iso
	qemu-system-aarch64 -M virt -cpu cortex-a72 -device ramfb -device qemu-xhci -device usb-kbd -m 2G -bios ovmf-aarch64/OVMF.fd -cdrom nova-aarch64-debug.iso -boot d
run-aarch64-log: ovmf-aarch64 nova-aarch64-debug.iso
	qemu-system-aarch64 -M virt -cpu cortex-a72 -device ramfb -device qemu-xhci -device usb-kbd -m 2G -bios ovmf-aarch64/OVMF.fd -cdrom nova-aarch64-debug.iso -boot d \
		-serial file:log_aarch64.txt


build-aarch64-release: limine
	cd nova && cargo build --target aarch64-unknown-none --release
nova-aarch64-release.iso: build-aarch64-release
	rm -rf iso_root
	mkdir -p iso_root
	cp -v nova/target/aarch64-unknown-none/release/nova \
		limine.conf limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v limine/BOOTAA64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o nova-aarch64-release.iso
	rm -rf iso_root
run-aarch64-release: ovmf-aarch64 nova-aarch64-release.iso
	qemu-system-aarch64 -M virt -cpu cortex-a72 -device ramfb -device qemu-xhci -device usb-kbd -m 2G -bios ovmf-aarch64/OVMF.fd -cdrom nova-aarch64-release.iso -boot d

# riscv64

ovmf-riscv64:
	mkdir -p ovmf-riscv64
	cd ovmf-riscv64 && curl -o OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASERISCV64_VIRT_CODE.fd && dd if=/dev/zero of=OVMF.fd bs=1 count=0 seek=33554432
build-riscv64-debug:
	cd nova && cargo build --target riscv64gc-unknown-none-elf
nova-riscv64-debug.iso: build-riscv64-debug
	rm -rf iso_root
	mkdir -p iso_root
	cp -v nova/target/riscv64gc-unknown-none-elf/debug/nova \
		limine.conf limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v limine/BOOTRISCV64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o nova-riscv64-debug.iso
	rm -rf iso_root
run-riscv64-debug: ovmf-riscv64 nova-riscv64-debug.iso
	qemu-system-riscv64 -M virt -cpu rv64 \
		-device ramfb -device qemu-xhci -device usb-kbd -m 2G -drive if=pflash,unit=0,format=raw,file=ovmf-riscv64/OVMF.fd \
		-device virtio-scsi-pci,id=scsi -device scsi-cd,drive=cd0 -drive id=cd0,format=raw,file=nova-riscv64-debug.iso
run-riscv64-debug-log: ovmf-riscv64 nova-riscv64-debug.iso
	qemu-system-riscv64 -M virt -cpu rv64 \
		-device ramfb -device qemu-xhci -device usb-kbd -m 2G -drive if=pflash,unit=0,format=raw,file=ovmf-riscv64/OVMF.fd \
		-device virtio-scsi-pci,id=scsi -device scsi-cd,drive=cd0 -drive id=cd0,format=raw,file=nova-riscv64-debug.iso \
		-serial file:log_riscv64.txt

build-riscv64-release:
	cd nova && cargo build --target riscv64gc-unknown-none-elf
nova-riscv64-release.iso: build-riscv64-release
	rm -rf iso_root
	mkdir -p iso_root
	cp -v nova/target/riscv64gc-unknown-none-elf/release/nova \
		limine.conf limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v limine/BOOTRISCV64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o nova-riscv64-release.iso
	rm -rf iso_root
run-riscv64-release: ovmf-riscv64 nova-riscv64-release.iso
	qemu-system-riscv64 -M virt -cpu rv64 \
		-device ramfb -device qemu-xhci -device usb-kbd -m 2G -drive if=pflash,unit=0,format=raw,file=ovmf-riscv64/OVMF.fd \
		-device virtio-scsi-pci,id=scsi -device scsi-cd,drive=cd0 -drive id=cd0,format=raw,file=nova-riscv64-release.iso

# clean commands

clean:
	cd nova && cargo clean
	rm -rf ovmf-aarch64
	rm -rf ovmf-riscv64
	rm -rf ovmf-x86_64
	rm -f nova-aarch64-debug.iso
	rm -f nova-riscv64-debug.iso
	rm -f nova-x86_64-debug.iso
	rm -f nova-aarch64-release.iso
	rm -f nova-riscv64-release.iso
	rm -f nova-x86_64-release.iso
	rm -f log_aarch64.txt
	rm -f log_riscv64.txt
	rm -f log_x86_64.txt

distclean: clean
	rm -rf limine
