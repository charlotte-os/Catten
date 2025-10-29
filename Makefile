Limine:
	@if [ ! -d "Limine" ]; then \
		git clone https://codeberg.org/Limine/Limine.git --branch=v10.x-binary --depth=1;\
	fi
	make -C Limine

ovmf-x86_64:
	mkdir -p ovmf-x86_64
	cd ovmf-x86_64 && curl -o OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASEX64_OVMF.fd

build-x86_64-debug: Limine
	cd catten && cargo build --target x86_64-unknown-none
	rm -rf iso_root
	mkdir -p iso_root
	cp -v catten/target/x86_64-unknown-none/debug/catten \
		limine.conf Limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v Limine/BOOTX64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o catten-x86_64-debug.iso
	rm -rf iso_root

run-x86_64-debug: ovmf-x86_64 build-x86_64-debug
	qemu-system-x86_64 -enable-kvm -cpu host -M q35 -m 16G -bios ovmf-x86_64/OVMF.fd -cdrom catten-x86_64-debug.iso -boot d -serial stdio

run-x86_64-debug-multicore: ovmf-x86_64 build-x86_64-debug
	qemu-system-x86_64 -enable-kvm -M q35 -smp 16 -cpu host -m 16G -bios ovmf-x86_64/OVMF.fd -cdrom catten-x86_64-debug.iso -boot d -serial stdio

run-x86_64-debug-numa: ovmf-x86_64 build-x86_64-debug
	qemu-system-x86_64 -enable-kvm -M q35 -cpu host -m 16G -bios ovmf-x86_64/OVMF.fd -cdrom catten-x86_64-debug.iso -boot d -serial stdio -smp 4 -object memory-backend-ram,size=4G,id=m0 -object memory-backend-ram,size=4G,id=m1 -numa node,memdev=m0,cpus=0-1,nodeid=0 -numa node,memdev=m1,cpus=2-3,nodeid=1

run-x86_64-extdb: ovmf-x86_64 build-x86_64-debug
	qemu-system-x86_64 -s -S -M q35 -m 16G -bios ovmf-x86_64/OVMF.fd -cdrom catten-x86_64-debug.iso -boot d -serial stdio -d int -D log.txt -M smm=off

run-x86_64-qdblog: ovmf-x86_64 build-x86_64-debug
	qemu-system-x86_64 -M q35 -m 16G -bios ovmf-x86_64/OVMF.fd -cdrom catten-x86_64-debug.iso -boot d -serial stdio -d int -D log.txt -M smm=off

run-x86_64-log: ovmf-x86_64 build-x86_64-debug
	qemu-system-x86_64 -enable-kvm -M q35 -cpu host -m 16G -bios ovmf-x86_64/OVMF.fd -cdrom catten-x86_64-debug.iso -boot d -serial file:log_x86_64.txt

build-x86_64-release: Limine
	cd catten && cargo build --target x86_64-unknown-none --release
	rm -rf iso_root
	mkdir -p iso_root
	cp -v catten/target/x86_64-unknown-none/release/catten \
		limine.conf Limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v Limine/BOOTX64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o catten-x86_64-release.iso
	rm -rf iso_root

run-x86_64-release: ovmf-x86_64 build-x86_64-release
	qemu-system-x86_64 -enable-kvm -M q35 -cpu host -smp 16 -m 16G -bios ovmf-x86_64/OVMF.fd -cdrom catten-x86_64-release.iso -boot d -serial stdio

check-x86_64:
	cd catten && cargo check --target x86_64-unknown-none

# aarch64

ovmf-aarch64:
	mkdir -p ovmf-aarch64
	cd ovmf-aarch64 && curl -o OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASEAARCH64_QEMU_EFI.fd
build-aarch64-debug: Limine
	cd catten && cargo build --target aarch64-unknown-none
catten-aarch64-debug.iso: build-aarch64-debug
	rm -rf iso_root
	mkdir -p iso_root
	cp -v catten/target/aarch64-unknown-none/debug/catten \
		limine.conf Limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v Limine/BOOTAA64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o catten-aarch64-debug.iso
	rm -rf iso_root
run-aarch64-debug: ovmf-aarch64 catten-aarch64-debug.iso
	qemu-system-aarch64 -M virt -cpu cortex-a76 -device ramfb -device qemu-xhci -device usb-kbd -m 2G -bios ovmf-aarch64/OVMF.fd -cdrom catten-aarch64-debug.iso -boot d
