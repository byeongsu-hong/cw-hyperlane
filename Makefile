PWD:=$(shell pwd)
BASE:=$(shell basename "$(PWD)")

clean:
	@cargo clean
	@rm -rf ./artifacts

install: install-dev

install-dev: install-prod
	cargo install --force cargo-llvm-cov cw-optimizoor beaker

install-prod:
	cargo install --force cosmwasm-check
	rustup target add wasm32-unknown-unknown

schema:
	ls ./contracts | xargs -n 1 -t beaker wasm ts-gen

check:
	ls -d ./artifacts/*.wasm | xargs -I x cosmwasm-check x

optimize:
	docker run --rm -v "$(PWD)":/code \
		--mount type=volume,source="$(BASE)_cache",target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/optimizer:0.16.1

optimize-fast:
	cargo cw-optimizoor
	rename --force 's/(.*)-(.*)\.wasm/$$1\.wasm/d' artifacts/*

build: optimize-fast check
	cargo build
	cargo wasm

ci-build: optimize check
	zip -jr wasm_codes.zip artifacts
