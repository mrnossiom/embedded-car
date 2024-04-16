# Embedded Car (WIP)

## Installation

-   `cargo install probe-rs`, installs `probe-rs` to flash the firmware to the microcontroller.
-   `rustup target add thumbv7m-none-eabi`, adds the target to the rust toolchain if you don't have it already. (Although the `rust-toolchain.toml` should do it already)

There is a [`probe-rs` extension](https://probe.rs/docs/tools/debugger/#building-and-testing-the-debug-extension-in-vs-code) for VS Code

### Circuit (Fritzing)

The circuit design is made with the open source application `Fritzing` and stored in the file `CircuitDesign.fzz`.
You can download a packed executable on the website by donating $8 or build it from source with the [installation instructions](https://github.com/fritzing/fritzing-app/wiki/1.-Building-Fritzing) on the repository.

## Material

> Many of the documents linked here are available in the `hardware-specs` folder of this repository.

-   **Blue Pill** Board (`STM32F103C8T6`) - [STM32-BASE](https://stm32-base.org/boards/STM32F103C8T6-Blue-Pill) - [Pinout Diagram](https://github.com/siyouluo/STM32-Blue-Pill/blob/master/PDF/The-Generic-STM32F103-Pinout-Diagram.pdf)
-   **ST-Link** V2 (`STM32F101C8T6`) - [STM32-base](https://stm32-base.org/boards/Debugger-STM32F101C8T6-STLINKV2)
-   Motor Driver Module (`HW-095` containing a `L298N`) - [Components101](https://components101.com/modules/l293n-motor-driver-module) and [AllDataSheets](https://www.alldatasheet.fr/datasheet-pdf/pdf/22440/STMICROELECTRONICS/L298N.html)
-   **Bluetooth v4 (BLE)** module (`HM-10` containing a `TI CC2540`) - [HackSpark](https://hackspark.fr/fr/outils-de-dev/1467-hm-10-serial-port-ble-cc2540-module-with-logic-level-translator-master-slave-.html) [CornellEngineering](https://people.ece.cornell.edu/land/courses/ece4760/PIC32/uart/HM10/DSD%20TECH%20HM-10%20datasheet.pdfs)
-   **Ultrasonic Sensor** (`HC-SR04`) - [SparkFun](https://cdn.sparkfun.com/datasheets/Sensors/Proximity/HCSR04.pdf)
-   **Servo Motor** (`SG90`) - [DataSheetsPDF](https://datasheetspdf.com/pdf/791970/TowerPro/SG90/1)

-   Large **cell** of `3600mA` (`LGDB-M36-1865`) - [SecondLifeStorage](https://secondlifestorage.com/index.php?threads/lg-lgdbm361865-cell-specifications.8329/)
-   A **battery charger** module for `18650` cells - [LetMeKnow](https://letmeknow.fr/fr/batteries/2541-module-d-alimentation-charge-micro-usb-18650.html)
-   A `18650` **cell holder** - [LetMeKnow](https://letmeknow.fr/fr/coupleurs/1581-support-pour-batterie-18650-avec-fils-652733546272.html)

-   **Breadboard** and MM/MF **jumper wires**
-   A large stock of resistors of every value - [HackSpark](https://hackspark.fr/en/electronics/1470-1-4w-metal-resistor-kit-30-values-600pieces.html)

## Ressources

### Building a mental scheme

-   Various knowledge about electricity: The Engineering Mindset - [YouTube](https://www.youtube.com/c/Theengineeringmindset/channels)
-   Wiring a motor driver module: [YouTube](https://www.youtube.com/watch?v=bNOlimnWZJE) and [How To Mechatronics](https://howtomechatronics.com/tutorials/arduino/arduino-dc-motor-control-tutorial-l298n-pwm-h-bridge/) tutorials.
-   `PWM` and Timers - [YouTube](https://www.youtube.com/watch?v=AjN58ceQaF4) and [Nordic Semiconductor](https://infocenter.nordicsemi.com/index.jsp?topic=%2Fcom.nordic.infocenter.nrf52832.ps.v1.1%2Fpwm.html)
-   `UART` protocol - [AnalogDialogue](https://www.analog.com/en/analog-dialogue/articles/uart-a-hardware-communication-protocol.html)
-   `Bluetooth Low Energy` - [Official Website](https://www.bluetooth.com/blog/a-developers-guide-to-bluetooth/)

## Structure

Project contains multiples crates to control or program behaviour.

-   `car-core`: contains microcontroller logic
-   `car-controller`: provides a `CLI` and a user interface to interact via `Bluetooth` with the car
-   `car-transport`: contains message logic between the _car_ and the _controller_
