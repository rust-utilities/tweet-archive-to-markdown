#!/usr/bin/env rust

use crate::arguments::Args;
use twitter_archive::structs::tweets::{Tweet, TweetEntitiesUserMention};

/// Combine results of `front_matter` and `content` functions
pub fn post(tweet: &Tweet, args: &Args) -> String {
	let front_matter = format!("---\n{}\n---", front_matter(tweet, args));

	let content = content(tweet, args);

	if args.verbose {
		eprintln!(
			"post_build::markdown::front_matter vvv\n{}\nmain::front_matter ^^^",
			front_matter
		);

		eprintln!(
			"post_build::markdown::content vvv\n{}\nmain::content ^^^",
			content
		);
	}

	format!("{}\n\n{}", front_matter, content)
}

/// FrontMatter from Twitter archive Tweets
///
/// ## Example output YAML (FrontMatter)
///
/// ```yaml
/// layout: post
/// date: "2021-01-05 20:17:27 +0000"
/// author: S0AndS0
/// tags: [JavaScript]
/// twitter:
///   post: https://twitter.com/i/web/status/1346551404433977352
///   reply_to:
///     account: https://twitter.com/akash_webdev
///     post: https://twitter.com/i/web/status/1346453666148990976
///   mentions:
///     - https://twitter.com/l422y
///   urls:
///     - https://developer.mozilla.org/en-US/docs/Web/API/History_API
/// ```
///
/// ## Example input JSON
///
/// ```json
/// [
///   {
///     "tweet": {
///       "edit_info": {
///         "initial": {
///           "editTweetIds": ["1346551404433977352"],
///           "editableUntil": "2021-01-05T21:17:27.189Z",
///           "editsRemaining": "5",
///           "isEditEligible": true
///         }
///       },
///       "retweeted": false,
///       "source": "<a href=\"https://mobile.twitter.com\" rel=\"nofollow\">Twitter Web App</a>",
///       "entities": {
///         "hashtags": [
///           {
///             "text": "JavaScript",
///             "indices": ["60", "71"]
///           }
///         ],
///         "symbols": [],
///         "user_mentions": [
///           {
///             "name": "L422Y",
///             "screen_name": "l422y",
///             "indices": ["13", "19"],
///             "id_str": "15904411",
///             "id": "15904411"
///           }
///         ],
///         "urls": [
///           {
///             "url": "https://t.co/7I1G7nykqo",
///             "expanded_url": "https://developer.mozilla.org/en-US/docs/Web/API/History_API",
///             "display_url": "developer.mozilla.org/en-US/docs/Web…",
///             "indices": ["113", "136"]
///           }
///         ]
///       },
///       "display_text_range": ["0", "292"],
///       "favorite_count": "0",
///       "in_reply_to_status_id_str": "1346453666148990976",
///       "id_str": "1346551404433977352",
///       "in_reply_to_user_id": "4602318258",
///       "truncated": false,
///       "retweet_count": "1",
///       "id": "1346551404433977352",
///       "in_reply_to_status_id": "1346453666148990976",
///       "possibly_sensitive": false,
///       "created_at": "Tue Jan 05 20:17:27 +0000 2021",
///       "favorited": false,
///       "full_text": "@__akash__19 @L422Y It may be possible to edit history from #JavaScript, check MDN documentation for details...\n\nhttps://t.co/7I1G7nykqo\n\n... TLDR, here's an untested example...\n\nfunction replaceHash(text) {\n  location.replace(text);\n  history.replaceState(undefined, document.title, text);\n}",
///       "lang": "en",
///       "in_reply_to_screen_name": "akash_webdev",
///       "in_reply_to_user_id_str": "4602318258"
///     }
///   }
/// ]
/// ```
pub fn front_matter(tweet: &Tweet, args: &Args) -> String {
	let mut lines: Vec<String> = vec![];

	lines.push(format!("layout: {}", args.post_layout));
	lines.push(front_matter_date(tweet, args));

	if let Some(author) = &args.post_author {
		lines.push(format!("author: {}", author));
	}

	let tags = front_matter_tags(tweet, args);
	if !tags.is_empty() {
		lines.push(format!("tags: [{}]", tags));
	}

	lines.push(front_matter_links(tweet, args));

	// Put it all together and ship it!
	let result = lines.join("\n");
	if args.verbose {
		eprintln!("post_build::front_matter -> {result}");
	}

	result
}

