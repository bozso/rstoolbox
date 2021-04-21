TPL ?= rstoolbox template

.PHONY: test

test:
	$(TPL) --config="config.json" --output="test/out/main.html"

install:
	cargo build --release
	cp target/release/rstoolbox ${HOME}/packages/usr/bin
