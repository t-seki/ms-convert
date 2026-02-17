# ms-convert

Convert human-readable time strings to milliseconds and vice versa. A Rust port of [vercel/ms](https://github.com/vercel/ms).

Zero dependencies.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
ms-convert = "0.1"
```

### Parse a time string to milliseconds

```rust
use ms_convert::parse;

assert_eq!(parse("2d").unwrap(), 172_800_000.0);
assert_eq!(parse("1.5h").unwrap(), 5_400_000.0);
assert_eq!(parse("1 hour").unwrap(), 3_600_000.0);
```

### Format milliseconds to a human-readable string

```rust
use ms_convert::format;

assert_eq!(format(60_000.0, false), "1m");
assert_eq!(format(60_000.0, true), "1 minute");
assert_eq!(format(172_800_000.0, true), "2 days");
```

## Supported Units

| Short | Long (singular / plural) |
|-------|--------------------------|
| `ms`  | `millisecond` / `milliseconds` |
| `s`   | `sec` / `second` / `seconds` |
| `m`   | `min` / `minute` / `minutes` |
| `h`   | `hr` / `hour` / `hours` |
| `d`   | `day` / `days` |
| `w`   | `week` / `weeks` |
| `mo`  | `month` / `months` |
| `y`   | `year` / `years` |

## License

MIT
