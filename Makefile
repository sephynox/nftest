.PHONY: all build run_anvil test cleanup

all: build run_anvil test cleanup

build:
	@echo "Building with Forge..."
	@forge --version
	@forge build --sizes
	@echo "Building with Cargo..."
	@cargo build

run_anvil:
	@echo "Running Anvil..."
	@anvil & echo $$! > anvil.pid

test:
	@echo "Running tests..."
	@RUST_TEST_THREADS=1 DATABASE_URL=my_db cargo test
	@rm -rf my_db/

cleanup:
	@echo "Cleaning up..."
	@kill `cat anvil.pid` && rm anvil.pid