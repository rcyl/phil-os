#!/bin/sh

MYID="$(id -u):$(id -g)"
MYID=${MYID} docker compose run app bash
