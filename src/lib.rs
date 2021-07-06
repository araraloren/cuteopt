use std::fmt::Debug;

const DEFAULT_STR: &'static str = "";

pub mod prelude {
    pub use super::Arg;
    pub use super::Ctx;
}

/// [`Arg`] hold option name and state
#[derive(Debug, Clone)]
pub enum Arg<'a, S>
where
    S: Debug + Clone + Eq + Default,
{
    Bool(&'a str, S),
    Opt(&'a str, S),
}

impl<'a, S> Arg<'a, S>
where
    S: Debug + Clone + Eq + Default,
{
    pub fn name(&self) -> &'a str {
        match self {
            Arg::Bool(name, _) | Arg::Opt(name, _) => name,
        }
    }

    pub fn get_state(&self) -> &S {
        match self {
            Arg::Bool(_, state) | Arg::Opt(_, state) => &state,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Arg::Bool(_, _) => true,
            _ => false,
        }
    }
}

/// [`Value`] hold the option value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    Str(String),
    None,
}

impl Value {
    pub fn from<'a, S>(arg: &Arg<'a, S>, value: String) -> Value
    where
        S: Debug + Clone + Eq + Default,
    {
        match arg {
            Arg::Bool(_, _) => Value::Bool(true),
            Arg::Opt(_, _) => Value::Str(value),
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(boolean) => *boolean,
            _ => false,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Value::Str(string) => string,
            _ => DEFAULT_STR,
        }
    }
}

/// An simple option data struct
#[derive(Debug, Clone)]
pub struct OptKeeper<'a, S>
where
    S: std::fmt::Debug + Clone + Default + Eq,
{
    pub opt: Arg<'a, S>,
    pub value: Value,
}

impl<'a, S> OptKeeper<'a, S>
where
    S: std::fmt::Debug + Clone + Default + Eq,
{
    pub fn name(&self) -> &'a str {
        self.opt.name()
    }

    pub fn state(&self) -> &S {
        self.opt.get_state()
    }
}

/// [`Ctx`] hold all the [`OptKeeper`]s,
/// provide the inteface parse the command line arguments.
/// 
/// ```no_run
/// use cuteopt::prelude::*;
/// 
/// #[derive(Debug, Clone, Eq, PartialEq)]
/// enum ParseState {
///     PSBoolean,
///     PSString,
///     PSDefault,
/// }
/// 
/// impl Default for ParseState {
///     fn default() -> Self {
///         Self::PSDefault
///     }
/// }
/// 
/// let mut ctx = Ctx::new();
/// 
/// ctx.add_bool("--boolean", ParseState::PSBoolean);
/// ctx.add_str("--string", ParseState::PSString);
/// 
/// ctx.parse(&mut std::env::args().skip(1));
/// 
/// // using ctx result
/// // dbg!(ctx.get_value_as_bool(ParseState::PSBoolean));
/// ```
#[derive(Debug, Default)]
pub struct Ctx<'a, S>
where
    S: std::fmt::Debug + Clone + Default + Eq,
{
    opt_keeper_repo: Vec<OptKeeper<'a, S>>,
}

