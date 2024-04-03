# Teixo
Personal website template with blog and other sections in Rust to learn webdev concepts. Spiritual child of my [previous experiences](https://github.com/deomorxsy/deomorxsy.github.io/) with Static Site Generators such as Jekyll and Hugo, also sligthly based on the infrastructure design by security-union's [yew-actix template](https://github.com/security-union/yew-actix-template/).

Featuring
- Actix as front-end
- Yew as back-end
- WASM/WebAssembly as virtualization API madness
- Podman as compose

## deployment

### Podman Service and $DOCKER_HOST

[compose](https://docs.docker.com/compose/) concentrates in orchestrating multiple containers in a single host. To do this with k8s, you would need kind, k3s (does not use virtualization) or similar. It was made to be compatible with other OCI runtimes, such as Podman, which was one of the first to enable rootless containers, and can be setup with compose using the Podman Service's systemd unit file for unix sockets.

The orchestration tool docker-compose supports Podman Service through the DOCKER_HOST environment variable. This makes it possible to run containers with podman but with the benefit of rootless.

[Source](./scripts/pdm.sh) the script and run it to run compose with podman:
```sh
; source ./scripts/pdm.sh
```
