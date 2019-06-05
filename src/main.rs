use structopt::StructOpt;
use std::fs::File;
use log::{info};

// super odd that you don't need to explicitly use one of these deps
// for example I don't reference Write but it is required to use `writeln!`
// method
use std::io::{BufReader, BufRead, Result, BufWriter, self, Write};

// I have no idea what I'm doing but this is meant to find lines in a file
#[derive(StructOpt)]
struct Cli {
	// pattern to look for
	pattern: String,
	// path to file
	#[structopt(parse(from_os_str))]
	path: std::path::PathBuf,
}

fn main() -> Result<()>  {
	env_logger::init();
	info!("Starting Up!");
    let args = Cli::from_args();

	info!("Starting reading file {:?}", args.path);
    let file = File::open(&args.path)?;
	info!("Finished reading file {:?}", &args.path);

    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout);

    for line in BufReader::new(file).lines() {
    	let this_line: String = line.unwrap();
		info!("Comparing line {:?}", this_line);
    	if this_line.contains(&args.pattern) {
			info!("Line passed check");
    		writeln!(handle, "{}", this_line)?;
    	} else {
    		info!("Line failed check")
    	}
    }

    Ok(())
}
