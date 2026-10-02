# build-01-start — the starting point for Build 1

This is **where Build 1 begins, not what it produces.** It is a working but
deliberately tiny 6502 emulator: a processor that understands five opcodes, a
64 KB memory bus, a memory-mapped output port, and four unit tests. Build 1
adds the instructions needed to print `Hello, world!`.

It exists because [Build 1 is published free][build1] on the website, and
without this you could read it but not run it. The scaffold is assembled over
Chapters 4 and 5 of *6502 Construction Set* — if you have those chapters,
build it yourself and ignore this repo. If you do not, start here.

[build1]: https://silicon-arcade.com/read/build-01/

## What you need

**Rust 1.85 or newer.** The crate declares edition 2024 and a `rust-version`
of 1.85, so an older toolchain is refused with a clear message rather than
failing somewhere confusing. If you have no Rust at all, install
[rustup](https://rustup.rs) and then:

```sh
rustup update stable
rustc --version        # expect 1.85.0 or higher
```

Nothing else. The crate has no dependencies — `[dependencies]` in
`Cargo.toml` is empty and stays that way for the whole of volume 1.

## Check it works before you change anything

```sh
git clone https://github.com/silicon-arcade/build-01-start
cd build-01-start
cargo test
```

Four tests should pass:

```
test tests_unit::lda_absolute_x_indexes_from_base ... ok
test tests_unit::ldx_immediate_loads_x_and_sets_flags ... ok
test tests_unit::ldx_immediate_negative_sets_n ... ok
test tests_unit::ldx_immediate_zero_sets_z ... ok
```

And the program runs:

```sh
cargo run
```

It prints a single `H` — eight bytes of 6502 assembled by hand in `main.rs`,
written to the output port at `$F001`, with a `JMP`-to-self as the halt
signal. That `H` is the whole of the emulator's output so far, and it is the
thing Build 1 turns into a string.

## What is here

| File | What it is |
|---|---|
| `src/bus.rs` | The seam: `memory_read` and `memory_write`, and nothing else. Finished — no later build changes it |
| `src/bus_flat.rs` | `FlatBus` — 64 KB where every address is ordinary RAM |
| `src/bus_console.rs` | `ConsoleBus` — the same RAM with `$F001` intercepted and captured |
| `src/flags.rs` | The eight status bits, each discriminant being its own bitmask |
| `src/cpu.rs` | Registers, `reset`, and a `step` that decodes five opcodes |
| `src/lib.rs` | Four modules, four re-exports |
| `src/main.rs` | The eight-byte program above |
| `src/tests_unit.rs` | Four unit tests |

Five opcodes are implemented: `$A9` LDA immediate, `$A2` LDX immediate,
`$BD` LDA absolute,X, `$8D` STA absolute, `$4C` JMP absolute. Anything else
panics on purpose, loudly, with the opcode and the address — an unimplemented
opcode means the emulator has lost track of where instructions begin, and
every byte after that is noise.

Four unit tests come with it, including the two that are easy to skip — load
`$00` and assert Z sets, load `$80` and assert N sets. A Z flag that never
sets is invisible right up until a branch depends on it, and in Build 1 one
does.

## What is deliberately missing

**The explanation.** This repo is the *output* of Chapters 4 and 5, not a
substitute for them. Why the bus is a two-method trait and not a struct, why
the flag update clears before it sets, why `panic!` is a design decision
rather than a placeholder, why PC stays parked on the opcode byte — that is
the half you cannot get from reading the source, and it is in the book.

## Licence

The code is **Apache-2.0**; see [LICENSE](LICENSE) and [NOTICE](NOTICE).
Apache was chosen over MIT for the NOTICE mechanism: anyone redistributing
this code has to carry the attribution in `NOTICE`, which a bare MIT
copyright line does not require.

The book's text is a separate work and is not licensed by either file.
