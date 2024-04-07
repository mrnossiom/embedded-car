_default:
	@just --list --unsorted --list-heading '' --list-prefix '—— '

# Build every crate
build: build-transport build-controller build-core

# Build `car-controller`
build-controller:
	cargo build --package car-controller

# Build `car-transport`
build-transport:
	cargo build --package car-transport
		
# Build `car-core`
build-core:
	cargo build --release --package car-core --target thumbv7m-none-eabi

# Run `car-core` with probe-run
run-car: build-core
	probe-run --chip STM32F103C8 target/thumbv7m-none-eabi/release/car-core
