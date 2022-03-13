# babibapp

Bap bap bap, babibapp bap bap.

`babibapp` is built in Rust using:
- [actix-web](https://actix.rs)
- [diesel](https://diesel.rs) with [postgresql](https://www.postgresql.org)

## user authentication
User authentication is done using [JWTs](https://jwt.io).
Passwords are only stored on the server as [bcrypt](https://en.wikipedia.org/wiki/Bcrypt) hashes.
However, `babibapp`'s security could be improved. There's Luft nach oben.

## musl cross compilation
To build the static binaries you may want to compile to [`musl libc`](https://www.musl-libc.org).
To make this work you can compile the server and the CLI in an Alpine Docker environment.
Use this command:

```sh
alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
rust-musl-builder cargo build --release
```

See [emk/rust-musl-builder](https://github.com/emk/rust-musl-builder).

## what's implemented

- [server](server)
	- features:
		- students
		- teachers
		- comments
			- with votes
	- JWT user authentication
- [APIs](apis)
	- [Rust API](apis/rust-api)
- [clients](clients)
	- [Command line interface](clients/cli)
