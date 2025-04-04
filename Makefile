NAME    := pingy
AUTHOR  := trinhminhtriet
DATE    := $(shell date +%FT%T%Z)
GIT     := $(shell [ -d .git ] && git rev-parse --short HEAD)
VERSION := $(shell git describe --tags)

default: build

build:
	cargo build --release
	ln -sf $(PWD)/target/release/$(NAME) /usr/local/bin/$(NAME)
	which $(NAME)
	$(NAME) --version

release:
	./scripts/release-version.sh

clean:
	$(RM) -r target

.PHONY: default test build release clean
