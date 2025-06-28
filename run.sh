#!/bin/sh

MYID="$(id -u):$(id -g)"
docker compose run app bash
