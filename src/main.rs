use amoeba_cli::{Arg, Cmd, CmdParser};
use std::io;
use std::io::Write; // for flush()

struct CLI {
    cmds: Vec<Cmd>,
}

impl CmdParser for CLI {
    fn get_cmd_by_name(&self, name: &str) -> Option<&Cmd> {
        self.cmds.iter().filter(|x| x.name == name).next()
    }
}

fn cmd_sum(args: &Vec<&str>) -> Result<String, String> {
    let a = match args[0].parse::<u32>() {
        Ok(val) => val,
        Err(_) => return Err(format!("Argument '{}' can't be converted to u32", args[0])),
    };
    let b = match args[1].parse::<u32>() {
        Ok(val) => val,
        Err(_) => return Err(format!("Argument '{}' can't be converted to u32", args[0])),
    };
    Ok(format!("{}", a + b))
}

fn main() {
    // construct comand-line interface with specific commands
    let cli = CLI {
        cmds: vec![Cmd {
            name: "sum",
            descr: "sum of two arguments",
            args: vec![
                Arg {
                    name: "a",
                    descr: "argument a",
                    type_id: "u32",
                },
                Arg {
                    name: "b",
                    descr: "argument b",
                    type_id: "u32",
                },
            ],
        }],
    };
    // imitate new string arrive (e.g. from UART)
    loop {
        let mut raw_str = String::new();
        print!("> ");
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
