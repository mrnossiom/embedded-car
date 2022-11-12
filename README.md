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

## Material

> Many of the documents linked here are available in the `hardware-specs` folder of this repository.

-   **Blue Pill** Board (`STM32F103C8T6`) - [STM32-BASE](https://stm32-base.org/boards/STM32F103C8T6-Blue-Pill) - [Pinout Diagram](https://github.com/siyouluo/STM32-Blue-Pill/blob/master/PDF/The-Generic-STM32F103-Pinout-Diagram.pdf)
-   **ST-Link** V2 (`STM32F101C8T6`) - [STM32-base](https://stm32-base.org/boards/Debugger-STM32F101C8T6-STLINKV2)
-   Motor Driver Module (`HW-095` containing a `L298N`) - [Components101](https://components101.com/modules/l293n-motor-driver-module) and [AllDataSheets](https://www.alldatasheet.fr/datasheet-pdf/pdf/22440/STMICROELECTRONICS/L298N.html)

-   Large **cell** of `3600mA` (`LGDB-M36-1865`) - [SecondLifeStorage](https://secondlifestorage.com/index.php?threads/lg-lgdbm361865-cell-specifications.8329/)
-   A **battery charger** module for `18650` cells - [LetMeKnow](https://letmeknow.fr/fr/batteries/2541-module-d-alimentation-charge-micro-usb-18650.html)
-   A `18650` **cell holder** - [LetMeKnow](https://letmeknow.fr/fr/coupleurs/1581-support-pour-batterie-18650-avec-fils-652733546272.html)

-   Unused **Servo Motor** for now (SG90) - [DataSheetsPDF](https://datasheetspdf.com/pdf/791970/TowerPro/SG90/1)
-   **Breadboard** and MM/MF **jumper wires**
