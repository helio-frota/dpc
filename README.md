# dpc

[![ci](https://github.com/helio-frota/dpc/actions/workflows/ci.yaml/badge.svg)](https://github.com/helio-frota/dpc/actions/workflows/ci.yaml)

> 'dpc' is a shorter name for 'duplicrabs' (which is a made-up word).

Basic and not 100% accurate for finding duplicate code blocks in Rust projects.

clone and run

```shell
cargo run --release /home/foobar/Desktop/guac-rs
or
cargo run --release /home/foobar/Desktop/guac-rs > out.md

pass whatever 3rd argument to show only exactly the same information on report

cargo run --release /home/foobar/Desktop/guac-rs abcde > out.md
```

[Example](./out.md)