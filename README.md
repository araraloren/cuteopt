# cuteopt
A little simple getopt tools for rust

# USAGE

`Ctx` hold all the `OptKeeper`s, provide the inteface parse the command line arguments.

```rust
    use cute::prelude::*;
    use std::fmt::Debug;
    use std::hash::Hash;

    // define a enum state for your options
    #[derive(Debug, Clone, Eq, PartialEq, Default, Hash)]
    enum ParseState {
        Boolean,
        String,
        #[default]
        Default,
    }

    let mut ctx = Ctx::new();

    // add options
    ctx.add(switch("--boolean", ParseState::Boolean));
    ctx.add(option("--string", ParseState::String));

    // parse the given strings

    ctx.parse(["--boolean", "--string=32"].iter())?;

    // using ctx result
    assert!(ctx.value::<bool>(ParseState::Boolean)?);
    assert_eq!(ctx.value::<&str>(ParseState::String)?, "32");
```

# Documents

see [`Documents`](https://araraloren.github.io/cuteopt/)

# LICENSE
MIT License
