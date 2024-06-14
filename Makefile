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
