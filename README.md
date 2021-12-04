# cccedict

cccedict is a [ CC-CEDICT](https://cc-cedict.org/wiki/format:syntax) parser for parsing
Chinese/English natural language dictionaries. It has the unique feature of supporting the
[cantonese.org](https://cantonese.org/) extensions to the CC-CEDICT format, which adds support
for [jyutping](https://en.wikipedia.org/wiki/Jyutping) pronunciations.

## Usage

A CedictEntry represents a single entry in a Cedict. As of the current version, this is the only way
to use the library:

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
assert_eq!(entry.definitions, Some(vec!["how are you?"]));
```

## Backlog

- [ ] Add a `Cedict` struct to convert an entire cedict files into `CedictEntry`s
- [ ] Implement some useful way of querying said `Cedict` (mimic `entries` API?)
