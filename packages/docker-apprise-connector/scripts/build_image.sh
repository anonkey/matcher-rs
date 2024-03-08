#!/bin/sh
## name: build_images.sh

docker build --progress=plain -t "ghcr.io/anonkey/docker-events-apprise:nightly" -f ./Dockerfile ..
