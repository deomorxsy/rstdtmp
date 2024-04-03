SHELL=/bin/bash

pdm=$(shell source ./scripts/pdm.sh; podman_compose)

tests_run:

start:

stop:

build:

podman:
	@echo "Running compose as a Podman Service from systemd's unit file..."
	@$(call pdm)
