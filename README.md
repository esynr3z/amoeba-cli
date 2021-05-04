# Command-line interpreter as primitive as amoeba

Quite naive implementation of CLI in Rust. Just

* construct structure with commands
* specify descriptions and callbacks
* and CLI goes brrr

```
$ cargo run

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
>help
Available commands:
  led        led control
  rgb        RGB led control
  id         set device id
  exit       exit CLI
Use 'help <command> to get more details.
>led on
Led is ON now
>help rgb
rgb <red> <green> <blue>
Use values from 0 to 255 to specify channel brightness.
>rgb 0x10 0b10101010 128
Ok, R=16, G=170, B=128
>id Mr.Krabs
Ok, id='Mr.Krabs'
>exit
```

