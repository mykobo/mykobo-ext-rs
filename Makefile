release:
	@cargo release patch --execute

test:
	@cargo nextest run


prepare:
	@cargo fmt --all
	@cargo clippy --fix
	@cargo check
	@cargo nextest run
