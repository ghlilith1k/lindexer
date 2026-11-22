install:
	@echo "lindexer v0.0.1-alpha, (C) 2026 ghlilith1k"
	@echo "see https://github.com/ghlilith1k/lindexer for details"
	@echo "running on:"
	@grep "PRETTY_NAME" /etc/os-release | cut -d'=' -f2
	@echo "compiling and installing lindexer v0.0.1-alpha"
	@echo "please wait..."
	@cargo build --release --quiet
	@sudo install -m 755 target/release/lindexer /usr/local/bin/lindexer
	@echo "lindexer installed at /usr/local/bin/lindexer"