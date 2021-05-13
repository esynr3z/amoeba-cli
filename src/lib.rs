#![no_std]

pub use arrayvec::ArrayString;
use core::fmt::Write;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    CmdFailure,
    BufferOverflow,
    CmdNotFound,
    EmptyCmd,
    NotEnoughArgs,
    InvalidArgType,
}

impl Error {
    fn to_str(&self) -> &'static str {
        match self {
            Error::CmdFailure => "Command execution failed!",
            Error::BufferOverflow => "String buffer overflow!",
            Error::CmdNotFound => "Command was not found!",
            Error::EmptyCmd => "Empty command!",
            Error::NotEnoughArgs => "Not enough arguments for the command!",
            Error::InvalidArgType => "Invalid argument type!",
        }
    }
}

impl<T> From<arrayvec::CapacityError<T>> for Error {
    fn from(_item: arrayvec::CapacityError<T>) -> Self {
        Self::BufferOverflow
    }
}

impl From<core::fmt::Error> for Error {
    fn from(_item: core::fmt::Error) -> Self {
        Self::BufferOverflow
    }
}

pub trait Interpreter<const CMD_BUF_SIZE: usize, const LINE_BUF_SIZE: usize> {
    const GREETING: &'static str = "@@@@ amoeba-cli @@@@
Type command and press 'Enter'. Use 'help' to list all available commands
or 'help foobar' to get more details about specific command.
";
    const PROMPT: &'static str = "> ";

    fn cmd_from_name(&self, name: &str) -> Option<&Cmd<CMD_BUF_SIZE>>;
    fn cmds_arr(&self) -> &[Cmd<CMD_BUF_SIZE>];
    fn line_buf_mut(&mut self) -> &mut ArrayString<LINE_BUF_SIZE>;
    fn line_buf(&self) -> &ArrayString<LINE_BUF_SIZE>;
    fn print(&self, s: &str);

    fn greeting(&self) {
        self.print_lines(Self::GREETING.lines()).unwrap();
        self.print(Self::PROMPT);
    }

    fn print_line(&self, line: &str) -> Result<(), Error> {
        let mut line_buf = ArrayString::<LINE_BUF_SIZE>::from(line)?;
        line_buf.try_push_str("\n")?;
        self.print(&line_buf);
        Ok(())
    }

    fn print_lines<'a, I>(&self, lines: I) -> Result<(), Error>
    where
        I: Iterator<Item = &'a str>,
    {
        for line in lines {
            self.print_line(&line)?;
        }
        Ok(())
    }

    fn help<'a>(&self, args: &mut dyn Iterator<Item = &'a str>) -> Result<ArrayString<CMD_BUF_SIZE>, Error> {
        let mut help_str = ArrayString::<CMD_BUF_SIZE>::new();
        match args.next() {
            Some(name) => match self.cmd_from_name(name) {
                Some(cmd) => {
                    help_str.try_push_str(cmd.help)?;
                    Ok(help_str)
                }
                None => Err(Error::CmdNotFound),
            },
            None => {
                help_str.try_push_str("Available commands:\n")?;
                for cmd in self.cmds_arr().iter() {
                    write!(help_str, "  {:10} {}\n", cmd.name, cmd.descr)?;
                }
                help_str.try_push_str("Use 'help <command> to get more details.")?;
                Ok(help_str)
            }
        }
    }

    fn put_char(&mut self, ch: char) {
        match ch {
            // if endline symbol received - time to execute the command
            '\n' => {
                if let Err(e) = self.exec(self.line_buf().as_str()) {
                    self.print_line(e.to_str()).unwrap();
                }
                self.print(Self::PROMPT);
                self.line_buf_mut().clear();
            }
            // try to push any other char to buffer and ignore buffer overflow
            _ => self.line_buf_mut().try_push(ch).unwrap_or_else(|_e| ()),
        };
    }

    fn exec(&self, cmd_line: &str) -> Result<(), Error> {
        // get command name and arguments from the input string
        let mut args = cmd_line.split_whitespace();
        let cmd_name = match args.next() {
            Some(name) => name,
            None => return Err(Error::EmptyCmd),
        };
        // execute selected command
        if cmd_name == "help" {
            Ok(self.print_lines(self.help(&mut args)?.lines())?)
        } else if let Some(cmd) = self.cmd_from_name(cmd_name) {
            Ok(self.print_lines((cmd.callback)(&mut args)?.lines())?)
        } else {
            Err(Error::CmdNotFound)
        }
    }
}

pub struct Cmd<const BUF_SIZE: usize> {
    pub name: &'static str,
    pub descr: &'static str,
    pub help: &'static str,
    pub callback: for<'a> fn(&mut dyn Iterator<Item = &'a str>) -> Result<ArrayString<BUF_SIZE>, Error>,
}

pub mod utils {
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
