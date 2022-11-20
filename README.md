# Embedded Car

## Installation

-   `cargo install probe-run`, installs `probe-run` to flash the firmware to the microcontroller.

-   `rustup target add thumbv7m-none-eabi`, adds the target to the rust toolchain if you don't have it already. (Although the `rust-toolchain.toml` should do it already)

-   `probe-run --list-probes`, lists all the connected probes, to check if your ST-Link is correctly recognized.

### VS Code

> Extracted from the `probe-rs` docs - [VSCode setup](https://probe.rs/docs/tools/vscode/)

-   Install the `probe-rs-debugger` extension in VS Code, by downloading the latest available `probe-rs-debugger-x.x.x.vsix` from the [probe-rs/vscode release page](https://github.com/probe-rs/vscode/releases)
    Then, install the extension with `code --install-extension probe-rs-debugger-x.x.x.vsix`
-   You also need to install the server component: `cargo install --git https://github.com/probe-rs/probe-rs --force --branch master probe-rs-debugger`

### Circuit (Fritzing)

The circuit design is made with the open source application `Fritzing` and stored in the file `CircuitDesign.fzz`.
You can download a packed executable on the website by donating $8 or build it from source with the [installation instructions](https://github.com/fritzing/fritzing-app/wiki/1.-Building-Fritzing) on the repository.

## Material

> Many of the documents linked here are available in the `hardware-specs` folder of this repository.

-   **Blue Pill** Board (`STM32F103C8T6`) - [STM32-BASE](https://stm32-base.org/boards/STM32F103C8T6-Blue-Pill) - [Pinout Diagram](https://github.com/siyouluo/STM32-Blue-Pill/blob/master/PDF/The-Generic-STM32F103-Pinout-Diagram.pdf)
-   **ST-Link** V2 (`STM32F101C8T6`) - [STM32-base](https://stm32-base.org/boards/Debugger-STM32F101C8T6-STLINKV2)
-   Motor Driver Module (`HW-095` containing a `L298N`) - [Components101](https://components101.com/modules/l293n-motor-driver-module) and [AllDataSheets](https://www.alldatasheet.fr/datasheet-pdf/pdf/22440/STMICROELECTRONICS/L298N.html)
-   Bluetooth module (`HC-05`) - [ElectroSchematics](https://www.electroschematics.com/wp-content/uploads/2013/07/HCSR04-datasheet-version-2.pdf)
-   Ultrasonic Sensor (`HC-SR04`) - [SparkFun](https://cdn.sparkfun.com/datasheets/Sensors/Proximity/HCSR04.pdf)

-   Large **cell** of `3600mA` (`LGDB-M36-1865`) - [SecondLifeStorage](https://secondlifestorage.com/index.php?threads/lg-lgdbm361865-cell-specifications.8329/)
-   A **battery charger** module for `18650` cells - [LetMeKnow](https://letmeknow.fr/fr/batteries/2541-module-d-alimentation-charge-micro-usb-18650.html)
-   A `18650` **cell holder** - [LetMeKnow](https://letmeknow.fr/fr/coupleurs/1581-support-pour-batterie-18650-avec-fils-652733546272.html)

-   Unused **Servo Motor** for now (`SG90`) - [DataSheetsPDF](https://datasheetspdf.com/pdf/791970/TowerPro/SG90/1)
-   **Breadboard** and MM/MF **jumper wires**

## Ressources

### Building a mental scheme

-   Various knowledge about electricity: The Engineering Mindset - [YouTube](https://www.youtube.com/c/Theengineeringmindset/channels)
-   Wiring a motor driver module: [YouTube](https://www.youtube.com/watch?v=bNOlimnWZJE) and [How To Mechatronics](https://howtomechatronics.com/tutorials/arduino/arduino-dc-motor-control-tutorial-l298n-pwm-h-bridge/) tutorials.
-   `PWM` and Timers - [YouTube](https://www.youtube.com/watch?v=AjN58ceQaF4) and [Nordic Semiconductor](https://infocenter.nordicsemi.com/index.jsp?topic=%2Fcom.nordic.infocenter.nrf52832.ps.v1.1%2Fpwm.html)
-   `UART` protocol - [AnalogDialogue](https://www.analog.com/en/analog-dialogue/articles/uart-a-hardware-communication-protocol.html)
