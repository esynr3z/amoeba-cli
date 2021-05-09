use amoeba_cli as acli;
use amoeba_cli::Cli;
use core::fmt::Write as core_fmt_Write;
use std::io;
use std::io::Write as std_io_Write; // for flush()

const BUF_CAP: usize = 1024;
type CliArgs<'a> = acli::ArgsIter<'a>;
type CliCmd = acli::Cmd<BUF_CAP>;
type CliErr = acli::Error;
type CliBuf = acli::StrBuf<BUF_CAP>;

fn cmd_led(args: &mut CliArgs) -> Result<CliBuf, CliErr> {
    let state = acli::arg_utils::bool_from_str(args.next())?;
    if state {
        Ok(CliBuf::from("Led is ON now")?)
    } else {
        Ok(CliBuf::from("Led is OFF now")?)
    }
}

fn cmd_rgb(args: &mut CliArgs) -> Result<CliBuf, CliErr> {
    let r = acli::arg_utils::int_from_str::<u8>(args.next())?;
    let g = acli::arg_utils::int_from_str::<u8>(args.next())?;
    let b = acli::arg_utils::int_from_str::<u8>(args.next())?;
    let mut res = CliBuf::new();
    write!(res, "Ok, R={}, G={}, B={}", r, g, b)?;
    Ok(res)
}

fn cmd_id(args: &mut CliArgs) -> Result<CliBuf, CliErr> {
    let id = acli::arg_utils::unwrap(args.next())?;
    let mut res = CliBuf::new();
    write!(res, "Ok, id='{}'", id)?;
    Ok(res)
}

fn cmd_exit(_args: &mut CliArgs) -> Result<CliBuf, CliErr> {
    std::process::exit(0);
}

struct AppCli<const CMD_N: usize> {
    greeting: &'static str,
    prompt: &'static str,
    cmds: [CliCmd; CMD_N],
}

impl<const CMD_N: usize> Cli<CMD_N, BUF_CAP> for AppCli<CMD_N> {
    fn get_cmd_by_name(&self, name: &str) -> Option<&CliCmd> {
        self.cmds.iter().filter(|x| x.name == name).next()
    }
    fn get_cmds(&self) -> &[CliCmd; CMD_N] {
        &self.cmds
    }
}

const CLI: AppCli<4> = AppCli {
    greeting: "@@@@ amoeba-cli @@@@
Type command and press 'Enter'. Use 'help' to list all available commands
or 'help foobar' to get more details about specific command.
",
    prompt: "> ",
    cmds: [
        CliCmd {
            name: "led",
            descr: "led control",
            help: "Use 'led on' or 'led off' to control the state of the led.",
            callback: cmd_led,
        },
        CliCmd {
            name: "rgb",
            descr: "RGB led control",
            help:
                "rgb <red> <green> <blue>\nUse values from 0 to 255 to specify channel brightness.",
            callback: cmd_rgb,
        },
        CliCmd {
            name: "id",
            descr: "set device id",
            help: "id <val>\nID have to be a string value.",
            callback: cmd_id,
        },
        CliCmd {
            name: "exit",
            descr: "exit CLI",
            help: "Yep, no jokes, program will be terminated.",
            callback: cmd_exit,
        },
    ],
};

fn main() {
    // construct comand-line interface with specific commands
    print!("{}", CLI.greeting);
    // imitate new string arrive (e.g. from UART)
    loop {
        let mut raw_str = String::new();
        print!("{}", CLI.prompt);
        io::stdout().flush().unwrap(); // to ensure the output is emitted immediately
        io::stdin()
            .read_line(&mut raw_str)
            .expect("Failed to read line");
        // parse the input string and print the result
        match CLI.exec(&raw_str) {
            Ok(msg) => println!("{}", msg),
            Err(etype) => match etype {
                CliErr::CmdFailure => eprintln!("command execution failed!"),
                CliErr::BufferOverflow => eprintln!("string buffer overflow!"),
                CliErr::CmdNotFound => eprintln!("command was not found!"),
                CliErr::EmptyCmd => eprintln!("empty command!"),
                CliErr::NotEnoughArgs => eprintln!("not enough arguments for the command!"),
                CliErr::InvalidArgType => eprintln!("invalid argument type!"),
            },
        }
    }
}