/// Build output MarkDown file name from Twitter metadata
pub fn file_name(tweet: &Tweet, args: &Args) -> String {
	let post_date = tweet.created_at.format("%F").to_string();

	let file_name = format!("{post_date}-{}.md", tweet.id_str);
	if args.verbose {
		eprintln!("post_build::file_name -> {file_name}");
	}

	file_name
}

/// Build mostly MarkDown compatible string from `.tweets[].tweet.full_text` and attempt to inject
/// links to mentioned users as well as any shared links by parsing;
///
/// - `.tweets[].tweet.entries.user_mentions[]`
/// - `.tweets[].tweet.entries.urls[]`
///
/// ## Example output MarkDown
///
/// ```markdown
/// @__akash__19 [@L422Y](https://twitter.com/l422y) It may be possible to edit history from #JavaScript, check MDN documentation for details...
///
/// [https://t.co/7I1G7nykqo](https://developer.mozilla.org/en-US/docs/Web/API/History_API)
///
/// ... TLDR, here's an untested example...
///
/// function replaceHash(text) {
///   location.replace(text);
///   history.replaceState(undefined, document.title, text);
/// }
/// ```
///
/// ## Example input JSON
///
/// ```json
/// [
///   {
///     "tweet": {
///       "edit_info": {
///         "initial": {
///           "editTweetIds": ["1346551404433977352"],
///           "editableUntil": "2021-01-05T21:17:27.189Z",
///           "editsRemaining": "5",
///           "isEditEligible": true
///         }
///       },
///       "retweeted": false,
///       "source": "<a href=\"https://mobile.twitter.com\" rel=\"nofollow\">Twitter Web App</a>",
///       "entities": {
///         "hashtags": [
///           {
///             "text": "JavaScript",
///             "indices": ["60", "71"]
///           }
///         ],
///         "symbols": [],
///         "user_mentions": [
///           {
///             "name": "L422Y",
///             "screen_name": "l422y",
///             "indices": ["13", "19"],
///             "id_str": "15904411",
///             "id": "15904411"
///           }
///         ],
///         "urls": [
///           {
///             "url": "https://t.co/7I1G7nykqo",
///             "expanded_url": "https://developer.mozilla.org/en-US/docs/Web/API/History_API",
///             "display_url": "developer.mozilla.org/en-US/docs/Web…",
///             "indices": ["113", "136"]
///           }
///         ]
///       },
///       "display_text_range": ["0", "292"],
///       "favorite_count": "0",
///       "in_reply_to_status_id_str": "1346453666148990976",
///       "id_str": "1346551404433977352",
///       "in_reply_to_user_id": "4602318258",
///       "truncated": false,
///       "retweet_count": "1",
///       "id": "1346551404433977352",
///       "in_reply_to_status_id": "1346453666148990976",
///       "possibly_sensitive": false,
///       "created_at": "Tue Jan 05 20:17:27 +0000 2021",
///       "favorited": false,
///       "full_text": "@__akash__19 @L422Y It may be possible to edit history from #JavaScript, check MDN documentation for details...\n\nhttps://t.co/7I1G7nykqo\n\n... TLDR, here's an untested example...\n\nfunction replaceHash(text) {\n  location.replace(text);\n  history.replaceState(undefined, document.title, text);\n}",
///       "lang": "en",
///       "in_reply_to_screen_name": "akash_webdev",
///       "in_reply_to_user_id_str": "4602318258"
///     }
///   }
/// ]
/// ```
pub fn content(tweet: &Tweet, args: &Args) -> String {
	if tweet.entities.user_mentions.is_empty() {
		return tweet.full_text.clone();
	}

	let mut user_mentions = tweet.entities.user_mentions.clone();
	let mut urls = tweet.entities.urls.clone();
	let mut characters: Vec<String> = vec![];
	for (index, character) in tweet.full_text.clone().chars().enumerate() {
		if user_mentions.is_empty() && urls.is_empty() {
			characters.push(character.to_string());
			continue;
		}

		// Yes these two loops be kinda ugly, but it works well enough
		for user_mention in user_mentions.clone() {
			if user_mention.indices[0] == index {
				characters.push("[".into());
			} else if user_mention.indices[1] == index {
				characters.push("]".into());

				let link = format!("({})", twitter_url_account(&user_mention.screen_name, args));
				link.chars().for_each(|c| characters.push(c.into()));

				// Reduce the size of `user_mentions` for next pass of outer for loop
				user_mentions.rotate_left(1);
				user_mentions.pop();
			}
		}

		for url in urls.clone() {
			if url.indices[0] == index {
				characters.push("[".into());
			} else if url.indices[1] == index {
				characters.push("]".into());

				let link = format!("({})", url.expanded_url);
				link.chars().for_each(|c| characters.push(c.into()));

				// Reduce the size of `urls` for next pass of outer for loop
				urls.rotate_left(1);
				urls.pop();
			}
		}

		characters.push(character.to_string());
	}

	characters.join("")
}

