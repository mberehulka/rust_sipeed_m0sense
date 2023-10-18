# Sipeed M0 Sense

This repo is a personal set of examples running on the [M0Sense](https://wiki.sipeed.com/hardware/en/maixzero/sense/maix_zero_sense.html) board

## ⚙ Installing
```
cargo install cargo-binutils
rustup component add llvm-tools
pip install bflb-mcu-tool
```

## 🚀 Running
Before running the command, the board should be in programming mode, set IO pin 28 to high before turning the board on, and keep until boot.

After your computer detect the new serial communication device and add a new COM port, run the code to upload the binary to board:

```
cargo objcopy --release --example blinky -- -O binary .bin/blinky.bin
bflb-mcu-tool --chipname bl702 --firmware .bin/blinky.bin
```

Or simply:

```
assets/run.bat blinky
```

## 🧾 Board specs

[BL702](https://en.bouffalolab.com/product/?type=detail&id=8) chip specs:
- single core, 32-bit, RISC-V CPU running at 144MHz with [FPU](https://en.wikipedia.org/wiki/Floating-point_unit).
- 132KB RAM
- 192KB ROM
- 512KB Flash
- Two 32-bit timer
- Eight DMA channels
- One SPI
- Two UART
- One I2C interface
- One I2S
- Five PWM
- One 12-bit ADC
- One 10-bit DAC
- 2.4Ghz BlueTooth V5.0
- 1Mbps and 2Mbps BLE
- USB 2.0 FS route to USB Type-C to burn firmware

Onboard featues:
- One analog microphone
- One RGB LED
- [QMI8658A](https://www.lcsc.com/product-detail/Attitude-Sensors_QST-QMI8658A_C3021082.html) six-axis [IMU](https://en.wikipedia.org/wiki/Inertial_measurement_unit)

[Datasheet](./assets/Datasheet.pdf)

## 🧐 Resources
- https://github.com/9names/bl702-hal
- https://wiki.sipeed.com/hardware/en/maixzero/sense/maix_zero_sense.html