run-aarch64-log: ovmf-aarch64 catten-aarch64-debug.iso
	qemu-system-aarch64 -M virt -cpu cortex-a76 -device ramfb -device qemu-xhci -device usb-kbd -m 2G -bios ovmf-aarch64/OVMF.fd -cdrom catten-aarch64-debug.iso -boot d \
		-serial file:log_aarch64.txt


build-aarch64-release: Limine
	cd catten && cargo build --target aarch64-unknown-none --release
catten-aarch64-release.iso: build-aarch64-release
	rm -rf iso_root
	mkdir -p iso_root
	cp -v catten/target/aarch64-unknown-none/release/catten \
		limine.conf Limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v Limine/BOOTAA64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o catten-aarch64-release.iso
	rm -rf iso_root
run-aarch64-release: ovmf-aarch64 catten-aarch64-release.iso
	qemu-system-aarch64 -M virt -cpu cortex-a72 -device ramfb -device qemu-xhci -device usb-kbd -m 2G -bios ovmf-aarch64/OVMF.fd -cdrom catten-aarch64-release.iso -boot d

# riscv64

ovmf-riscv64:
	mkdir -p ovmf-riscv64
	cd ovmf-riscv64 && curl -o OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASERISCV64_VIRT_CODE.fd && dd if=/dev/zero of=OVMF.fd bs=1 count=0 seek=33554432
build-riscv64-debug:
	cd catten && cargo build --target riscv64gc-unknown-none-elf
catten-riscv64-debug.iso: build-riscv64-debug
	rm -rf iso_root
	mkdir -p iso_root
	cp -v catten/target/riscv64gc-unknown-none-elf/debug/catten \
		limine.conf Limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v Limine/BOOTRISCV64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o catten-riscv64-debug.iso
	rm -rf iso_root
run-riscv64-debug: ovmf-riscv64 catten-riscv64-debug.iso
	qemu-system-riscv64 -M virt -cpu rv64 \
		-device ramfb -device qemu-xhci -device usb-kbd -m 2G -drive if=pflash,unit=0,format=raw,file=ovmf-riscv64/OVMF.fd \
		-device virtio-scsi-pci,id=scsi -device scsi-cd,drive=cd0 -drive id=cd0,format=raw,file=catten-riscv64-debug.iso
run-riscv64-debug-log: ovmf-riscv64 catten-riscv64-debug.iso
	qemu-system-riscv64 -M virt -cpu rv64 \
		-device ramfb -device qemu-xhci -device usb-kbd -m 2G -drive if=pflash,unit=0,format=raw,file=ovmf-riscv64/OVMF.fd \
		-device virtio-scsi-pci,id=scsi -device scsi-cd,drive=cd0 -drive id=cd0,format=raw,file=catten-riscv64-debug.iso \
		-serial file:log_riscv64.txt

build-riscv64-release:
	cd catten && cargo build --target riscv64gc-unknown-none-elf
catten-riscv64-release.iso: build-riscv64-release
	rm -rf iso_root
	mkdir -p iso_root
	cp -v catten/target/riscv64gc-unknown-none-elf/release/catten \
		limine.conf Limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v Limine/BOOTRISCV64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o catten-riscv64-release.iso
	rm -rf iso_root
run-riscv64-release: ovmf-riscv64 catten-riscv64-release.iso
	qemu-system-riscv64 -M virt -cpu rv64 \
		-device ramfb -device qemu-xhci -device usb-kbd -m 2G -drive if=pflash,unit=0,format=raw,file=ovmf-riscv64/OVMF.fd \
		-device virtio-scsi-pci,id=scsi -device scsi-cd,drive=cd0 -drive id=cd0,format=raw,file=catten-riscv64-release.iso

# clean commands

clean:
	cd catten && cargo clean
	rm -rf ovmf-aarch64
	rm -rf ovmf-riscv64
	rm -rf ovmf-x86_64
	rm -f catten-aarch64-debug.iso
	rm -f catten-riscv64-debug.iso
	rm -f catten-x86_64-debug.iso
	rm -f catten-aarch64-release.iso
	rm -f catten-riscv64-release.iso
	rm -f catten-x86_64-release.iso
	rm -f log_aarch64.txt
	rm -f log_riscv64.txt
	rm -f log_x86_64.txt

distclean: clean
	rm -rf Limine ovmf*
