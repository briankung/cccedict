/*!
A `Cedict` stores and provides the API for interacting with a CC-CEDICT compatible dictionary.

# Usage:

You can instantiate a `Cedict` from a `FromStr` implementor:

```
use cccedict::cedict::Cedict;
use std::str::FromStr;

let cedict_entries = "\
你嘅 你嘅 [ni3 ge2] {nei5 ge3} /your's (spoken)/
你地 你地 [ni3 di4] {nei5 dei6} /you guys; you all/
你好嗎 你好吗 [ni3 hao3 ma5] {nei5 hou2 maa1} /how are you?/";

let cedict = Cedict::from_str(cedict_entries).unwrap();
assert_eq!(cedict.entries.len(), 3);
```

You can also instantiate one from a `Read` implementor:


```
# use cccedict::cedict::Cedict;
# use std::str::FromStr;
#
# let cedict_entries = "\
# 你嘅 你嘅 [ni3 ge2] {nei5 ge3} /your's (spoken)/
# 你地 你地 [ni3 di4] {nei5 dei6} /you guys; you all/
# 你好嗎 你好吗 [ni3 hao3 ma5] {nei5 hou2 maa1} /how are you?/";

let reader: &[u8] = cedict_entries.as_bytes();
let cedict = Cedict::from_file(reader).unwrap();
assert_eq!(cedict.entries.len(), 3);
```

Finally, you can instantiate a `Cedict` from a path to a file:

```
# use cccedict::cedict::Cedict;
# use std::str::FromStr;
use std::path::Path;
let path = Path::new("fixtures/cccanto-test.txt");
let cedict = Cedict::from_path(path).unwrap();
assert_eq!(cedict.entries.len(), 3);
```
*/

pub use crate::cedict_entry::CedictEntry;
use crate::errors::BoxError;
pub use crate::errors::CedictError;
use std::str::FromStr;

use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cedict {
    pub entries: Vec<CedictEntry>,
}

impl FromStr for Cedict {
    type Err = BoxError;
    // add code here
    fn from_str(cedict_entries: &str) -> Result<Self, Self::Err> {
        let entries: Vec<CedictEntry> = cedict_entries
            .lines()
            .filter_map(|line| CedictEntry::new(line).ok())
            .collect();
        Ok(Cedict { entries })
    }
}

impl Cedict {
    pub fn from_file<R: Read>(mut cedict_reader: R) -> Result<Self, BoxError> {
        let mut cedict_entries: String = "".into();
        cedict_reader.read_to_string(&mut cedict_entries)?;

        Self::from_str(&cedict_entries)
    }

    pub fn from_path<P: AsRef<Path>>(cedict_path: P) -> Result<Self, BoxError> {
        let cedict_file = File::open(cedict_path)?;
        Self::from_file(cedict_file)
    }
}
