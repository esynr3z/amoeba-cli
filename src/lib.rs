pub struct Cmd {
    pub name: &'static str,
    pub descr: &'static str,
    pub args: Vec<Arg>,
}

pub struct Arg {
    pub name: &'static str,
    pub descr: &'static str,
    pub type_id: &'static str,
}

pub fn int_from_str<T>(s: &str) -> Result<T, String>
where
    T: num::Integer + num::Bounded + std::fmt::Display,
{
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
            "Not able to convert '{}' to integer, allowed range is [{};{}]",
            s,
            <T>::min_value(),
            <T>::max_value(),
        )),
    }
}

pub fn float_from_str<T>(s: &str) -> Result<T, String>
where
    T: num::Float + std::str::FromStr,
{
    match s.parse::<T>() {
        Ok(val) => Ok(val),
        Err(_) => Err(format!("Not able to convert '{}' to float!", s)),
    }
}

pub fn bool_from_str(s: &str) -> Result<bool, String> {
    let true_aliases = ["true", "yes", "on", "enable", "y", "1"];
    let false_aliases = ["false", "no", "off", "disable", "n", "0"];
    if true_aliases.contains(&s) {
        Ok(true)
    } else if false_aliases.contains(&s) {
        Ok(false)
    } else {
        Err(format!("Not able to convert '{}' to bool!", s))
    }
}

pub trait CmdParser {
    fn get_cmd_by_name(&self, name: &str) -> Option<&Cmd>;

    fn parse(&self, raw_str: &String) -> Result<String, String> {
        // get command name and arguments from the input string
        let mut args = raw_str.split_whitespace();
        let cmd_name = match args.next() {
            Some(name) => name,
            None => return Err("Empty command name!".to_string()),
        };
        let cmd_args: Vec<&str> = args.collect();
        dbg!(&cmd_name, &cmd_args);
        // execute selected command
        match self.get_cmd_by_name(cmd_name) {
            Some(cmd) => return Ok(format!("command '{}' was executed!", cmd.name)),
            None => return Err(format!("command '{}' was not found!", cmd_name)),
        }
    }
}
