#!/usr/bin/env rust

//! Convert archived Tweets to MarkDown with FrontMater

#![forbid(unsafe_code)]
#![deny(clippy::all, missing_docs)]

mod arguments;
mod post_build;

use clap::CommandFactory;
use clap::Parser;
use std::io::{Read, Write};
use std::{env, fs, io, path};
use twitter_archive::structs::manifest::Manifest;
use twitter_archive::structs::tweets::TweetObject;
use zip::read::ZipArchive;

use arguments::Args;

/// Entry point for binary, this is where the magic starts and stops!
fn main() -> io::Result<()> {
	let args = Args::parse();

	// Print shell completions and exit if requested
	if let Some(shell) = args.build_completions {
		println!("#!/usr/bin/env {}", shell.to_string().to_lowercase());
		arguments::print_completions(shell);
		std::process::exit(0);
	}

	// Use current working directory if output path is undefined
	let output_directory = &args
		.output_directory
		.clone()
		.unwrap_or(env::current_dir()?.display().to_string());

	// Create output path if none exists
	let output_directory_path = path::Path::new(&output_directory);
	if !output_directory_path.is_dir() {
		if args.verbose || args.dry_run {
			eprintln!(
				"main -> create output_directory_path -> {}",
				output_directory_path.display()
			);
		} else {
			fs::create_dir_all(output_directory_path)?;
		}
	} else if args.verbose {
		eprintln!(
			"main -> output_directory_path -> {}",
			output_directory_path.display()
		);
	}

	let input_path = path::Path::new(&args.input_path);
	if input_path.is_file() {
		let extension = input_path
			.extension()
			.and_then(std::ffi::OsStr::to_str)
			.unwrap();

		match extension {
			"zip" => {
				let javascript_manfifest =
					read_zip_by_name_to_string(&args.input_path, "data/manifest.js", &args);

				let json_manifest = javascript_manfifest.replacen("window.__THAR_CONFIG = ", "", 1);

				let data_manifest: Manifest =
					serde_json::from_str(&json_manifest).expect("Unable to parse string as JSON");

				data_manifest
					.data_types
					.tweets
					.files
					.iter()
					.for_each(|data_manifest_tweets| {
						let file_name = &data_manifest_tweets.file_name;
						let pattern = format!("window.{} = ", data_manifest_tweets.global_name);

						let javascript_tweets =
							read_zip_by_name_to_string(&args.input_path, file_name, &args);

						let json_tweets = javascript_tweets.replacen(&pattern, "", 1);

						if args.verbose {
							eprintln!(
								"main is_file zip:\n  file_name: {file_name}\n  pattern: {pattern}"
							);
						}

						let data_tweets: Vec<TweetObject> =
							serde_json::from_str(&json_tweets).expect("Unable to parse as JSON");

						tweets_to_markdown(&data_tweets, output_directory_path, &args).unwrap();
					});
			}
			"js" => {
				if args.verbose {
					eprintln!("main is_file js:\n  input_path: {}", args.input_path);
				}

				let javascript_tweets =
					fs::read_to_string(input_path).expect("Unable to read --input-path");

				let json_tweets = javascript_tweets.replacen(&args.javascript_pattern, "", 1);

				let data_tweets: Vec<TweetObject> =
					serde_json::from_str(&json_tweets).expect("Unable to parse as JSON");

				tweets_to_markdown(&data_tweets, output_directory_path, &args).unwrap();
			}
			"json" => {
				if args.verbose {
					eprintln!("main is_file json:\n  input_path: {}", args.input_path);
				}

				let json_tweets =
					fs::read_to_string(&args.input_path).expect("Unable to read --input-path");

				let data_tweets: Vec<TweetObject> =
					serde_json::from_str(&json_tweets).expect("Unable to parse as JSON");

				tweets_to_markdown(&data_tweets, output_directory_path, &args).unwrap();
			}
			_ => {
				let mut cmd = Args::command();
				cmd.build();
				cmd.print_help().unwrap();
				eprintln!("main is_file _:\n  Unexpected file extension: {extension}");
				std::process::exit(1);
			}
		}
	} else if input_path.is_dir() {
		let path_manifest = input_path.join("data").join("manifest.js");

		if args.verbose {
			eprintln!("main is_dir:\n  path_manifest: {}", path_manifest.display());
		}

		let javascript_manfifest = fs::read_to_string(path_manifest.to_str().unwrap())
			.unwrap_or_else(|_| panic!("Unable to read manifest path: {}", path_manifest.display()));

		let json_manifest = javascript_manfifest.replacen("window.__THAR_CONFIG = ", "", 1);

		let data_manifest: Manifest = serde_json::from_str(&json_manifest)
			.unwrap_or_else(|_| panic!("Unable to parse manifest path: {}", path_manifest.display()));

		data_manifest
			.data_types
			.tweets
			.files
			.iter()
			.for_each(|data_manifest_tweets| {
				let file_name = &data_manifest_tweets.file_name;
				let pattern = format!("window.{} = ", data_manifest_tweets.global_name);

				let mut javascript_path = path::PathBuf::from(input_path);
				file_name.split('/').for_each(|p| {
					javascript_path.push(p);
				});

				if args.verbose {
					eprintln!("main is_dir:\n  file_name: {file_name}\n  pattern: {pattern}\n  javascript: {}", javascript_path.display());
				}

				let javascript_tweets = fs::read_to_string(&javascript_path).unwrap();

				let json_tweets = javascript_tweets.replacen(&pattern, "", 1);

				let data_tweets: Vec<TweetObject> =
					serde_json::from_str(&json_tweets).expect("Unable to parse as JSON");

				tweets_to_markdown(&data_tweets, output_directory_path, &args).unwrap();
			});
	} else {
		let mut cmd = Args::command();
		cmd.build();
		cmd.print_help().unwrap();
		eprintln!(
			"main -> Unexpected file extension or path type for --input-path -> {}",
			args.input_path
		);
		std::process::exit(1);
	}

	Ok(())
}

/// Create a file for each Tweet that does not yet have a corresponding MarkDown file
pub fn tweets_to_markdown(
	data_tweets: &[twitter_archive::structs::tweets::TweetObject],
	output_directory_path: &path::Path,
	args: &Args,
) -> io::Result<()> {
	for (index, object) in data_tweets.iter().enumerate() {
		if args.verbose {
			eprintln!("tweets_to_markdown -> Parsing Tweet index -> {index}");
		}

		let markdown_file_name = post_build::file_name(&object.tweet, args);
		let markdown_file_path = output_directory_path.join(markdown_file_name);
		if markdown_file_path.is_file() {
			eprintln!(
				"tweets_to_markdown -> Skipping existing file -> {}",
				markdown_file_path.display()
			);
			continue;
		}

		let post = post_build::post(&object.tweet, args);

		if args.dry_run {
			println!("{}", post);
		} else {
			let mut output = fs::File::create(&markdown_file_path)?;
			write!(output, "{}", post)?;
			if !args.verbose {
				eprintln!("Wrote file -> {}", markdown_file_path.display());
			}
		}
	}

	Ok(())
}

/// Load contents of `zip_path:file_name` into returned String
pub fn read_zip_by_name_to_string(zip_path: &str, file_name: &str, _args: &Args) -> String {
	let zip_file_descriptor = fs::File::open(zip_path).expect("Unable to read input_path");

	let mut buffer = String::new();
	ZipArchive::new(zip_file_descriptor)
		.unwrap()
		.by_name(file_name)
		.unwrap()
		.read_to_string(&mut buffer)
		.unwrap();

	buffer
}
