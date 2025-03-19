#![doc = include_str!("../README.md")]
pub mod err;
pub mod opt;
pub mod val;

pub mod prelude {
    pub use crate::opt::*;
    pub use crate::val::*;
    pub use crate::Cute;
}

pub use err::Error;

use std::collections::HashMap;

use opt::{State, StateOpt};
use val::ValueParser;

#[derive(Default)]
/// The main parser struct that holds options and their values.
///
/// # Type Parameters
/// * `S` - The state type that implements the `State` trait
pub struct Cute<S: State> {
    opts: Vec<Box<dyn StateOpt<S = S>>>,
    values: HashMap<S, Vec<String>>,
}

impl<S: State> Cute<S> {
    /// Creates a new Cute instance with empty options and values.
    pub fn new() -> Self {
        Self {
            opts: vec![],
            values: HashMap::default(),
        }
    }

    /// Adds a new option to the parser.
    ///
    /// # Arguments
    /// * `arg` - The option to add, must implement `StateOpt`
    pub fn add(&mut self, arg: impl StateOpt<S = S> + 'static) -> &mut Self {
        self.opts.push(Box::new(arg));
        self
    }

    /// Gets an option by its state.
    ///
    /// # Arguments
    /// * `s` - The state to search for
    ///
    /// # Returns
    /// Option containing a reference to the option if found
    pub fn get(&self, s: S) -> Option<&dyn StateOpt<S = S>> {
        self.opts.iter().find(|v| v.state() == &s).map(|v| &**v)
    }

    pub fn has(&self, s: S) -> bool {
        self.opts.iter().any(|v| v.state() == &s)
    }

    /// Gets a parsed value for a given state.
    ///
    /// # Type Parameters
    /// * `V` - The value parser type
    ///
    /// # Arguments
    /// * `s` - The state to get the value for
    ///
    /// # Returns
    /// Result containing the parsed value or an error
    pub fn value<V: ValueParser>(&self, s: S) -> Result<V::Out<'_>, V::Error> {
        let val = self.values.get(&s).and_then(|v| v.first());

        V::parse(val)
    }

    pub fn pop_raw_value(&mut self, s: S) -> Result<String, Error> {
        self.values
            .get_mut(&s)
            .and_then(|v| v.pop())
            .ok_or_else(|| Error::Value(format!("{s:?}")))
    }

    pub fn raw_values(&self, s: S) -> Result<&[String], Error> {
        self.values
            .get(&s)
            .map(|v| v.as_ref())
            .ok_or_else(|| Error::Value(format!("{s:?}")))
    }

    /// Parses command-line arguments from an iterator.
    ///
    /// # Type Parameters
    /// * `I` - The iterator type
    ///
    /// # Arguments
    /// * `iter` - The iterator of arguments to parse
    ///
    /// # Returns
    /// Result containing unprocessed arguments or an error
    pub fn parse<I: Iterator>(&mut self, iter: I) -> Result<Vec<String>, Error>
    where
        I::Item: ToString,
    {
        let mut iter = iter.map(|v| v.to_string());
        let mut rets = vec![];

        while let Some(item) = iter.next() {
            let mut matched = false;

            for opt in self.opts.iter_mut() {
                if let opt::Match::Okay(val) = opt.r#match(&item) {
                    matched = true;

                    let val = if opt.consume() {
                        if let Some(val) = val {
                            val.to_string()
                        } else {
                            iter.next()
                                .ok_or_else(|| Error::Argument(opt.name().to_string()))?
                        }
                    } else {
                        String::default()
                    };

                    self.values
                        .entry(opt.state().clone())
                        .or_default()
                        .push(val);
                }
            }
            if !matched {
                rets.push(item);
            }
        }
        Ok(rets)
    }

    /// Parses arguments from the environment.
    ///
    /// # Returns
    /// Result containing unprocessed arguments or an error
    pub fn parse_env(&mut self) -> Result<Vec<String>, Error> {
        self.parse(std::env::args())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        opt::{option, switch},
        Error,
    };

    #[test]
    fn opt_test() {
        assert!(opt_test_impl().is_ok());
    }

    fn opt_test_impl() -> Result<(), Error> {
        use super::*;

        #[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
        enum TestState {
            Cmd,
            Bool,
            Option,
            Help,
            #[default]
            Unkown,
        }

        let mut cute = Cute::new();

        cute.add(switch("-bool", TestState::Bool));
        cute.add(option("-opt", TestState::Option));
        cute.add(switch("/?", TestState::Help));
        cute.add(switch("cmd", TestState::Cmd));

        let args: Vec<String> = ["cmd", "-bool", "-opt", "value", "/?"]
            .iter()
            .map(|data| String::from(*data))
            .collect();

        assert!(cute.parse(&mut args.into_iter()).is_ok(),);
        assert!(cute.value::<bool>(TestState::Bool)?,);
        assert!(cute.value::<bool>(TestState::Help)?,);
        assert!(cute.value::<bool>(TestState::Cmd)?,);
        assert_eq!(
            cute.value::<String>(TestState::Option)?,
            String::from("value")
        );
        Ok(())
    }
}
