

info:
	$(info Targets)
	$(info -----------------------------------------------------------------------)
	$(info assets      | generate default theme packs and syntax)
	$(info - OTHER TARGETS -------------------------------------------------------)
	$(info themes      | generate default theme pack)
	$(info packs       | generate default syntax pack)
	$(info syntest     | run syntax test summary)

# make
rq:
	cargo run --quiet

# make br
br:
	cargo build --release

# make b
b:
	cargo build

# make r
r:
	cargo run

# make rr
rr:
	cargo run --release


t:
	cargo test -j 8 -- --show-output

test:
	cargo test -j 8 -- --show-output

test_private:
	cargo test -j 8 --lib -- --show-output

flow:
	@cargo run --example flow_control --quiet

rflow:
	@cargo run --example flow_control --release --quiet


spin:
	@cargo run --example spinners --quiet

rspin:
	@cargo run --example spinners --quiet --release


mac:
	@cargo run --example macros --quiet

rmac:
	@cargo run --example macros --quiet --release



# .SILENT: flow | rflow | run | br | b | r | rr

lint:
	cargo clippy
