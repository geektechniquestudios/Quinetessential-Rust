# Quinetessential-Rust
A simple Rust quine for fun. 

```
fn main() {
    let quine = "fn main() { let quine = \"fn main() { let quine = \\\"fn main() { let quine = \\\\\\\"\\\\\\\"{}\\\\\\\"; print!(quine, quine=quine) }\\\"; print!(quine, quine=quine) }\n\"; print!(quine, quine=quine) }";
    print!("{}", quine);
}
```

Not much to see here; just appreciate how cool programming is. Enjoy your day.
