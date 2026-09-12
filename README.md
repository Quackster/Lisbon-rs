# Lisbon — Rust port

Rust port of the Lisbon Habbo Hotel v26 server. The workspace contains two
crates:

| Crate | Binary | What it runs |
|---|---|---|
| `lisbon-server` | `lisbon-server` | The game server (game/MUS/RCON listeners, login, rooms, games, web-independent managers) |
| `lisbon-web` | `lisbon-web` | The web backend (HavanaWeb — site, account, community, news; depends on `lisbon-server`) |

## Requirements

- Rust (stable, edition 2021 toolchain)
- A MySQL/MariaDB server with the Lisbon database (schema + data) loaded —
  both binaries refuse to start without a working DB connection
- `RUST_LOG` is honoured (default level: `info`)

## Compile

From this directory:

```bash
# check only (fast)
cargo check --workspace

# debug build of both binaries
cargo build --workspace

# release build
cargo build --release --workspace
```

Binaries land in `target/debug/` (or `target/release/`):
`target/release/lisbon-server`, `target/release/lisbon-web`.

## Configuration

Both binaries read an INI config file. Path resolution (first hit wins):

1. `lisbon-server`: env `LISBON_SERVER_CONFIG` or `LISBON_CONFIG`
2. `lisbon-web`: env `LISBON_WEB_CONFIG` or `LISBON_CONFIG`
3. `config/server.ini` / `config/webserver-config.ini` (if `config/` exists)
4. `server.ini` / `webserver-config.ini` in the current directory

Game settings (`GameConfiguration`) are **not** in the INI: built-in defaults
are seeded into the DB `settings` table on first run and read back from it
afterwards. An optional `config/log4j.properties` (or `log4j.properties` in
the working directory) is read at startup for the log level/output; when
absent, built-in defaults apply.

### `server.ini` (game server)

```ini
[Server]
server.bind=0.0.0.0
server.port=30000

[Rcon]
rcon.bind=127.0.0.1
rcon.port=12309

[Mus]
mus.bind=0.0.0.0
mus.port=12322

[Database]
mysql.hostname=127.0.0.1
mysql.port=3306
mysql.username=lisbon
mysql.password=
mysql.database=lisbon
```

### `webserver-config.ini` (web backend)

```ini
[Site]
site.directory=./tools/www

[Global]
bind.ip=127.0.0.1
bind.port=8080

[Rcon]
rcon.ip=127.0.0.1
rcon.port=12309

[Database]
mysql.hostname=127.0.0.1
mysql.port=3306
mysql.username=lisbon
mysql.password=
mysql.database=lisbon

[Template]
template.directory=./tools/www-tpl
template.name=default-en
```

## Run

Each binary is standalone — run them as separate processes (e.g. two
terminals, or your process supervisor).

```bash
# game server
LISBON_SERVER_CONFIG=./config/server.ini ./target/release/lisbon-server

# web backend
LISBON_WEB_CONFIG=./config/webserver-config.ini ./target/release/lisbon-web
```

`lisbon-server` bootstraps: config → `Storage` (MySQL pool) → ~22 managers →
game / MUS / RCON accept loops; Ctrl-C disposes cleanly.
`lisbon-web` bootstraps: config → `Storage` → settings/wordfilter/sticker/
item/news managers → 1 s watchdog thread → route registration → HTTP server
on `bind.port`.

Expected startup log (game server):

```
Lisbon - Habbo Hotel V26 Emulation
... Setting up game ...
```

If the DB connection fails the binary logs the error and exits.

## Verify

```bash
cargo check --workspace   # 0 errors
```

The port is complete: `grep -rn "TODO(port)\|todo!" --include=*.rs` over
`lisbon-server/src` + `lisbon-web/src` returns no matches; remaining
behavioural deviations from the Java original are labelled
`// Port note:` in the source. See `../PORT_PROGRESS.md` for the full
port status and the per-marker resolution log.
