.PHONY = all rebuild build flash clean examples prerequisites

# e.g. examples/foo.rs
# EXAMPLES_RS = $(wildcard examples/*.rs)
# e.g. examples/foo
# EXAMPLES = $(EXAMPLES_RS:.rs=)
TARGET_ROOT = target/thumbv7em-none-eabihf
DEBUG_ROOT = $(TARGET_ROOT)/debug
RELEASE_ROOT = $(TARGET_ROOT)/release
# e.g. target/thumbv7em-none-eabihf/debug/examples/foo
# DEBUG_ELF = $(EXAMPLES:%=$(DEBUG_ROOT)/%)
DEBUG_ELF = $(DEBUG_ROOT)/tm4c123exploration
# e.g. target/thumbv7em-none-eabihf/release/examples/foo
# RELEASE_ELF = $(EXAMPLES:%=$(RELEASE_ROOT)/%)
RELEASE_ELF = $(RELEASE_ROOT)/tm4c123exploration
# e.g. target/thumbv7em-none-eabihf/debug/examples/foo.bin
DEBUG_BIN = $(DEBUG_ELF:%=%.bin)
# DEBUG_BIN = $(DEBUG_ROOT:%=%.bin)
# e.g. target/thumbv7em-none-eabihf/release/examples/foo.bin
RELEASE_BIN = $(RELEASE_ELF:%=%.bin)
# RELEASE_BIN = $(RELEASE_ROOT:%=%.bin)
# e.g. stable
TOOLCHAIN = stable

all: build

rebuild: clean build

build: $(DEBUG_BIN) $(RELEASE_BIN)

flash: $(RELEASE_BIN)
	$${HOME}/projects/lm4tools/lm4flash/lm4flash $(RELEASE_BIN)

clean:
	cargo clean

$(DEBUG_BIN) $(RELEASE_BIN): %.bin: %
	arm-none-eabi-size -x $<
	arm-none-eabi-objcopy -O binary $< $@

$(TARGET_ROOT)/debug/%: FORCE
	cargo +$(TOOLCHAIN) build

$(TARGET_ROOT)/release/%: FORCE
	cargo +$(TOOLCHAIN) build --release

prerequisites:
	rustup install $(TOOLCHAIN)
	rustup component add rust-src
	rustup target add thumbv7em-none-eabihf

FORCE: