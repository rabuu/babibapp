# babibapp

Bap bap bap, babibapp bap bap.

`babibapp` is built in Rust using:
- [actix-web](https://actix.rs)
- [diesel](https://diesel.rs) with [postgresql](https://www.postgresql.org)

## user authentication
User authentication is done using [JWTs](https://jwt.io).
Passwords are only stored on the server as [bcrypt](https://en.wikipedia.org/wiki/Bcrypt) hashes.
However, `babibapp`'s security could be improved. There's Luft nach oben.

## TODO

- [server](server)
	- features:
		- students (basic stuff mostly done)
		- teachers
		- comments
		- quotes
	- user authentication (mostly done)
- APIs
	- Rust API
	- Maybe JavaScript/TypeScipt API
- clients
	- Web UI
	- Command line interface
