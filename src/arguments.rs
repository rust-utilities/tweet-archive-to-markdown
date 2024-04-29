#!/usr/bin/env rust

use clap::{CommandFactory, Parser};
use clap_complete::{Generator, Shell};

#[derive(Parser, Debug)]
#[clap(author, version)]
#[clap(about, verbatim_doc_comment)]
#[clap(arg_required_else_help = true)]
/// CLI application arguments for tweet-archive-to-markdown
///
/// Convert archive of Twitter/X Tweets/Posts into MarkDown files, with FrontMatter, compatible
/// with Jekyll and similar static site generators
///
/// ## Users may wish to review
///
/// - https://jekyllrb.com/docs/front-matter/
///
/// ## Developers may wish to review
///
/// - https://github.com/clap-rs/clap/blob/v3.0.14/examples/derive_ref/README.md#arg-types
/// - https://github.com/clap-rs/clap/issues/3198
///
/// ## Example usage
///
/// ```bash
/// GH_NAME="S0AndS0"
///
/// tweet-archive-to-markdown --input-path "~/Downloads/twitter.archive.zip" \
///   --output-directory "~/git/hub/${GH_NAME}.github.io/_tweets" \
///   --post-date-format "%Y-%m-%d %H:%M:%S %z" \
///   --post-author "${GH_NAME}" \
///   --post-layout "post" \
///   --post-twitter-key "twitter" \
///   --verbose \
///   --dry-run
/// ```
///
/// ## Example result
///
/// - File path: `~/git/hub/S0AndS0.github.io/_tweets/2023-08-30-1697011324369178968.md`
///
/// ```markdown
/// ---
/// layout: post
/// date: "2023-08-30 22:20:03 +0000"
/// author: S0AndS0
/// twitter:
///   post: https://twitter.com/i/web/status/1697011324369178968
/// ---
/// 
/// Anyone else occasionally annoyed we're not allowed to modify visibility of `details` HTML element children via `:target` ID with CSS only?
/// 
/// details:is(:target) &gt; *:not(summary) {
///   background: hotpink;
///   display: block;
///   visibility: visible;
/// }
/// ```
pub struct Args {
	/// Path to input file
	///
	/// ## Example input zip file path
	///
	/// ```
	/// tweet-archive-to-markdown --input-path "./twitter.archive.zip"
	/// ```
	///
	/// [possible file extensions: zip, js, json]
	///
	/// > zip -- attempt to read `./twitter.archive.zip::data/manifest.js` and parse all relative
	/// > paths to `data/tweets.js` into JSON, then writes MarkDown files
	/// >
	/// > js -- use `--javascript-pattern` value to convert JavaScript into JSON, then will attempt
	/// > to write MarkDown files
	/// >
	/// > json -- assumes you have already converted JavaScript into JSON, and will attempt to
	/// > write MarkDown files from parsed JSON data
	/// >
	/// > Directory -- assumes archive was unpacked/extracted and `data/manifest.js` file, and
	/// > files it points to may be found under `--input-path` value
	#[arg(long, verbatim_doc_comment, value_hint = clap::ValueHint::FilePath)]
	pub input_path: String,

	/// Path to directory where MarkDown files will be written
	///
	/// ## Example
	///
	/// ```
	/// tweet-archive-to-markdown --output-directory "./_posts/tweets"
	/// ```
	///
	/// [default: current working directory]
	#[arg(long, verbatim_doc_comment, value_hint = clap::ValueHint::DirPath, required = false)]
	pub output_directory: Option<String>,

	/// Useful if/when `--input-path` targets an explicit `data/tweets.js` file path
	///
	/// ## Example
	///
	/// ```
	/// tweet-archive-to-markdown --javascript-pattern "window.YTD.tweets.part0"
	/// ```
	#[arg(
		long,
		verbatim_doc_comment,
		required = false,
		default_value = "window.YTD.tweets.part0"
	)]
	#[clap(value_enum)]
	pub javascript_pattern: String,

	/// Custom format string for `DateTime` output parsed from `.tweets[].tweet.created_at` used to
	/// generate post FrontMatter for `date` YAML
	///
	/// ## Example
	///
	/// ```
	/// tweet-archive-to-markdown --post-date-format "%Y-%m-%d %H:%M:%S %z"
	/// ```
	#[arg(
		long,
		verbatim_doc_comment,
		required = false,
		default_value = "%F %T %z"
	)]
	#[clap(value_enum)]
	pub post_date_format: String,

	/// Post `author` FrontMatter value for all MarkDown files written
	///
	/// ## Example
	///
	/// ```
	/// tweet-archive-to-markdown --post-author "S0AndS0"
	/// ```
	#[arg(long, verbatim_doc_comment, required = false)]
	#[clap(value_enum)]
	pub post_author: Option<String>,

	/// Post `layout` FrontMatter value for all MarkDown files written
	///
	/// ## Example
	///
	/// ```
	/// tweet-archive-to-markdown --post-layout "post"
	/// ```
	#[arg(long, verbatim_doc_comment, required = false, default_value = "post")]
	#[clap(value_enum)]
	pub post_layout: String,

	/// FrontMatter key under which extra Twitter Post metadata and links will be provided
	///
	/// ## Example
	///
	/// ```
	/// tweet-archive-to-markdown --post-twitter-key "twitter"
	/// ```
	#[arg(
		long,
		verbatim_doc_comment,
		required = false,
		default_value = "twitter"
	)]
	#[clap(value_enum)]
	pub post_twitter_key: String,

	/// Output shell completions to standard out then exit
	///
	/// ## Example
	///
	/// ```
	/// tweet-archive-to-markdown --build-completions bash
	/// ```
	#[arg(long, verbatim_doc_comment, required = false)]
	#[clap(value_enum)]
	pub build_completions: Option<Shell>,

	/// Send data to standard out without writing files
	///
	/// ## Example
	///
	/// ```
	/// tweet-archive-to-markdown --dry-run
	/// ```
	#[arg(long, verbatim_doc_comment, required = false)]
	pub dry_run: bool,

	/// Send parsing data and debugging information to standard error
	///
	/// ## Example
	///
	/// ```
	/// tweet-archive-to-markdown --verbose
	/// ```
	#[arg(long, verbatim_doc_comment, required = false)]
	pub verbose: bool,
}

/// Display tab-completion configuration for given shell
///
/// ## Resources for further reading
///
/// - https://github.com/clap-rs/clap/blob/master/clap_complete/examples/completion-derive.rs
/// - https://github.com/clap-rs/clap/discussions/3671
/// - https://github.com/clap-rs/clap/discussions/2417
pub fn print_completions<G: Generator>(shell: G) {
	let mut cmd = Args::command();
	let name = cmd.get_name().to_string();
	clap_complete::generate(shell, &mut cmd, &name, &mut std::io::stdout());
}
