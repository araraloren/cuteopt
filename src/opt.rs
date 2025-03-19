use std::{fmt::Debug, hash::Hash};

/// A trait representing a state that can be used in command-line options
pub trait State: Debug + Clone + Eq + Hash + Default {}

impl<T: Debug + Clone + Eq + Hash + Default> State for T {}

/// Represents the result of matching a command-line argument
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Match<'a> {
    /// Successful match with optional value
    Okay(Option<&'a str>),
    /// Failed match
    None,
}

/// Trait defining the behavior of a command-line option
pub trait StateOpt {
    /// The associated state type
    type S: State;

    /// Returns the name of the option
    fn name(&self) -> &str;

    /// Returns the associated state
    fn state(&self) -> &Self::S;

    /// Indicates whether this option consumes the next argument
    fn consume(&self) -> bool {
        true
    }

    /// Attempts to match the given argument with this option
    fn r#match<'a>(&self, arg: &'a str) -> Match<'a> {
        if let Some((name, val)) = arg.split_once('=') {
            if name == self.name() {
                Match::Okay(Some(val))
            } else {
                Match::None
            }
        } else if self.name() == arg {
            Match::Okay(None)
        } else {
            Match::None
        }
    }
}

/// Represents a command-line option with its name, state and consume behavior
pub struct Opt<'a, S> {
    name: &'a str,
    state: S,
    consume: bool,
}

impl<'a, S> Opt<'a, S> {
    /// Creates a new Opt instance
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

/// Creates a new switch option that doesn't consume the next argument
pub fn switch<S: State>(name: &str, state: S) -> Opt<'_, S> {
    Opt::new(name, state, false)
}

/// Creates a new option that consumes the next argument
pub fn option<S: State>(name: &str, state: S) -> Opt<'_, S> {
    Opt::new(name, state, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
    enum TestState {
        On,
        #[default]
        Unknown,
    }

    #[test]
    fn test_opt_new() {
        let opt = Opt::new("test", TestState::On, true);
        assert_eq!(opt.name, "test");
        assert!(opt.consume);
    }

    #[test]
    fn test_stateopt_implementation() {
        let opt = Opt::new("test", TestState::On, false);
        assert_eq!(opt.name(), "test");
        assert!(!opt.consume());
    }

    #[test]
    fn test_match_method() {
        let opt = Opt::new("test", TestState::On, true);

        assert_eq!(opt.r#match("test"), Match::Okay(None));
        assert_eq!(opt.r#match("test=value"), Match::Okay(Some("value")));
        assert_eq!(opt.r#match("testing"), Match::None);
        assert_eq!(opt.r#match("other"), Match::None);
    }

    #[test]
    fn test_switch_function() {
        let opt = switch("test", TestState::On);
        assert_eq!(opt.name, "test");
        assert!(!opt.consume);
    }

    #[test]
    fn test_option_function() {
        let opt = option("test", TestState::On);
        assert_eq!(opt.name, "test");
        assert!(opt.consume);
    }
}
