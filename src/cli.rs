/// FORTYTWO-LANG COMPILER
#[derive(clap::Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
	#[clap(subcommand)]
	pub command: Command,
}

#[derive(clap::Parser, Debug)]
pub enum Command {
	/// Format the code.
	Fmt {
		/// The file to format. Note that this file will be overwritten.
		#[clap(parse(from_os_str))]
		file: std::path::PathBuf,
	},

	/// Compile to an executable.
	Compile {
		/// The file to compile.
		#[clap(parse(from_os_str))]
		file: std::path::PathBuf,
	},

	/// Compile and execute.
	Run {
		/// The file to run.
		#[clap(parse(from_os_str))]
		file: std::path::PathBuf,
	},
}
