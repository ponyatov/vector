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
	cargo run fmt $? && touch $@

# install
.PHONY: install update gz ref
install: gz ref
	$(MAKE) update
update:
	sudo apt update
	sudo apt install -uy `cat apt.txt`
gz:
ref:
