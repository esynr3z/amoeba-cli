pub struct CLI {
    pub greeting: &'static str,
    pub prompt: &'static str,
    pub cmds: Vec<Cmd>,
}

impl Default for CLI {
    fn default() -> CLI {
        CLI {
            greeting: "
        @@@@@@@
        @@@@@   @@@@@@
      @@@@          @@@@@@@@,
      @@@                  @@@
     @@@@                   @@
  (@@@@.          @@@@@    @@@
@@@@@     @@@@@             @@@
@@@@       @@@@@              @@@@
@@@                @@*         @@@
@@@@             @@@@@         @@@
 *@@@@            @@@          @@@@
   @@@@                     @@@@@
    @@@     amoeba-cli     @@@@
   ,@@@@                  @@@
    %@@@@@@@@@@@@@@@@@@@@@@
                   @@@@@@
Type command and press 'Enter'. Use 'help' to list all available commands
or 'help foobar' to get more details about specific command.
",
            prompt: ">",
            cmds: Vec::new(),
        }
    }
}

impl CLI {
    fn get_cmd_by_name(&self, name: &str) -> Option<&Cmd> {
        self.cmds.iter().filter(|x| x.name == name).next()
    }

    fn help(&self, args: Vec<&str>) -> Result<String, String> {
        if args.len() == 0 {
            let mut help_str = "Available commands:\n".to_string();
            for cmd in self.cmds.iter() {
                help_str.push_str(&format!("  {:10} {}\n", cmd.name, cmd.descr));
            }
            help_str.push_str("Use 'help <command> to get more details.");
            Ok(help_str)
        } else {
            let cmd_name = args[0];
            match self.get_cmd_by_name(cmd_name) {
                Some(cmd) => Ok(cmd.help.to_string()),
                None => Err(format!("command '{}' was not found!", cmd_name)),
            }
        }
    }

    pub fn parse(&self, raw_str: &String) -> Result<String, String> {
        // get command name and arguments from the input string
        let mut args_iter = raw_str.split_whitespace();
        let cmd_name = match args_iter.next() {
            Some(name) => name,
            None => return Err("Empty command name!".to_string()),
        };
        let cmd_args: Vec<&str> = args_iter.collect();
        // execute selected command
        if cmd_name == "help" {
            self.help(cmd_args)
        } else {
            match self.get_cmd_by_name(cmd_name) {
                Some(cmd) => (cmd.callback)(cmd_args),
                None => Err(format!("command '{}' was not found!", cmd_name)),
            }
        }
    }
}

pub struct Cmd {
    pub name: &'static str,
    pub descr: &'static str,
    pub help: &'static str,
    pub callback: Box<dyn Fn(Vec<&str>) -> Result<String, String>>,
}

pub mod arg_utils {
    pub fn check_len(args: &Vec<&str>, expected_len: usize) -> Result<(), String> {
        if args.len() < expected_len {
            Err(format!("Not enough arguments - expected {}!", expected_len))
        } else {
            Ok(())
        }
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
                "Not able to convert '{}' to {} integer!",
                s,
                std::any::type_name::<T>(),
            )),
        }
    }

    pub fn float_from_str<T>(s: &str) -> Result<T, String>
    where
        T: num::Float + std::str::FromStr,
    {
        match s.parse::<T>() {
            Ok(val) => Ok(val),
            Err(_) => Err(format!(
                "Not able to convert '{}' to {} float!",
                s,
                std::any::type_name::<T>()
            )),
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
            Err(format!("Not able to convert '{}' to bool!", &s))
        }
    }
}
