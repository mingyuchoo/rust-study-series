dev: clean build run

prod: clean build pack

clean:
	cargo clean
	docker builder prune -f

watch:
	cargo watch -x run

build:
	cargo build --release

run:
	cd ./target/release && ./demo-rocket

pack:
	docker build --tag demo-rocket:latest .

launch:
	docker run                  \
	--name demo-rocket         \
	--network $(DOCKER_NETWORK) \
	--publish 8000:8000         \
	--detach                    \
	demo-rocket