impl<'a, S> Ctx<'a, S>
where
    S: std::fmt::Debug + Clone + Default + Eq,
{
    pub fn new() -> Self {
        Ctx {
            opt_keeper_repo: vec![],
        }
    }

    pub fn add(&mut self, arg: Arg<'a, S>) -> &mut Self {
        let value = match arg {
            Arg::Bool(_, _) => Value::Bool(false),
            _ => Value::None,
        };
        self.opt_keeper_repo.push(OptKeeper { opt: arg, value });
        self
    }

    pub fn add_bool(&mut self, name: &'a str, s: S) -> &mut Self {
        self.opt_keeper_repo.push(OptKeeper {
            opt: Arg::Bool(name, s),
            value: Value::Bool(false),
        });
        self
    }

    pub fn add_str(&mut self, name: &'a str, s: S) -> &mut Self {
        self.opt_keeper_repo.push(OptKeeper {
            opt: Arg::Opt(name, s),
            value: Value::None,
        });
        self
    }

    pub fn get(&self, s: S) -> Option<&Arg<'a, S>> {
        for opt_keeper in self.opt_keeper_repo.iter() {
            if opt_keeper.opt.get_state().clone() == s {
                return Some(&opt_keeper.opt);
            }
        }
        None
    }

    pub fn has(&self, s: S) -> bool {
        for opt_keeper in self.opt_keeper_repo.iter() {
            if opt_keeper.opt.get_state().clone() == s {
                return true;
            }
        }
        false
    }

    pub fn get_value(&self, s: S) -> Option<&Value> {
        for opt_keeper in self.opt_keeper_repo.iter() {
            if opt_keeper.opt.get_state().clone() == s {
                return Some(&opt_keeper.value);
            }
        }
        None
    }

    pub fn get_value_as_bool(&self, s: S) -> bool {
        if let Some(value) = self.get_value(s) {
            value.as_bool()
        } else {
            false
        }
    }

    pub fn get_value_as_str(&self, s: S) -> &str {
        if let Some(value) = self.get_value(s) {
            value.as_str()
        } else {
            DEFAULT_STR
        }
    }

    pub fn len(&self) -> usize {
        self.opt_keeper_repo.len()
    }

    fn _get_opt_i32(&self, index: i32) -> &OptKeeper<'a, S> {
        &self.opt_keeper_repo[index as usize]
    }

    fn _get_opt_mut_i32(&mut self, index: i32) -> &mut OptKeeper<'a, S> {
        &mut self.opt_keeper_repo[index as usize]
    }

    pub fn parse(
        &mut self,
        args: &mut impl Iterator<Item = String>,
    ) -> Result<Vec<String>, String> {
        let mut while_flag = true;
        let mut ret = vec![];

        while while_flag {
            let mut current_index: i32 = -1;

            match args.next() {
                Some(arg) => {
                    for index in 0..self.len() {
                        if self._get_opt_i32(index as i32).name() == arg {
                            current_index = index as i32;
                            break;
                        }
                    }

                    if current_index == -1 {
                        ret.push(arg);
                    }
                }
                None => {
                    while_flag = false;
                }
            }

            if current_index != -1 {
                if self._get_opt_i32(current_index).opt.is_bool() {
                    self._get_opt_mut_i32(current_index).value = Value::Bool(true);
                } else {
                    match args.next() {
                        Some(value) => {
                            self._get_opt_mut_i32(current_index).value = Value::Str(value);
                        }
                        None => {
                            return Err(format!(
                                "Option need argument: {:?}",
                                self._get_opt_i32(current_index).opt
                            ));
                        }
                    }
                }
            }
        }
        return Ok(ret);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn opt_test() {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Eq)]
        enum TestState {
            CmdState,
            BoolState,
            OptionState,
            HelpState,
            UnknowState,
        }

        impl Default for TestState {
            fn default() -> TestState {
                TestState::UnknowState
            }
        }

        let mut ctx = Ctx::new();

        ctx.add(Arg::Bool("-bool", TestState::BoolState));
        ctx.add(Arg::Opt("-opt", TestState::OptionState));
        ctx.add(Arg::Bool("/?", TestState::HelpState));
        ctx.add(Arg::Bool("cmd", TestState::CmdState));

        let args: Vec<String> = ["cmd", "-bool", "-opt", "value", "/?"]
            .iter()
            .map(|data| String::from(*data))
            .collect();

        assert_eq!(ctx.parse(&mut args.into_iter()).is_ok(), true);
        assert_eq!(
            ctx.get_value(TestState::BoolState),
            Some(&Value::Bool(true))
        );
        assert_eq!(
            ctx.get_value(TestState::HelpState),
            Some(&Value::Bool(true))
        );
        assert_eq!(ctx.get_value(TestState::CmdState), Some(&Value::Bool(true)));
        assert_eq!(
            ctx.get_value(TestState::OptionState),
            Some(&Value::Str(String::from("value")))
        );
    }
}
