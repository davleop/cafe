# Utility Makefile
#

all: build

build: build-server build-client

build-server: server.Dockerfile
	docker build -t server-test -f server.Dockerfile .

build-client: client.Dockerfile
	docker build -t client-test -f client.Dockerfile .

run: build
	docker run --rm -dit -p 42069:42069 server-test
	@echo Sleeping for 10 seconds...
	sleep 10
	docker run --rm -it client-test

server:
	docker run --rm -it -p 42069:42069 server-test

client:
	docker run --rm -it client-test

stop:
	docker stop $(shell docker ps -q)

clean: stop

.PHONY: all clean
