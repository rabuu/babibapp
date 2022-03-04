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
		- students (X)
		- teachers (X)
		- comments (X)
			- plus votes (X)
		- (quotes)
		- (rankings)
	- user authentication (X)
- APIs
	- Rust API (X)
	- (JavaScript/TypeScript API)
- clients
	- Terminal UI
	- (Web UI)
