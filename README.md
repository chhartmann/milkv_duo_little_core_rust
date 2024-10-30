Based on https://gitlab.com/dzamlo/milkv-duos-little-core-rust-firmware/
Playground for Rust with https://embassy.dev

The default image from https://github.com/milkv-duo/duo-buildroot-sdk/releases/download/Duo-V1.1.2/arduino-milkv-duo-sd-v1.1.2-2024-0801.img.zip has only 184K reserved for little core.
This project by default assumes, that 2M are reserved for the little core. For MilkV Duo with sd card, https://github.com/chhartmann/duo-buildroot-sdk/releases/download/rproc_wo_arduino_v0.1/milkv-duo-11593480108.zip has been prepared for this purpose.
The linker file memory.x has to be adapted, if the default image is used.

ssh-keygen -f "/home/milkv/.ssh/known_hosts" -R "192.168.42.1" has to be executed in the devcontainer, so that the firmware can be deployed automatically.

See also: https://community.milkv.io/t/jtag-jtag-debug-guide-for-duo/1138
