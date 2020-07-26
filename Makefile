PROJECT_NAME = 'klusterview'

# Version
VERSION = `date +%y.%m`

# If unable to grab the version, default to N/A
ifndef VERSION
    VERSION = "n/a"
endif

#
# Makefile options
#


# State the "phony" targets
.PHONY: all debug build run clean install uninstall

all: build

debug:
	@echo 'Building debug binary of ${PROJECT_NAME}...'
	@cargo build

build:
	@echo 'Building release for ${PROJECT_NAME}...'
	@cargo build --release

run:
	@RUST_LOG=trace cargo run

clean:
	@echo 'Cleaning...'
	@cargo clean

install: build
	@echo Installing executable file to /usr/local/bin/${PROJECT_NAME}
	@sudo cp ./target/release/${PROJECT_NAME} /usr/local/bin/${PROJECT_NAME}
	@sudo cp ./lib/systemd/system/${PROJECT_NAME}.service /lib/systemd/system/${PROJECT_NAME}.service

uninstall: clean
	@echo Removing executable file from /usr/local/bin/${PROJECT_NAME}
	@sudo rm -f /usr/local/bin/${PROJECT_NAME}
	@sudo rm -f /lib/systemd/system/${PROJECT_NAME}.service
