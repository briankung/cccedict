/*!
cccedict is a [ CC-CEDICT](https://cc-cedict.org/wiki/format:syntax) parser for parsing
Chinese/English natural language dictionaries. It has the unique feature of supporting the
[cantonese.org](https://cantonese.org/) extensions to the CC-CEDICT format, which adds support
for [jyutping](https://en.wikipedia.org/wiki/Jyutping) pronunciations.
*/

pub mod cedict_entry;
pub mod errors;
pub mod syllable;
