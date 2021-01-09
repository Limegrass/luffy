build-musl:
	cargo build --target x86_64-unknown-linux-musl --release
docker-build-musl:
	docker build -f Dockerfile.alpine . -t luffy:latest
run-info:
	RUST_LOG=info cargo run -- -c ${CONFIG_PATH}
