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
	cd ./target/release && ./hello-rocket

pack:
	docker build --tag hello-rocket:latest .

launch:
	docker run                  \
	--name hello-rocket         \
	--network $(DOCKER_NETWORK) \
	--publish 8000:8000         \
	--detach                    \
	hello-rocket
