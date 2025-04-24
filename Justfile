_default:
	@just --list --unsorted --list-heading '' --list-prefix '—— '
	
run-cli *ARGS:
	cargo run --bin cli -- {{ARGS}}

run-bt-config *ARGS:
	cargo run --bin bt-config -- {{ARGS}}

alias b := build

build *ARGS:
	cargo build {{ARGS}}

fmt:
	cargo fmt -- --config "group_imports=StdExternalCrate"
	cd car-core && cargo fmt -- --config "group_imports=StdExternalCrate"
