# Talkclassical Top List

Site with top lists of classical music works.

All content is a property of [Talk Classical](https://talkclassical.com) form members.

Exercise in doing a website with Rust.

## Run in dev environment natively

Have [Rust](https://www.rust-lang.org/tools/install) installed.

Have [Sass](https://sass-lang.com/install) installed as a command line utility.

Compile CSS from SCSS:

```bash
./build-css.sh
```

Or use another preferred way to compile `styles/index.scss` to `static/stiles.css`.

Compile and run Rust project:

```bash
cargo run
```

Use `http://localhost:8088` to access the site.

## Run in development using Docker

Have Docker installed.

```bash
docker-compose up -d
```

Use `http://localhost:8088` to access the site.

## Deploy

Not yet implemented. Will use Docker.