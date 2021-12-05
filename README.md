# cccedict

cccedict is a [ CC-CEDICT](https://cc-cedict.org/wiki/format:syntax) parser for parsing
Chinese/English natural language dictionaries. It has the unique feature of supporting the
[cantonese.org](https://cantonese.org/) extensions to the CC-CEDICT format, which adds support
for [jyutping](https://en.wikipedia.org/wiki/Jyutping) pronunciations.

## Usage

A `CedictEntry` represents a single entry in a `Cedict`:

```
use cccedict::cedict_entry::*;

let line = "你好嗎 你好吗 [ni3 hao3 ma5] {nei5 hou2 maa1} /how are you?/";
let entry = CedictEntry::new(line).unwrap();

assert_eq!(entry.traditional, "你好嗎");
assert_eq!(entry.simplified, "你好吗");
assert_eq!(entry.pinyin, Some(
    vec![
        Syllable::new("ni", "3"),
        Syllable::new("hao", "3"),
        Syllable::new("ma", "5"),
    ]
));
assert_eq!(entry.jyutping, Some(
    vec![
        Syllable::new("nei", "5"),
        Syllable::new("hou", "2"),
        Syllable::new("maa", "1"),
    ]
));
assert_eq!(entry.definitions, Some(vec!["how are you?".to_string()]));
```

You can also instantiate a `Cedict` from a `FromStr`, `Read`, or `AsRef<Path>` implementor:

```
use cccedict::cedict::Cedict;
use std::str::FromStr;

let cedict_entries = "\
你嘅 你嘅 [ni3 ge2] {nei5 ge3} /your's (spoken)/
你地 你地 [ni3 di4] {nei5 dei6} /you guys; you all/
你好嗎 你好吗 [ni3 hao3 ma5] {nei5 hou2 maa1} /how are you?/";

let cedict = Cedict::from_str(cedict_entries).unwrap();
assert_eq!(cedict.entries.len(), 3);

let reader: &[u8] = cedict_entries.as_bytes();
let cedict = Cedict::from_file(reader).unwrap();
assert_eq!(cedict.entries.len(), 3);

use std::path::Path;
let path = Path::new("fixtures/cccanto-test.txt");
let cedict = Cedict::from_path(path).unwrap();
assert_eq!(cedict.entries.len(), 3);
```


## Backlog

- [x] Add a `Cedict` struct to convert an entire cedict files into `CedictEntry`s
- [ ] Allow writes to `Cedict`s
- [ ] Allow searching a `Cedict`'s entries. Some things to think about:
        Exact matches, partial matches, and fuzzy matches. Also searching definitions, simplified
        characters, and traditional characters.
