# Create
To create a solution file:
```shell
cargo run -- create -y <year> -d <day> --t <input-type>
```

where `input-type` is one of:

| `input-type` | Rust code |
| --- | --- |
| str-slice | `&str` |
| slice-of-str-slice | `&[&str]` |
| vec_of_str_slice | `Vec<&str>` |
| u8_slice | `&[u8]` |
| slice_of_u8_slice | `&[&[u8]]` |
| vec_of_u8_slice | `Vec<&[u8]>` |

# Test
To test either a solution or another crate in the tree, run the command:

```shell
cargo test --package <package> -- --show-output <test-module>
```

where:
- `package` is a valid Rust package name (one of the named members in the root Cargo.toml).
- `test-module` is either a test name, or a path to a test module.

# Run
To run a solution file:
```shell
cargo run -- run -y <year> -d <day> [-a] [-b] [-v alt-version]
```

where `alt-version` is a named alternate version of a solution
that is defined in a solution file:

```rust
#[crate::aoc(year = 2015, day = 1, part = "A")]
fn day01a(input: &str) -> usize { .. }

#[crate::aoc(year = 2015, day = 1, part = "A", version = "alt")]
fn day01a_alternate(input: &str) -> usize { .. }
```

By default, `day01a` would be run, but `day01a_alternate` can be run with the command:

```shell
cargo run -- run -y 2015 -d 1 -v alt -a
```
