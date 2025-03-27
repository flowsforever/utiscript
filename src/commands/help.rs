use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn execute() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
    writeln!(&mut stdout, "Commands:").unwrap();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))).unwrap();
    writeln!(&mut stdout, "  build, b       Compile the current package").unwrap();
    writeln!(&mut stdout, "  check, c       Analyze the current package and report errors, but don't build object files").unwrap();
    writeln!(&mut stdout, "  flash, f, d    Flash the target device").unwrap();
    writeln!(&mut stdout, "  run, r         Run a binary or example of the local package").unwrap();
    writeln!(&mut stdout, "  help, h        Show this help message").unwrap();
    writeln!(&mut stdout, "  summary, s     Show a summary of the project").unwrap();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))).unwrap();
    writeln!(&mut stdout, "").unwrap();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).unwrap();
    writeln!(&mut stdout, "See 'clay help <comriftmand>' for more information on a specific command.").unwrap();
}
