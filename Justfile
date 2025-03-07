_default:
	@just --list --unsorted --list-heading '' --list-prefix '—— '
	
alias b := build

build:
	cargo build

fmt:
	cargo fmt -- --config "group_imports=StdExternalCrate"
	cd car-core && cargo fmt -- --config "group_imports=StdExternalCrate"
