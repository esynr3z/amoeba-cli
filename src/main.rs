use amoeba_cli::{arg_utils, ArgsIter, Cmd, CLI};
use std::io;
use std::io::Write; // for flush()

fn cmd_led(args: &mut ArgsIter) -> Result<String, String> {
    let state = arg_utils::bool_from_str(args.next())?;
    if state {
        Ok("Led is ON now".to_string())
    } else {
        Ok("Led is OFF now".to_string())
    }
}

fn cmd_rgb(args: &mut ArgsIter) -> Result<String, String> {
    let r = arg_utils::int_from_str::<u8>(args.next())?;
    let g = arg_utils::int_from_str::<u8>(args.next())?;
    let b = arg_utils::int_from_str::<u8>(args.next())?;
    Ok(format!("Ok, R={}, G={}, B={}", r, g, b))
}

fn cmd_id(args: &mut ArgsIter) -> Result<String, String> {
    let id = arg_utils::unwrap(args.next())?;
    Ok(format!("Ok, id='{}'", id))
}

fn cmd_exit(_args: &mut ArgsIter) -> Result<String, String> {
    std::process::exit(0);
}

struct AppCLI<const CMD_N: usize> {
    greeting: &'static str,
    prompt: &'static str,
    cmds: [Cmd; CMD_N],
}

impl<const CMD_N: usize> CLI<CMD_N> for AppCLI<CMD_N> {
    fn get_cmd_by_name(&self, name: &str) -> Option<&Cmd> {
        self.cmds.iter().filter(|x| x.name == name).next()
    }
    fn get_cmds(&self) -> &[Cmd; CMD_N] {
        &self.cmds
    }
}

fn main() {
    // construct comand-line interface with specific commands
    let cli = AppCLI {
        greeting: "@@@@ amoeba-cli @@@@
Type command and press 'Enter'. Use 'help' to list all available commands
or 'help foobar' to get more details about specific command.
",
        prompt: "> ",
        cmds: [
            Cmd {
                name: "led",
                descr: "led control",
                help: "Use 'led on' or 'led off' to control the state of the led.",
                callback: Box::new(cmd_led),
            },
            Cmd {
                name: "rgb",
                descr: "RGB led control",
                help:
                    "rgb <red> <green> <blue>\nUse values from 0 to 255 to specify channel brightness.",
                callback: Box::new(cmd_rgb),
            },
            Cmd {
                name: "id",
                descr: "set device id",
                help: "id <val>\nID have to be a string value.",
                callback: Box::new(cmd_id),
            },
            Cmd {
                name: "exit",
                descr: "exit CLI",
                help: "Yep, no jokes, program will be terminated.",
                callback: Box::new(cmd_exit),
            },
        ],
    };
    print!("{}", cli.greeting);
    // imitate new string arrive (e.g. from UART)
    loop {
        let mut raw_str = String::new();
        print!("{}", cli.prompt);
        io::stdout().flush().unwrap(); // to ensure the output is emitted immediately
        io::stdin()
            .read_line(&mut raw_str)
            .expect("Failed to read line");
        // parse the input string and print the result
        match cli.parse(&raw_str) {
            Ok(msg) => println!("{}", msg),
            Err(msg) => println!("{}", msg),
        }
    }
}
