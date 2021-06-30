use std::env::args;
use std::process::Command;
use swayipc::{Connection, Error};

fn main() -> Result<(), Error> {
    // Prepare command
    let args: Vec<String> = args().collect();
    let child_process_name = args.get(1).expect("No process to start");
    let mut child_process = Command::new(child_process_name);
    if args.len() > 1 {
        child_process.args(args.get(2..).unwrap());
    }

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

    // Mark window and move to scratchpad
    con.run_command(format!("mark {}; move scratchpad", mark))?;

    // Run command
    let status = child_process.status();

    // Move the hidden window back (and disable floating because idk)
    con.run_command(format!(
        "[pid={}] focus; move mark {}; floating disable",
        pid, mark
    ))?;

    // Print child command status
    match status {
        Ok(output) => match output.code() {
            Some(code) => {
                if code > 0 {
                    println!("{} exited with code: {}", child_process_name, code)
                }
            }
            None => println!("{} was terminated by signal", child_process_name),
        },
        Err(e) => println!("Failed to start {}: {}", child_process_name, e),
    }

    Ok(())
}
