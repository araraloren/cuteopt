use std::{fmt::Debug, hash::Hash};

pub trait State: Debug + Clone + Eq + Hash + Default {}

impl<T: Debug + Clone + Eq + Hash + Default> State for T {}

pub enum Match<'a> {
    Ok(Option<&'a str>),
    Err,
}

pub trait StateOpt {
    type S: State;

    fn name(&self) -> &str;

    fn state(&self) -> &Self::S;

    fn consume(&self) -> bool {
        true
    }

    fn r#match<'a>(&self, arg: &'a str) -> Match<'a> {
        let matched = arg.starts_with(self.name());

        if matched {
            if let Some((name, val)) = arg.split_once('=') {
                if name == self.name() {
                    Match::Ok(Some(val))
                } else {
                    Match::Err
                }
            } else {
                Match::Ok(None)
            }
        } else {
            Match::Err
        }
    }
}

pub struct Opt<'a, S> {
    name: &'a str,
    state: S,
    consume: bool,
}

impl<'a, S> Opt<'a, S> {
    pub fn new(name: &'a str, state: S, consume: bool) -> Self {
        Self {
            name,
            state,
            consume,
        }
    }
}

impl<S: State> StateOpt for Opt<'_, S> {
    type S = S;

    fn name(&self) -> &str {
        self.name
    }

    fn state(&self) -> &Self::S {
        &self.state
    }

    fn consume(&self) -> bool {
        self.consume
    }
}

pub fn switch<S: State>(name: &str, state: S) -> Opt<'_, S> {
    Opt::new(name, state, false)
}

pub fn option<S: State>(name: &str, state: S) -> Opt<'_, S> {
    Opt::new(name, state, true)
}
