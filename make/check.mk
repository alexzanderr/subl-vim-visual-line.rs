


check-rust-app:
	cargo watch \
		--clear \
		--delay 1.2 \
		--why \
		--watch-when-idle \
		--watch="rust-app/src/" \
		--watch="rust-app/Cargo.toml" \
		--shell="cargo check --color=auto --manifest-path rust-app/Cargo.toml"
