#!/usr/bin/env rust

use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use regex::Regex;
use std::fs;
use std::io::{Read, Result};
use zip::read::ZipArchive;

use twitter_archive::structs::tweets;

#[derive(Parser, Debug)]
#[clap(author, version)]
#[clap(about, verbatim_doc_comment)]
#[clap(arg_required_else_help = true)]
/// CLI application arguments for search-tweets
///
/// ## Developers may wish to review
///
/// - https://github.com/clap-rs/clap/blob/v3.0.14/examples/derive_ref/README.md#arg-types
/// - https://github.com/clap-rs/clap/issues/3198
struct Args {
	/// Path to input file
	///
	/// ## Example print Twitter archived direct messages
	///
	/// ```
	/// cargo run --example search-tweets -- \
	///   --input-file "~/Downloads/twitter-archive.zip"
	/// ```
	#[arg(long, verbatim_doc_comment, value_hint = clap::ValueHint::FilePath)]
	pub input_file: Option<String>,

	/// Regular expression to use for searching within Twitter direct messages
	///
	/// ## Example
	///
	/// ```
	/// cargo run --example search-tweets -- \
	///   --input-file "~/Downloads/twitter-archive.zip" \
	///   --expression "^Hi"
	/// ```
	#[arg(long, verbatim_doc_comment)]
	pub expression: Option<String>,

	/// Attempt to output shell completions
	///
	/// ## Example
	///
	/// ```
	/// cargo run --example search-tweets -- \
	///   --build-completions bash
	/// ```
	#[arg(long, verbatim_doc_comment, required = false)]
	#[clap(value_enum)]
	pub build_completions: Option<Shell>,
}

///
fn main() -> Result<()> {
	let args = Args::parse();

	// Display tab-completion configuration for given shell then exit
	if let Some(shell) = args.build_completions {
		// ## Resources for further reading
		//
		// - https://github.com/clap-rs/clap/blob/master/clap_complete/examples/completion-derive.rs
		// - https://github.com/clap-rs/clap/discussions/3671
		// - https://github.com/clap-rs/clap/discussions/2417
		println!("#!/usr/bin/env {}", shell.to_string().to_lowercase());
		let mut cmd = Args::command();
		let name = cmd.get_name().to_string();
		clap_complete::generate(shell, &mut cmd, &name, &mut std::io::stdout());
		std::process::exit(0);
	}

	// Ensure required CLI values are present
	let expression = args.expression.expect("Undefined value for: --expression");
	let input_file = args.input_file.expect("Undefined value for: --input-file");

	// Read "data/direct_messages.js" file from Zip archive into String buffer
	let file_descriptor = fs::File::open(input_file)?;
	let mut zip_archive = ZipArchive::new(file_descriptor)?;
	let mut zip_file = zip_archive.by_name("data/tweets.js")?;
	let mut buff = String::new();
	zip_file.read_to_string(&mut buff)?;

	// Clear JavaScript prefix and parse remaining text as JSON
	let javascript = "window.YTD.tweets.part0 = ";
	let json = buff.replacen(javascript, "", 1);
	let data: Vec<tweets::TweetObject> = serde_json::from_str(&json).unwrap();

	// Do the search for Regex pattern and print matches thing!
	let re = Regex::new(&expression).expect("Invalid regular expression: --expression");
	for (index, object) in data.iter().enumerate() {
		let tweet = &object.tweet;
		let Some(_caps) = re.captures(&tweet.full_text) else { continue };

		println!("Index: {index}");
		println!("Created at: {}", tweet.created_at);
		println!("vvv Content\n{}\n^^^ Content", tweet.full_text);
	}

	Ok(())
}
