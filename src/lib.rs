#![no_std]

use core::fmt::Write;

pub type StrBuf<const CAP: usize> = arrayvec::ArrayString<CAP>;

pub type ArgsIter<'a> = core::str::SplitWhitespace<'a>;

#[derive(Debug)]
pub enum Error {
    CmdFailure,
    BufferOverflow,
    CmdNotFound,
    EmptyCmd,
    NotEnoughArgs,
    InvalidArgType,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::CmdFailure => "Command execution failed!",
                Error::BufferOverflow => "String buffer overflow!",
                Error::CmdNotFound => "Command was not found!",
                Error::EmptyCmd => "Empty command!",
                Error::NotEnoughArgs => "Not enough arguments for the command!",
                Error::InvalidArgType => "Invalid argument type!",
            }
        )
    }
}

impl From<arrayvec::CapacityError<&str>> for Error {
    fn from(_item: arrayvec::CapacityError<&str>) -> Self {
        Self::BufferOverflow
    }
}

impl From<core::fmt::Error> for Error {
    fn from(_item: core::fmt::Error) -> Self {
        Self::BufferOverflow
    }
}

pub trait Cli<const CMD_N: usize, const BUF_CAP: usize> {
    fn get_cmd_by_name(&self, name: &str) -> Option<&Cmd<BUF_CAP>>;
    fn get_cmds(&self) -> &[Cmd<BUF_CAP>; CMD_N];

    fn help(&self, args: &mut ArgsIter) -> Result<StrBuf<BUF_CAP>, Error> {
        match args.next() {
            Some(cmd_name) => match self.get_cmd_by_name(cmd_name) {
                Some(cmd) => Ok(StrBuf::from(cmd.help)?),
                None => Err(Error::CmdNotFound),
            },
            None => {
                let mut help_str = StrBuf::from("Available commands:\n")?;
                for cmd in self.get_cmds().iter() {
                    write!(help_str, "  {:10} {}\n", cmd.name, cmd.descr)?;
                }
                help_str.try_push_str("Use 'help <command> to get more details.")?;
                Ok(help_str)
            }
        }
    }

    fn exec(&self, raw_str: &str) -> Result<StrBuf<BUF_CAP>, Error> {
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

pub struct Cmd<const BUF_CAP: usize> {
    pub name: &'static str,
    pub descr: &'static str,
    pub help: &'static str,
    pub callback: fn(&mut ArgsIter) -> Result<StrBuf<BUF_CAP>, Error>,
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
        T: num::Integer + num::Bounded + core::fmt::Display,
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
        T: num::Float + core::str::FromStr,
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
