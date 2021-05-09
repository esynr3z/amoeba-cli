pub type ArgsIter<'a> = core::str::SplitWhitespace<'a>;

#[derive(Debug)]
pub enum Error {
    CmdNotFound,
    EmptyCmd,
    NotEnoughArgs,
    InvalidArgType,
}

pub trait Cli<const CMD_N: usize> {
    fn get_cmd_by_name(&self, name: &str) -> Option<&Cmd>;
    fn get_cmds(&self) -> &[Cmd; CMD_N];

    fn help(&self, args: &mut ArgsIter) -> Result<String, Error> {
        match args.next() {
            Some(cmd_name) => match self.get_cmd_by_name(cmd_name) {
                Some(cmd) => Ok(cmd.help.to_string()),
                None => Err(Error::CmdNotFound),
            },
            None => {
                let mut help_str = "Available commands:\n".to_string();
                for cmd in self.get_cmds().iter() {
                    help_str.push_str(&format!("  {:10} {}\n", cmd.name, cmd.descr));
                }
                help_str.push_str("Use 'help <command> to get more details.");
                Ok(help_str)
            }
        }
    }

    fn parse(&self, raw_str: &str) -> Result<String, Error> {
        // get command name and arguments from the input string
        let mut args = raw_str.split_whitespace();
        let cmd_name = match args.next() {
            Some(name) => name,
            None => return Err(Error::EmptyCmd),
        };
        // execute selected command
        if cmd_name == "help" {
            self.help(&mut args)
        } else {
            match self.get_cmd_by_name(cmd_name) {
                Some(cmd) => (cmd.callback)(&mut args),
                None => Err(Error::CmdNotFound),
            }
        }
    }
}

pub struct Cmd {
    pub name: &'static str,
    pub descr: &'static str,
    pub help: &'static str,
    pub callback: Box<dyn Fn(&mut ArgsIter) -> Result<String, Error>>,
}

pub mod arg_utils {
    use super::Error;
    pub fn unwrap(s: Option<&str>) -> Result<&str, Error> {
        match s {
            Some(val) => Ok(val),
            None => Err(Error::NotEnoughArgs),
        }
    }

    pub fn int_from_str<T>(s: Option<&str>) -> Result<T, Error>
    where
        T: num::Integer + num::Bounded + std::fmt::Display,
    {
        let s = unwrap(s)?;
        let mut radix = 10;
        let mut s_clean = s;
        if s.starts_with("0x") {
            radix = 16;
            s_clean = s.trim_start_matches("0x");
        } else if s.starts_with("0b") {
            radix = 2;
            s_clean = s.trim_start_matches("0b");
        }
        match <T>::from_str_radix(s_clean, radix) {
            Ok(val) => Ok(val),
            Err(_) => Err(Error::InvalidArgType),
        }
    }

    pub fn float_from_str<T>(s: Option<&str>) -> Result<T, Error>
    where
        T: num::Float + std::str::FromStr,
    {
        let s = unwrap(s)?;
        match s.parse::<T>() {
            Ok(val) => Ok(val),
            Err(_) => Err(Error::InvalidArgType),
        }
    }

    pub fn bool_from_str(s: Option<&str>) -> Result<bool, Error> {
        let s = unwrap(s)?;
        let true_aliases = ["true", "yes", "on", "enable", "y", "1"];
        let false_aliases = ["false", "no", "off", "disable", "n", "0"];
        if true_aliases.contains(&s) {
            Ok(true)
        } else if false_aliases.contains(&s) {
            Ok(false)
        } else {
            Err(Error::InvalidArgType)
        }
    }
}
