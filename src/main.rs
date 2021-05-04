use amoeba_cli::{arg_utils, Cmd, CLI};
use std::io;
use std::io::Write; // for flush()

fn cmd_led(args: Vec<&str>) -> Result<String, String> {
    arg_utils::check_len(&args, 1)?;
    let state = arg_utils::bool_from_str(args[0])?;
    if state {
        Ok("Led is ON now".to_string())
    } else {
        Ok("Led is OFF now".to_string())
    }
}

fn cmd_rgb(args: Vec<&str>) -> Result<String, String> {
    arg_utils::check_len(&args, 3)?;
    let r = arg_utils::int_from_str::<u8>(args[0])?;
    let g = arg_utils::int_from_str::<u8>(args[1])?;
    let b = arg_utils::int_from_str::<u8>(args[2])?;
    Ok(format!("Ok, R={}, G={}, B={}", r, g, b))
}

fn cmd_id(args: Vec<&str>) -> Result<String, String> {
    arg_utils::check_len(&args, 1)?;
    let id = args[0];
    Ok(format!("Ok, id='{}'", id))
}

fn cmd_exit(_args: Vec<&str>) -> Result<String, String> {
    std::process::exit(0);
}

fn main() {
    // construct comand-line interface with specific commands
    let cli = CLI {
        cmds: vec![
            Cmd {
                name: "led",
                descr: "led control",
                help: "Use 'led on' or 'led off' to control the state of the led.",
                callback: Box::new(cmd_led),
            },
            Cmd {
                name: "rgb",
                descr: "RGB led control",
                help: "rgb <red> <green> <blue>\nUse values from 0 to 255 to specify channel brightness.",
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
        ..Default::default()
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
