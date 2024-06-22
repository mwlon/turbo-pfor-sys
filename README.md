# TurboPFor Rust Bindings

See
[TurboPFor](https://github.com/powturbo/TurboPFor-Integer-Compression)
for information about this lossless codec for numerical data.
Note that I am not the original author; this just binds to the original.

These bindings are published to crates.io and build from source; the ones in
the TurboPFor main repo are local and rely on a pre-built library in the
default search path.

These bindings currently only support Unix.

## Also Relevant

* [Pcodec](https://github.com/mwlon/pcodec/):
  another lossless codec for numerical data; contains a CLI with
  with benchmarks against a variety of other codecs.
