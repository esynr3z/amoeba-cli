use amoeba_cli::{utils, Error, Interpreter};
use core::fmt::Write as _core_fmt_Write;
use ncurses::{addstr, endwin, getch, initscr, nocbreak, raw, scrollok};

const CMD_BUF_SIZE: usize = 1024;
const LINE_BUF_SIZE: usize = 128;
const CMD_N: usize = 4;

type Cmd = amoeba_cli::Cmd<CMD_BUF_SIZE>;
type CmdBuf = amoeba_cli::ArrayString<CMD_BUF_SIZE>;
type LineBuf = amoeba_cli::ArrayString<LINE_BUF_SIZE>;

fn cmd_led<'a>(args: &mut dyn Iterator<Item = &'a str>) -> Result<CmdBuf, Error> {
    let state = utils::bool_from_str(args.next())?;
    if state {
        Ok(CmdBuf::from("Led is ON now")?)
    } else {
        Ok(CmdBuf::from("Led is OFF now")?)
    }
}

fn cmd_rgb<'a>(args: &mut dyn Iterator<Item = &'a str>) -> Result<CmdBuf, Error> {
    let r = utils::int_from_str::<u8>(args.next())?;
    let g = utils::int_from_str::<u8>(args.next())?;
    let b = utils::int_from_str::<u8>(args.next())?;
    let mut res = CmdBuf::new();
    write!(res, "Ok, R={}, G={}, B={}", r, g, b)?;
    Ok(res)
}

fn cmd_id<'a>(args: &mut dyn Iterator<Item = &'a str>) -> Result<CmdBuf, Error> {
    let id = utils::unwrap(args.next())?;
    let mut res = CmdBuf::new();
    write!(res, "Ok, id='{}'", id)?;
    Ok(res)
}

fn cmd_exit<'a>(_args: &mut dyn Iterator<Item = &'a str>) -> Result<CmdBuf, Error> {
    endwin();
    std::process::exit(0);
}

static CMDS: [Cmd; CMD_N] = [
    Cmd {
        name: "led",
        descr: "led control",
        help: "Use 'led on' or 'led off' to control the state of the led.",
        callback: cmd_led,
    },
    Cmd {
        name: "rgb",
        descr: "RGB led control",
        help: "rgb <red> <green> <blue>\nUse values from 0 to 255 to specify channel brightness.",
        callback: cmd_rgb,
    },
    Cmd {
        name: "id",
        descr: "set device id",
        help: "id <val>\nID have to be a string value.",
        callback: cmd_id,
    },
    Cmd {
        name: "exit",
        descr: "exit CLI",
        help: "Yep, no jokes, program will be terminated.",
        callback: cmd_exit,
    },
];

pub struct Cli {
    pub line_buf: LineBuf,
    pub cmds: &'static [Cmd],
}

impl Interpreter<CMD_BUF_SIZE, LINE_BUF_SIZE> for Cli {
    fn cmd_from_name(&self, name: &str) -> Option<&Cmd> {
        self.cmds.iter().filter(|x| x.name == name).next()
    }
    fn cmds_arr(&self) -> &[Cmd] {
        self.cmds
    }
    fn line_buf_mut(&mut self) -> &mut LineBuf {
        &mut self.line_buf
    }
    fn line_buf(&self) -> &LineBuf {
        &self.line_buf
    }
    fn print(&self, s: &str) {
        addstr(s);
    }
}

fn main() {
    let mut cli = Cli {
        line_buf: LineBuf::new(),
        cmds: &CMDS,
    };

    let w = initscr();
    scrollok(w, true);
    nocbreak();
    raw();
    cli.greeting();
    loop {
        let ch = getch().to_be_bytes()[3] as char;
        cli.put_char(ch);
    }
}
