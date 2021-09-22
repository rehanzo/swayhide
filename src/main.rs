use std::env::args;
use std::process::Command;
use std::process::exit;
use swayipc::{Connection, Error, EventType};

const USAGE: &str = r#"
A window swallower for sway

USAGE:
    swayhide [OPTIONS] [COMMAND]

OPTIONS:
    -h  --help  Show this help page

COMMAND:
    Can be either a string, or a command with zero or more arguments

EXAMPLES:
    swayhide firefox
    swayhide zathura document.pdf
    swayhide "imv image.jpg"

swayhide was written by Nomisiv <simon@nomisiv.com>
Report bugs to https://github.com/NomisIV/swayhide/issues
"#;

fn show_help() -> Result<(), Error> {
    Ok(println!("{}", USAGE))
}

fn parse_child_cmd(child_cmd: Vec<String>) -> Command {
    let mut child_cmd = child_cmd;

    // If first item is string
    let first_item = child_cmd.get(0).unwrap().as_str();
    if first_item.contains(&" ") {
        child_cmd = first_item.split(" ").map(|arg| arg.to_owned()).collect();
    }

    let mut cmd = Command::new(child_cmd.get(0).unwrap());
    if child_cmd.len() > 1 {
        cmd.args(child_cmd.get(1..).unwrap());
    }
    return cmd;
}

fn hide(args: Vec<String>) -> Result<(), swayipc::Error> {
    // Prepare command
    let child_process_name = args.get(1).unwrap(); // This can be replaced with child_process.get_program() when https://github.com/rust-lang/rust/issues/44434 is merged into stable
    let mut child_process = parse_child_cmd(args.get(1..).expect("No process to start").into());

    // Set up connection
    let mut con = Connection::new()?;

    // Get pid from focused window
    let pid: i32 = con
        .get_tree()?
        .find_focused(|node| node.focused == true)
        .expect("No focused node")
        .pid
        .unwrap();

    let mark = format!("hidden-{}", pid);
    con.run_command(format!("mark {}", mark))?;
    con.run_command("split v")?;

    // Run command
    let mut child = child_process.spawn().map_err(|err| Error::from_boxed_compat(Box::new(err)))?;

    for _ in Connection::new()?.subscribe(&[EventType::Window])? {
        // FIXME: It's still possible for this window to be entirely unrelated.
        break;
    }

    // Focus our marked window and hide it.
    con.run_command(format!("[con_mark=\"{}\"] focus; move scratchpad", mark))?;

    // Wait for command to exit
    let status = child.wait().map_err(|err| Error::from_boxed_compat(Box::new(err)))?.code();

    // Move the hidden window back (and disable floating because idk)
    con.run_command(format!(
        "[pid={}] focus; move mark {}; floating disable",
        pid, mark
    ))?;

    // Print child command status
    match status {
        Some(code) => {
            if code > 0 {
                eprintln!("swayhide: {} exited with code: {}", child_process_name, code);
            }
        }
        None => eprintln!("swayhide: {} was terminated by signal", child_process_name),
    }

    Ok(())
}

fn main() {
    // Make sure a command is provided, or print usage guide otherwise
    let args: Vec<String> = args().collect();
    let result = match args.get(1).map(|a| a.as_str()) {
        None => show_help(),
        Some("-h") => show_help(),
        Some("--help") => show_help(),
        Some(_) => hide(args)
    };
    let exit_code = match result {
        Ok(_) => exitcode::OK,
        Err(e) => {
            println!("{}", e);
            exitcode::USAGE
        }
    };
    exit(exit_code);
}
