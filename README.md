# dpc

[![ci](https://github.com/helio-frota/dpc/actions/workflows/ci.yaml/badge.svg)](https://github.com/helio-frota/dpc/actions/workflows/ci.yaml)

> 'dpc' is a shorter name for 'duplicrabs' (which is a made-up word).

Basic and not 100% accurate for finding duplicate code blocks in Rust projects.

clone and run

```shell
cargo run --release -- -d /home/foobar/Desktop/guac-rs
```

or

```shell
cargo run --release -- -d /home/foobar/Desktop/guac-rs > out.md
```

Increase the threshold to 1 with `-t 1` to print only exactly the same code blocks

```shell
cargo run --release -- -d /home/foobar/Desktop/guac-rs -t 1
```

Ignore scanning the code on tests directory

```shell
cargo run --release -- -d /home/heliofrota/Desktop/tc/guac-rs/ -i tests
```

[Example](./out.md)
