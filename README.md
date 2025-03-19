# cute
A little simple getopt tools for rust

# USAGE

`Cute` hold all the `Opt`s, provide the inteface parse the command line arguments.

```rust
use cute::prelude::*;
use std::fmt::Debug;
use std::hash::Hash;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // define a enum state for your options
    #[derive(Debug, Clone, Eq, PartialEq, Default, Hash)]
    enum ParseState {
        Boolean,
        String,
        #[default]
        Default,
    }

    let mut cute = Cute::new();

    // add options
    cute.add(switch("--boolean", ParseState::Boolean));
    cute.add(option("--string", ParseState::String));

    // parse the given strings

    cute.parse(["--boolean", "--string=32"].iter())?;

    // using ctx result
    assert!(cute.value::<bool>(ParseState::Boolean)?);
    assert_eq!(cute.value::<&str>(ParseState::String)?, "32");
    Ok(())
}
```

# Documents

see [`Documents`](https://araraloren.github.io/cuteopt/)

# LICENSE
MIT License
