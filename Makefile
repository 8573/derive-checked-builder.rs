
build:
	@nix-shell --run 'cargo build'

check:
	@nix-shell --run 'cargo check'

test:
	@nix-shell --run 'cargo test'

run:
	@nix-shell --run 'cargo run'

docs:
	@nix-shell --run 'cargo doc --no-deps --all-features'

view-docs:
	@nix-shell --run '\
	   BROWSER=chromium-browser \
	   cargo doc --no-deps --all-features --open \
	 '

clean:
	@nix-shell --run 'cargo clean'

fmt:
	@nix-shell --run '\
	   rustfmt src/lib.rs; \
	   cargo-fmt; \
	 '

shell:
	@nix-shell

expanded-test-1.rs: tests/eddyb-1.rs
	@nix-shell --run '\
	   cargo rustc --test eddyb-1 -- \
	     -Z unstable-options --pretty expanded \
	     | rustfmt \
	     > expanded-test-1.rs \
	 '
