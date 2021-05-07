TPL ?= rstoolbox template

.PHONY: test

test: install
	$(TPL) --config="config.json"

install:
	cargo build --release
	cp target/release/rstoolbox ${HOME}/packages/usr/bin
