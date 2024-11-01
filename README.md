# Rust for MilkV Duo little core
Based on https://gitlab.com/dzamlo/milkv-duos-little-core-rust-firmware.

This project can be used as starting poing to play with Rust and https://embassy.dev

This project by default assumes, that 2M are reserved for the little core. For MilkV Duo with sd card https://github.com/chhartmann/duo-buildroot-sdk/releases/download/rproc_wo_arduino_v0.2/milkv-duo-11627979022.zip has been prepared for this purpose. Note: This image also includes the patch for the firmware to enable jtag debugging with GP15 (for more see link for debugging below).

# Build
The devcontainer includes everything to start programming and compiling.

# Run on target
The shell script run_on_target.sh is used by the vscode task "Run on target" as well as in "cargo run".
The script copies the firmware to the board and starts it via remoteproc interface.
Make sure a connection to the board can be established with "ssh root@192.168.42.1" before.

# Debug on target
The launch task in vscode for debugging is configured to automatically download the firmware via gdb and start debugging.
Debugging with jtag debugger is described in https://community.milkv.io/t/jtag-jtag-debug-guide-for-duo/1138. 
The debug server is already installed in the devcontainer.
Make sure the debug server can connect to the debugger with "DebugServerConsole" first.
Note: The MilkV Duo board has to be powered up before connecting the debugger to the PC (at least for the debugger based on STM32 board).
