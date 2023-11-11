extern crate getopts;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optflag("", "version", "Print version");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f) }
    };

    if matches.opt_present("version") {
        const VERSION: &str = env!("CARGO_PKG_VERSION");

        println!("schmergles {}", VERSION);
        println!("Copyright (C) 2023 Lukas Jonathan Gutschmidt");
        println!("License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>");
        println!("There is NO WARRANTY, to the extent permitted by law.");
        return
    }

    println!("Hello, world!");
}
