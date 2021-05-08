 pub type ArgsIter<'a> = core::str::SplitWhitespace<'a>;

pub trait CLI<const CMD_N: usize> {
    fn get_cmd_by_name(&self, name: &str) -> Option<&Cmd>;
    fn get_cmds(&self) -> &[Cmd; CMD_N];

    fn help(&self, args: &mut ArgsIter) -> Result<String, String> {
        match args.next() {
            Some(cmd_name) => match self.get_cmd_by_name(cmd_name) {
                Some(cmd) => Ok(cmd.help.to_string()),
                None => Err(format!("command '{}' was not found!", cmd_name)),
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

    fn parse(&self, raw_str: &str) -> Result<String, String> {
        // get command name and arguments from the input string
        let mut args = raw_str.split_whitespace();
        let cmd_name = match args.next() {
            Some(name) => name,
            None => return Err("Empty command name!".to_string()),
        };
        // execute selected command
        if cmd_name == "help" {
            self.help(&mut args)
        } else {
            match self.get_cmd_by_name(cmd_name) {
                Some(cmd) => (cmd.callback)(&mut args),
                None => Err(format!("command '{}' was not found!", cmd_name)),
            }
        }
    }
}

pub struct Cmd {
    pub name: &'static str,
    pub descr: &'static str,
    pub help: &'static str,
    pub callback: Box<dyn Fn(&mut ArgsIter) -> Result<String, String>>,
}

pub mod arg_utils {
    pub fn unwrap(s: Option<&str>) -> Result<&str, String> {
        match s {
            Some(val) => Ok(val),
            None => Err("Not enough arguments".to_string()),
        }
    }

    pub fn int_from_str<T>(s: Option<&str>) -> Result<T, String>
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
            Err(_) => Err(format!(
                "Not able to convert '{}' to {} integer!",
                s,
                std::any::type_name::<T>(),
            )),
        }
    }

    pub fn float_from_str<T>(s: Option<&str>) -> Result<T, String>
    where
        T: num::Float + std::str::FromStr,
    {
        let s = unwrap(s)?;
        match s.parse::<T>() {
            Ok(val) => Ok(val),
            Err(_) => Err(format!(
                "Not able to convert '{}' to {} float!",
                s,
                std::any::type_name::<T>()
            )),
        }
    }

    pub fn bool_from_str(s: Option<&str>) -> Result<bool, String> {
        let s = unwrap(s)?;
        let true_aliases = ["true", "yes", "on", "enable", "y", "1"];
        let false_aliases = ["false", "no", "off", "disable", "n", "0"];
        if true_aliases.contains(&s) {
            Ok(true)
        } else if false_aliases.contains(&s) {
            Ok(false)
        } else {
            Err(format!("Not able to convert '{}' to bool!", &s))
        }
    }
}
