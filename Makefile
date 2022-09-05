.PHONY: help
help:
	@echo "Usage: make [clean|build]"


.PHONY: clean
clean:
	@echo "Cleaning..."
	@rm -rf target

.PHONY: build
build:
	@echo "\n\n build darwin"
	cargo build --release --target x86_64-apple-darwin
	@echo "\n\n build windows"
# brew install mingw-w64
# rustup target add x86_64-pc-windows-gnu
	cargo build --release --target x86_64-pc-windows-gnu