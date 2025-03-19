pub mod err;
pub mod opt;
pub mod val;

pub mod prelude {
    pub use crate::opt::*;
    pub use crate::val::*;
    pub use crate::Ctx;
}

pub use err::Error;

use std::collections::HashMap;

use opt::{State, StateOpt};
use val::ValueParser;

#[derive(Default)]
pub struct Ctx<S: State> {
    opts: Vec<Box<dyn StateOpt<S = S>>>,
    values: HashMap<S, Vec<String>>,
}

impl<S: State> Ctx<S> {
    pub fn new() -> Self {
        Self {
            opts: vec![],
            values: HashMap::default(),
        }
    }

    pub fn add(&mut self, arg: impl StateOpt<S = S> + 'static) -> &mut Self {
        self.opts.push(Box::new(arg));
        self
    }

    pub fn get(&self, s: S) -> Option<&dyn StateOpt<S = S>> {
        self.opts.iter().find(|v| v.state() == &s).map(|v| &**v)
    }

    pub fn has(&self, s: S) -> bool {
        self.opts.iter().any(|v| v.state() == &s)
    }

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

    pub fn parse<I: Iterator>(&mut self, iter: I) -> Result<Vec<String>, Error>
    where
        I::Item: ToString,
    {
        let mut iter = iter.map(|v| v.to_string());
        let mut rets = vec![];

        while let Some(item) = iter.next() {
            let mut matched = false;

            for opt in self.opts.iter_mut() {
                if let opt::Match::Ok(val) = opt.r#match(&item) {
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

        let mut ctx = Ctx::new();

        ctx.add(switch("-bool", TestState::Bool));
        ctx.add(option("-opt", TestState::Option));
        ctx.add(switch("/?", TestState::Help));
        ctx.add(switch("cmd", TestState::Cmd));

        let args: Vec<String> = ["cmd", "-bool", "-opt", "value", "/?"]
            .iter()
            .map(|data| String::from(*data))
            .collect();

        assert!(ctx.parse(&mut args.into_iter()).is_ok(),);
        assert!(ctx.value::<bool>(TestState::Bool)?,);
        assert!(ctx.value::<bool>(TestState::Help)?,);
        assert!(ctx.value::<bool>(TestState::Cmd)?,);
        assert_eq!(
            ctx.value::<String>(TestState::Option)?,
            String::from("value")
        );
        Ok(())
    }
}
