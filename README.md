# cuteopt
A little simple getopt tools for rust

# USAGE

[`Ctx`] hold all the [`OptKeeper`]s, provide the inteface parse the command line arguments.

```no_run
use cuteopt::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
enum ParseState {
    PSBoolean,
    PSString,
    PSDefault,
}

impl Default for ParseState {
    fn default() -> Self {
        Self::PSDefault
    }
}

let mut ctx = Ctx::new();

ctx.add_bool("--boolean", ParseState::PSBoolean);
ctx.add_str("--string", ParseState::PSString);

ctx.parse(&mut std::env::args().skip(1));

// using ctx result
// dbg!(ctx.get_value_as_bool(ParseState::PSBoolean));
```

# LICENSE
MIT License
