fn main() {
    let quine = "fn main() { let quine = \"fn main() { let quine = \\\"fn main() { let quine = \\\\\\\"\\\\\\\"{}\\\\\\\"; print!(quine, quine=quine) }\\\"; print!(quine, quine=quine) }\n\"; print!(quine, quine=quine) }";
    print!("{}", quine);
}

