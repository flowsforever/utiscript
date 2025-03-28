use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn execute() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
    writeln!(&mut stdout, "Usage: ").unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))).unwrap();
    writeln!(&mut stdout, "  clay [COMMAND]").unwrap();

    writeln!(&mut stdout).unwrap();

    let commands: &[(&str, &str)] = &[
        ("build, b", "Compile the current project"),
        ("check, c", "Analyze the current project and report errors"),
        ("flash, f, d", "Flash(Download) to the target device"),
        ("run, r", "Run a binary or example on the target device"),
        ("help, h", "Show this help message"),
        ("summary, s", "Show a summary of the project"),
    ];

    let max_cmd_len = commands.iter().map(|(cmd, _)| cmd.len()).max().unwrap_or(0);

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
    writeln!(&mut stdout, "Commands:").unwrap();

    for (cmd, desc) in commands {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))).unwrap();
        write!(&mut stdout, "  {}", cmd).unwrap();
        let padding = max_cmd_len - cmd.len() + 2;
        write!(&mut stdout, "{: <width$}", "", width = padding).unwrap();
        stdout.reset().unwrap();
        writeln!(&mut stdout, "{}", desc).unwrap();
    }

    writeln!(&mut stdout).unwrap();

    stdout.reset().unwrap();
    writeln!(&mut stdout, "See 'clay help <command>' for more information on a specific command.").unwrap();
}