/// Re-format Tweet `created_at` with CLI provided format string
fn front_matter_date(tweet: &Tweet, args: &Args) -> String {
	let post_date_format = args.post_date_format.clone().as_str().to_owned();

	format!(
		"date: {:?}",
		tweet.created_at.format(&post_date_format).to_string()
	)
}

/// Convert Tweeted hashtags into YAML compatible list of strings
fn front_matter_tags(tweet: &Tweet, _args: &Args) -> String {
	tweet
		.entities
		.hashtags
		.iter()
		.map(|hashtag| hashtag.text.clone())
		.collect::<Vec<String>>()
		.join(",")
}

/// Extract and/or re-format various links from Tweet into YAML compatible syntax
fn front_matter_links(tweet: &Tweet, args: &Args) -> String {
	let mut lines: Vec<String> = vec![];

	lines.push(format!("{}:", args.post_twitter_key));

	lines.push(format!(
		"  post: {}",
		twitter_url_status(&tweet.id_str, args)
	));

	// Differentiate between replies, with link to previous Tweet, and initial mentions
	if tweet.in_reply_to_screen_name.is_some() && tweet.in_reply_to_status_id_str.is_some() {
		let in_reply_to_screen_name = tweet.in_reply_to_screen_name.clone().unwrap();
		let in_reply_to_status_id_str = tweet.in_reply_to_status_id_str.clone().unwrap();

		lines.push("  reply_to:".to_string());

		lines.push(format!(
			"    account: {}",
			twitter_url_account(&in_reply_to_screen_name, args)
		));

		lines.push(format!(
			"    post: {}",
			twitter_url_status(&in_reply_to_status_id_str, args)
		));

		let user_mentions = tweet
			.entities
			.user_mentions
			.iter()
			.filter(|user_mention| user_mention.screen_name != in_reply_to_screen_name)
			.cloned()
			.collect::<Vec<TweetEntitiesUserMention>>();

		if !user_mentions.is_empty() {
			lines.push(front_matter_mentions(&user_mentions, args));
		}
	} else if !tweet.entities.user_mentions.is_empty() {
		lines.push(front_matter_mentions(&tweet.entities.user_mentions, args));
	}

	let urls = tweet
		.entities
		.urls
		.iter()
		.map(|url| url.expanded_url.clone())
		.collect::<Vec<String>>();

	if !urls.is_empty() {
		lines.push("  urls:".into());
		urls.iter().for_each(|url| {
			lines.push(format!("    - {}", url));
		});
	}

	lines.join("\n")
}

/// Assumes caller has already checked if `user_mentions.len()` is greater than `0`
fn front_matter_mentions(user_mentions: &Vec<TweetEntitiesUserMention>, args: &Args) -> String {
	let mut lines: Vec<String> = vec![];

	lines.push("  mentions:".to_string());
	for mention in user_mentions {
		lines.push(format!(
			"    - {}",
			twitter_url_account(&mention.screen_name, args)
		));
	}

	lines.join("\n")
}

/// Prepend `.id_str` JSON value with full URL path
fn twitter_url_status(id_str: &str, _args: &Args) -> String {
	format!("https://twitter.com/i/web/status/{}", id_str)
}

/// Prepend `.screen_name` JSON value with full URL path
fn twitter_url_account(screen_name: &str, _args: &Args) -> String {
	format!("https://twitter.com/{}", screen_name)
}
