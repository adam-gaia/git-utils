use commandstream::CommandStream;
use color_eyre::Result;

struct Command<'a> {
   command: &'a [String]
}

impl<'a> Command<'a> {
    fn new(command: &'a [String]) -> Self {
        Command {
            command
        }
    }
}

impl<'a> CommandStream<'_> for Command<'a> {
    fn command(&self) -> &[String] {
        &self.command
    }

    fn handle_stdout(&self, line: &str) -> Result<()> {
        println!("[STDOUT] {}", line);
        Ok(())
    }

    fn handle_stderr(&self, line: &str) -> Result<()> {
        eprintln!("[STDERR] {}", line);
        Ok(())
    }
}

pub async fn run(command: &str, args: &[&str]) -> Result<i32> {
    let num_args = args.len();
    let mut cmd = Vec::with_capacity(num_args + 1);
    cmd.push(command.to_string());
    for arg in args {
        cmd.push(arg.to_string());
    }
    let command = Command::new(&cmd);
    let return_code = command.run().await?;
    Ok(return_code)
}
