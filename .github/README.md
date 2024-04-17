# Twitter Archive
[heading__top]:
  #twitter-archive
  "&#x2B06; Serde structs, deserialize, and serialize definitions for Twitter archived data"


Serde structs, deserialize, and serialize definitions for Twitter archived data

## [![Byte size of Twitter Archive][badge__main__twitter_archive__source_code]][twitter_archive__main__source_code] [![Open Issues][badge__issues__twitter_archive]][issues__twitter_archive] [![Open Pull Requests][badge__pull_requests__twitter_archive]][pull_requests__twitter_archive] [![Latest commits][badge__commits__twitter_archive__main]][commits__twitter_archive__main]   [![GitHub Actions Build Status][badge__github_actions]][activity_log__github_actions] [![License][badge__license]][branch__current__license]


---


- [:arrow_up: Top of Document][heading__top]
- [:building_construction: Requirements][heading__requirements]
- [:zap: Quick Start][heading__quick_start]
- [&#x1F9F0; Usage][heading__usage]
- [&#x1F5D2; Notes][heading__notes]
  - [Tips for application authors][heading__tips_for_application_authors]
  - [Running tests][heading__running_tests]
  - [Running examples][heading__running_examples]
- [:chart_with_upwards_trend: Contributing][heading__contributing]
  - [:trident: Forking][heading__forking]
  - [:currency_exchange: Sponsor][heading__sponsor]
- [:card_index: Attribution][heading__attribution]
- [:balance_scale: Licensing][heading__license]
  - [Commercial and/or proprietary use][heading__commercial_andor_proprietary_use]
  - [Non-commercial and FOSS use][heading__noncommercial_and_foss_use]


---



## Requirements
[heading__requirements]:
  #requirements
  "&#x1F3D7; Prerequisites and/or dependencies that this project needs to function properly"


This repository requires [Rust][rust_home] language/compiler to build from
source

As of last update to this ReadMe file, the recommended method of installing
Rust is via the installer script...

```Bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


______


## Quick Start
[heading__quick_start]:
  #quick-start
  "&#9889; Perhaps as easy as one, 2.0,..."


This repository is a Rust library, define it as a dependency within a project
`Cargo.toml` file...

```bash
cargo add twitter-archive
```

**`Cargo.toml` (snip)**

```toml
[dependencies]
twitter_archive = "0.0.1"
```

> Check
> [Rust -- Doc -- Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
> for details about defining dependencies.

Then include within a source file via `use` statement...

```rust
use twitter_archive;
```


______


## Usage
[heading__usage]:
  #usage
  "&#x1F9F0; How to utilize this repository"


Twitter archive parsing example, print all tweets' creation date and full text;

```rust
use zip::read::ZipArchive;
use std::{fs, path};
use twitter_archive::structs::tweets;

fn main() {
    let input_file = "path/to/twitter.zip";

    let file_descriptor = fs::File::open(input_file).expect("Unable to read --input-file");
    let mut zip_archive = ZipArchive::new(file_descriptor).unwrap();
    let mut zip_file = zip_archive.by_name("data/tweets.js").unwrap();

    let mut buff = String::new();
    zip_file.read_to_string(&mut buff).unwrap();
    let json = buff.replacen("window.YTD.tweets.part0 = ", "", 1);

    let data: Vec<tweets::TweetObject> = serde_json::from_str(&json).expect("Unable to parse");

    for (index, object) in data.iter().enumerate() {
        /* Do stuff with each Tweet */
        println!("Index: {index}");
        println!("Created at: {}", object.tweet.created_at);
        println!("vvv Content\n{}\n^^^ Content", object.tweet.full_text);
    }
}
```

Check the `examples/` directory for more examples!


______


## Notes
[heading__notes]:
  #notes
  "&#x1F5D2; Additional things to keep in mind when developing"


This repository is **not** be feature complete or fully functional, Pull
Requests that add features or fix bugs are certainly welcomed.


---


### Tips for application authors
[heading__tips_for_application_authors]: #tips-for-application-authors


The `data/manifest.js` file, parse-able via `src/structs/manifest.rs`, defines
pointers to files and strings that may be helpful for pre-parsing/stripping of
other files within the archived directory/file structure.

All accessors/key-names defined by JSON/JavaScript Twitter archive data are
available via `snake_case` via Rust data-structures, regardless of source's
choice(s) to mix `camelCase` and `snake_case` formatting.


---


### Running tests
[heading__running_tests]: #running-tests


Individual data-structures documentation test may be run via;

```bash
RUST_BACKTRACE=1 cargo test --doc 'structs::personalization::InferredAgeInfo'
```

---


### Running examples
[heading__running_examples]: #running-examples


Examples may be run via `cargo` incantations similar to;

```bash
cargo run --example search-tweets -- --help
```

> Note; the `--` separator to pass arguments to the example instead of Cargo
> sub-command


______


## Contributing
[heading__contributing]:
  #contributing
  "&#x1F4C8; Options for contributing to twitter-archive and rust-utilities"


Options for contributing to twitter-archive and rust-utilities


---


### Forking
[heading__forking]:
  #forking
  "&#x1F531; Tips for forking twitter-archive"


> :warning: Creating fork(s), submitting contribution(s), publishing derivative
> work(s), etc. based on this repository will form an agreement to be bound by
> the use-cased based [licensing][heading__license] sub-sections.
>
> I.E. if you choose to contribute to or use this project, you acknowledge and
> accept these usage based licensing terms will apply to any such works too.

Start making a [Fork][twitter_archive__fork_it] of this repository to an
account that you have write permissions for.


- Add remote for fork URL. The URL syntax is
  _`git@github.com:<NAME>/<REPO>.git`_...


```Bash
cd ~/git/hub/rust-utilities/twitter-archive

git remote add fork git@github.com:<NAME>/twitter-archive.git
```


- Commit your changes and push to your fork, eg. to fix an issue...

```Bash
cd ~/git/hub/rust-utilities/twitter-archive


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
  "&#x1F4B1; Methods for financially supporting rust-utilities that maintains twitter-archive"


Thanks for even considering it!

Via Liberapay you may
<sub>[![sponsor__shields_io__liberapay]][sponsor__link__liberapay]</sub> on a
repeating basis.

Regardless of if you're able to financially support projects such as
twitter-archive that rust-utilities maintains, please consider sharing projects
that are useful with others, because one of the goals of maintaining Open
Source repositories is to provide value to the community.


______


## Attribution
[heading__attribution]:
  #attribution
  "&#x1F4C7; Resources that where helpful in building this project so far."


- [GitHub -- `github-utilities/make-readme`](https://github.com/github-utilities/make-readme)
- [GitHub -- `rust-utilities/tweet-archive-to-markdown`](https://github.com/rust-utilities/tweet-archive-to-markdown)
- [Stack Overflow -- How can I use serdy JSON on a JSON object with variable key names](https://stackoverflow.com/questions/58233949/how-can-i-use-serde-json-on-a-json-object-with-variable-key-names)


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

... For further details review full length version of
[AGPL-3.0][branch__current__license] License.



[branch__current__license]:
  /LICENSE
  "&#x2696; Full length version of AGPL-3.0 License"

[badge__license]:
  https://img.shields.io/github/license/rust-utilities/twitter-archive

[badge__commits__twitter_archive__main]:
  https://img.shields.io/github/last-commit/rust-utilities/twitter-archive/main.svg

[commits__twitter_archive__main]:
  https://github.com/rust-utilities/twitter-archive/commits/main
  "&#x1F4DD; History of changes on this branch"

[twitter_archive__community]:
  https://github.com/rust-utilities/twitter-archive/community
  "&#x1F331; Dedicated to functioning code"

[issues__twitter_archive]:
  https://github.com/rust-utilities/twitter-archive/issues
  "&#x2622; Search for and _bump_ existing issues or open new issues for project maintainer to address."

[twitter_archive__fork_it]:
  https://github.com/rust-utilities/twitter-archive/fork
  "&#x1F531; Fork it!"

[pull_requests__twitter_archive]:
  https://github.com/rust-utilities/twitter-archive/pulls
  "&#x1F3D7; Pull Request friendly, though please check the Community guidelines"

[twitter_archive__main__source_code]:
  https://github.com/rust-utilities/twitter-archive/
  "&#x2328; Project source!"

[badge__issues__twitter_archive]:
  https://img.shields.io/github/issues/rust-utilities/twitter-archive.svg

[badge__pull_requests__twitter_archive]:
  https://img.shields.io/github/issues-pr/rust-utilities/twitter-archive.svg

[badge__main__twitter_archive__source_code]:
  https://img.shields.io/github/repo-size/rust-utilities/twitter-archive

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
  https://github.com/rust-utilities/twitter-archive/actions/workflows/test.yaml/badge.svg?branch=main

[activity_log__github_actions]:
  https://github.com/rust-utilities/twitter-archive/deployments/activity_log

[truffle__package_management_via_npm]:
  https://www.trufflesuite.com/docs/truffle/getting-started/package-management-via-npm
  "Documentation on how to install, import, and interact with Solidity packages"
