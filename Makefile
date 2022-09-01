

# make
rq:
	cargo run --quiet

# make b
b:
	@cargo make b

# make br
br:
	@cargo make br


# make r
r:
	@cargo make r

# make rr
rr:
	@cargo make rr


test_all:
	@cargo make t_all

test_priv_all:
	@cargo make t_priv_all

test_datetime:
	@cargo make t_datetime

test_stringlib:
	@cargo make t_stringlib

run_stringlib_alignment:
	@cargo make r_stringlib_alignment

# .SILENT: flow | rflow | run | br | b | r | rr

lint:
	cargo clippy


watch_check_tests_and_all:
	@cargo watch -w=src -w=tests -w=examples --shell='cargo check --features all && ./scripts/compiled.sh || ./scripts/compile_error.sh' -c


clippy-dirty:
	clippy-dirty -- --all-features -- -D clippy::all
cld: clippy-dirty

