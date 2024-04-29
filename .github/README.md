# Tweet Archive To Markdown
[heading__top]:
  #tweet-archive-to-markdown
  "&#x2B06; Convert archived Tweets to MarkDown with FrontMatter"


Convert archived Tweets to MarkDown with FrontMatter

## [![Byte size of Tweet Archive To Markdown][badge__main__tweet_archive_to_markdown__source_code]][tweet_archive_to_markdown__main__source_code] [![Open Issues][badge__issues__tweet_archive_to_markdown]][issues__tweet_archive_to_markdown] [![Open Pull Requests][badge__pull_requests__tweet_archive_to_markdown]][pull_requests__tweet_archive_to_markdown] [![Latest commits][badge__commits__tweet_archive_to_markdown__main]][commits__tweet_archive_to_markdown__main] [![GitHub Actions Build Status][badge__github_actions]][activity_log__github_actions] [![License][badge__license]][branch__current__license]


---


- [:arrow_up: Top of Document][heading__top]
- [:building_construction: Requirements][heading__requirements]
- [:zap: Quick Start][heading__quick_start]
- [&#x1F9F0; Usage][heading__usage]
- [&#x1F5D2; Notes][heading__notes]
- [:chart_with_upwards_trend: Contributing][heading__contributing]
  - [:trident: Forking][heading__forking]
  - [:currency_exchange: Sponsor][heading__sponsor]
- [:card_index: Attribution][heading__attribution]
- [:balance_scale: Licensing][heading__license]


---



## Requirements
[heading__requirements]:
  #requirements
  "&#x1F3D7; Prerequisites and/or dependencies that this project needs to function properly"


This repository requires [Rust][rust_home] language/compiler to build from
source.  As of last update to this ReadMe file, the recommended method of
installing Rust is via their installer script;

```Bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```



______


## Quick Start
[heading__quick_start]:
  #quick-start
  "&#9889; Perhaps as easy as one, 2.0,..."


This repository is a Rust binary

- Install via `cargo`

   ```bash
   cargo install tweet-archive-to-markdown
   ```

- Write tab completions for preferred shell (if supported), Bash with XDG
  support example;

   ```bash
   _shell_name="bash"

   _completions_directory="${XDG_DATA_HOME:-${HOME}/.local/share}/bash-completion/completions"

   tweet-archive-to-markdown --build-completions "${_shell_name}" |
     tee "${_completions_directory}/tweet-archive-to-markdown" 1>/dev/null
   ```

   > possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`


______


## Usage
[heading__usage]:
  #usage
  "&#x1F9F0; How to utilize this repository"


- Print all available command-line options

   ```bash
   tweet-archive-to-markdown -h
   #> Convert archived Tweets to MarkDown with FrontMater
   #>
   #> Usage: tweet-archive-to-markdown [OPTIONS] --input-path <INPUT_PATH>
   #>
   #> Options:
   #>       --input-path <INPUT_PATH>
   #>           Path to input file
   #>       --output-directory <OUTPUT_DIRECTORY>
   #>           Path to directory where MarkDown files will be written
   #>       --javascript-pattern <JAVASCRIPT_PATTERN>
   #>           Useful if/when `--input-path` targets an explicit `data/tweets.js` file path [default: window.YTD.tweets.part0]
   #>       --post-date-format <POST_DATE_FORMAT>
   #>           Custom format string for `DateTime` output parsed from `.tweets[].tweet.created_at` used to
   #>           generate post FrontMatter for `date` YAML [default: "%F %T %z"]
   #>       --post-author <POST_AUTHOR>
   #>           Post `author` FrontMatter value for all MarkDown files written
   #>       --post-layout <POST_LAYOUT>
   #>           Post `layout` FrontMatter value for all MarkDown files written [default: post]
   #>       --post-twitter-key <POST_TWITTER_KEY>
   #>           FrontMatter key under which extra Twitter Post metadata and links will be provided [default: twitter]
   #>       --build-completions <BUILD_COMPLETIONS>
   #>           Output shell completions to standard out then exit [possible values: bash, elvish, fish, powershell, zsh]
   #>       --dry-run
   #>           Send data to standard out without writing files
   #>       --verbose
   #>           Send parsing data and debugging information to standard error
   #>   -h, --help
   #>           Print help (see more with '--help')
   #>   -V, --version
   #>           Print version
   ```

- Example usage

   ```bash
   GH_NAME="S0AndS0"

   tweet-archive-to-markdown --input-path "~/Downloads/twitter.archive.zip" \
     --output-directory "~/git/hub/${GH_NAME}.github.io/_tweets" \
     --post-date-format "%Y-%m-%d %H:%M:%S %z" \
     --post-author "${GH_NAME}" \
     --post-layout "post" \
     --post-twitter-key "twitter" \
     --verbose \
     --dry-run
   ```

- Example result -- `~/git/hub/S0AndS0.github.io/_tweets/2023-08-30-1697011324369178968.md`

   ```markdown
   ---
   layout: post
   date: "2023-08-30 22:20:03 +0000"
   author: S0AndS0
   twitter:
     post: https://twitter.com/i/web/status/1697011324369178968
   ---

   Anyone else occasionally annoyed we're not allowed to modify visibility of `details` HTML element children via `:target` ID with CSS only?

   details:is(:target) &gt; *:not(summary) {
     background: hotpink;
     display: block;
     visibility: visible;
   }
   ```


______


## Notes
[heading__notes]:
  #notes
  "&#x1F5D2; Additional things to keep in mind when developing"


This repository may not be feature complete and/or fully functional, Pull
Requests that add features or fix bugs are certainly welcomed.


______


## Contributing
[heading__contributing]:
  #contributing
  "&#x1F4C8; Options for contributing to tweet-archive-to-markdown and rust-utilities"


Options for contributing to tweet-archive-to-markdown and rust-utilities


---


### Forking
[heading__forking]:
  #forking
  "&#x1F531; Tips for forking tweet-archive-to-markdown"


> :warning: Creating fork(s), submitting contribution(s), publishing derivative
> work(s), etc. based on this repository will form an agreement to be bound by
> the use-cased based [licensing][heading__license] sub-sections.
>
> I.E. if you choose to contribute to or use this project, you acknowledge and
> accept these usage based licensing terms will apply to any such works too.

Start making a [Fork][tweet_archive_to_markdown__fork_it] of this repository to
an account that you have write permissions for.

- Add remote for fork URL. The URL syntax is
  _`git@github.com:<NAME>/<REPO>.git`_...

```Bash
cd ~/git/hub/rust-utilities/tweet-archive-to-markdown

git remote add fork git@github.com:<NAME>/tweet-archive-to-markdown.git
```


- Commit your changes and push to your fork, eg. to fix an issue...

```Bash
cd ~/git/hub/rust-utilities/tweet-archive-to-markdown


git commit -F- <<'EOF'
:bug: Fixes #42 Issue


**Edits**


- `<SCRIPT-NAME>` script, fixes some bug reported in issue
EOF


git push fork main
```

> Note, the `-u` option may be used to set `fork` as the default remote, eg.
> _`git push -u fork main`_ however, this will also default the `fork` remote
> for pulling from too! Meaning that pulling updates from `origin` must be done
> explicitly, eg. _`git pull origin main`_


- Then on GitHub submit a Pull Request through the Web-UI, the URL syntax is
  _`https://github.com/<NAME>/<REPO>/pull/new/<BRANCH>`_

> Note; to decrease the chances of your Pull Request needing modifications
> before being accepted, please check the
> [dot-github](https://github.com/rust-utilities/.github) repository for
> detailed contributing guidelines.


---


### Sponsor
  [heading__sponsor]:
  #sponsor
  "&#x1F4B1; Methods for financially supporting rust-utilities that maintains tweet-archive-to-markdown"


Thanks for even considering it!

Via Liberapay you may
<sub>[![sponsor__shields_io__liberapay]][sponsor__link__liberapay]</sub> on a
repeating basis.

Regardless of if you're able to financially support projects such as
tweet-archive-to-markdown that rust-utilities maintains, please consider
sharing projects that are useful with others, because one of the goals of
maintaining Open Source repositories is to provide value to the community.


______


## Attribution
[heading__attribution]:
  #attribution
  "&#x1F4C7; Resources that where helpful in building this project so far."


- [GitHub -- `github-utilities/make-readme`](https://github.com/github-utilities/make-readme)
- [GitHub -- `rust-utilities/twitter-archive`](https://github.com/rust-utilities/twitter-archive)


______


## License
[heading__license]:
  #license
  "&#x2696; Legal side of Open Source"


This project is licensed based on use-case


---


### Commercial and/or proprietary use
[heading__commercial_andor_proprietary_use]: #commercial-andor-proprietary-use


If a project is **either** commercial or (`||`) proprietary, then please
contact the author for pricing and licensing options to make use of code and/or
features from this repository.


---


### Non-commercial and FOSS use
[heading__noncommercial_and_foss_use]: #noncommercial-and-foss-use


If a project is **both** non-commercial and (`&&`) published with a licence
compatible with AGPL-3.0, then it may utilize code from this repository under
the following terms.


```
Serde structs, deserialize, and serialize definitions for Twitter archived data
Copyright (C) 2024 S0AndS0

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, version 3 of the License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```



[branch__current__license]:
  /LICENSE
  "&#x2696; Full length version of AGPL-3.0 License"

[badge__license]:
  https://img.shields.io/github/license/rust-utilities/tweet-archive-to-markdown

[badge__commits__tweet_archive_to_markdown__main]:
  https://img.shields.io/github/last-commit/rust-utilities/tweet-archive-to-markdown/main.svg

[commits__tweet_archive_to_markdown__main]:
  https://github.com/rust-utilities/tweet-archive-to-markdown/commits/main
  "&#x1F4DD; History of changes on this branch"

[tweet_archive_to_markdown__community]:
  https://github.com/rust-utilities/tweet-archive-to-markdown/community
  "&#x1F331; Dedicated to functioning code"

[issues__tweet_archive_to_markdown]:
  https://github.com/rust-utilities/tweet-archive-to-markdown/issues
  "&#x2622; Search for and _bump_ existing issues or open new issues for project maintainer to address."

[tweet_archive_to_markdown__fork_it]:
  https://github.com/rust-utilities/tweet-archive-to-markdown/fork
  "&#x1F531; Fork it!"

[pull_requests__tweet_archive_to_markdown]:
  https://github.com/rust-utilities/tweet-archive-to-markdown/pulls
  "&#x1F3D7; Pull Request friendly, though please check the Community guidelines"

[tweet_archive_to_markdown__main__source_code]:
  https://github.com/rust-utilities/tweet-archive-to-markdown/
  "&#x2328; Project source!"

[badge__issues__tweet_archive_to_markdown]:
  https://img.shields.io/github/issues/rust-utilities/tweet-archive-to-markdown.svg

[badge__pull_requests__tweet_archive_to_markdown]:
  https://img.shields.io/github/issues-pr/rust-utilities/tweet-archive-to-markdown.svg

[badge__main__tweet_archive_to_markdown__source_code]:
  https://img.shields.io/github/repo-size/rust-utilities/tweet-archive-to-markdown

[rust_home]:
  https://www.rust-lang.org/
  "Home page for Rust language"

[rust_github]:
  https://github.com/rust-lang
  "Source code for Rust on GitHub"

[sponsor__shields_io__liberapay]:
  https://img.shields.io/static/v1?logo=liberapay&label=Sponsor&message=rust-utilities

[sponsor__link__liberapay]:
  https://liberapay.com/rust-utilities
  "&#x1F4B1; Sponsor developments and projects that rust-utilities maintains via Liberapay"

[badge__github_actions]:
  https://github.com/rust-utilities/tweet-archive-to-markdown/actions/workflows/test.yaml/badge.svg?branch=main

[activity_log__github_actions]:
  https://github.com/rust-utilities/tweet-archive-to-markdown/deployments/activity_log

