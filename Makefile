# var
MODULE = $(notdir $(CURDIR))

# version
JQUERY_VER = 3.7.1

# dir
CWD = $(CURDIR)

# tool
CURL = curl -L -o

# package
JQUERY_URL = https://code.jquery.com

# src
R += $(wildcard src/*.rs)

# all
.PHONY: all
all:
	cargo run

# format
.PHONY: format
format: tmp/format_rs
tmp/format_rs: $(R)
	cargo fmt && touch $@

# install
.PHONY: install update gz ref
install: gz ref
	$(MAKE) update
update:
	sudo apt update
	sudo apt install -uy `cat apt.txt`
gz: cdn
ref:

.PHONY: cdn
cdn: \
	static/cdn/jquery.js
static/cdn/jquery.js:
	$(CURL) $@ $(JQUERY_URL)/jquery-$(JQUERY_VER).slim.min.js
