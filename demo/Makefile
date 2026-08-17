KERNEL := target/x86_64-unknown-none/debug/demo
ISO := build/segterm.iso
BUILD_DIR := build

.PHONY: default build iso run run-uefi debug clean

default: run

build:
	cargo build --target x86_64-unknown-none

iso: build
	mkdir -p $(BUILD_DIR)/iso/boot
	cp $(KERNEL) $(BUILD_DIR)/iso/boot/segterm-demo
	cp limine.conf $(BUILD_DIR)/iso/boot/limine.conf

	cp extern/limine/limine-bios-cd.bin $(BUILD_DIR)/iso/
	cp extern/limine/limine-uefi-cd.bin $(BUILD_DIR)/iso/
	cp extern/limine/limine-bios.sys $(BUILD_DIR)/iso/

	mkdir -p $(BUILD_DIR)/iso/EFI/BOOT
	cp extern/limine/BOOTX64.EFI $(BUILD_DIR)/iso/EFI/BOOT/
	cp extern/limine/BOOTIA32.EFI $(BUILD_DIR)/iso/EFI/BOOT/

	xorriso -as mkisofs \
		-b limine-bios-cd.bin \
		-no-emul-boot \
		-boot-load-size 4 \
		-boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part \
		--efi-boot-image \
		--protective-msdos-label \
		$(BUILD_DIR)/iso \
		-o $(ISO)

	extern/limine/limine bios-install $(ISO)

run: iso
	qemu-system-x86_64 \
		-cdrom $(ISO) \
		-m 128M \
		-serial stdio

debug: iso
	qemu-system-x86_64 \
		-cdrom $(ISO) \
		-s \
		-S \
		-m 128M

clean:
	cargo clean
	rm -rf $(BUILD_DIR)
