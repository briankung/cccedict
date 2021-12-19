/*!
A CedictEntry represents a single entry in a Cedict.

# Usage:
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
*/

use crate::errors::{BoxError, CedictEntryError};
pub use crate::syllable::Syllable;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CedictEntry {
    pub traditional: String,
    pub simplified: String,
    pub pinyin: Option<Vec<Syllable>>,
    pub jyutping: Option<Vec<Syllable>>,
    pub definitions: Option<Vec<String>>,
}

impl CedictEntry {
    pub fn new(input: &str) -> Result<CedictEntry, BoxError> {
        match parsers::parse_line(input).unwrap_or(("", None)) {
            (_, Some(entry)) => Ok(entry),
            (_, None) => Err(Box::new(CedictEntryError)),
        }
    }
}

pub(self) mod parsers {
    use super::*;

    use nom::{bytes, character, combinator, multi, sequence, IResult};

    pub fn parse_line(i: &str) -> IResult<&str, Option<CedictEntry>> {
        combinator::all_consuming(|i| {
            let (i, entry) = combinator::opt(cedict_entry)(i)?;
            let (i, _) = character::complete::space0(i)?;
            let (i, _) = combinator::opt(comment)(i)?;

            Ok((i, entry))
        })(i)
    }

    fn cedict_entry(i: &str) -> IResult<&str, CedictEntry> {
        let (i, traditional) = not_whitespace(i)?;
        let (i, _) = character::complete::space1(i)?;
        let (i, simplified) = not_whitespace(i)?;
        let (i, _) = character::complete::space1(i)?;
        let (i, pinyin) = pinyin(i)?;
        let (i, _) = character::complete::space1(i)?;
        let (i, jyutping) = combinator::opt(jyutping)(i)?;
        let (i, _) = character::complete::space0(i)?;
        let (i, definitions) = definitions(i)?;

        Ok((
            i,
            CedictEntry {
                traditional: traditional.into(),
                simplified: simplified.into(),
                pinyin,
                jyutping,
                definitions,
            },
        ))
    }

    fn comment(i: &str) -> IResult<&str, (&str, &str)> {
        sequence::tuple((
            bytes::complete::tag("#"),
            character::complete::not_line_ending,
        ))(i)
    }

    fn not_whitespace(i: &str) -> IResult<&str, &str> {
        bytes::complete::is_not(" \t")(i)
    }

    fn pinyin(i: &str) -> IResult<&str, Option<Vec<Syllable>>> {
        let (rest, (_, pronunciations, _)) = sequence::tuple((
            bytes::complete::tag("["),
            combinator::opt(bytes::complete::is_not("]")),
            bytes::complete::tag("]"),
        ))(i)?;

        if let Some(pronunciations) = pronunciations {
            let (_, syllables) = syllables(pronunciations)?;

            Ok((rest, Some(syllables)))
        } else {
            Ok((rest, None))
        }
    }

    fn jyutping(i: &str) -> IResult<&str, Vec<Syllable>> {
        let (rest, pronunciations) = sequence::delimited(
            bytes::complete::tag("{"),
            bytes::complete::is_not("}"),
            bytes::complete::tag("}"),
        )(i)?;

        let (_, syllables) = syllables(pronunciations)?;

        Ok((rest, syllables))
    }

    /// takes a series of possibly undelimited syllables such as "ni3hao3" and returns a Vec of Syllables
    fn syllables(i: &str) -> IResult<&str, Vec<Syllable>> {
        multi::many0(syllable)(i)
    }

    fn syllable(i: &str) -> IResult<&str, Syllable> {
        let (rest, (_, pronunciation, tone)) = sequence::tuple((
            character::complete::space0,
            character::complete::alpha1,
            character::complete::digit0,
        ))(i)?;

        Ok((rest, Syllable::new(pronunciation, tone)))
    }

    fn definitions(i: &str) -> IResult<&str, Option<Vec<String>>> {
        if let Some(last_slash) = i.rfind('/') {
            let (defs, rest) = i.split_at(last_slash + 1);

            let (_, untrimmed_defs) = sequence::delimited(
                bytes::complete::tag("/"),
                multi::separated_list0(bytes::complete::tag("/"), bytes::complete::is_not("/")),
                bytes::complete::tag("/"),
            )(defs)?;

            if untrimmed_defs.is_empty() {
                return Ok((rest, None));
            }

            let trimmed: Vec<String> = untrimmed_defs.iter().map(|x| x.trim().to_owned()).collect();

            Ok((rest, Some(trimmed)))
        } else {
            Ok((i, None))
        }
    }

    #[cfg(test)]
    mod test_cedict_entry {
        use super::*;

        #[test]
        fn test_new() {
            let line = "抄字典 抄字典 [chao1 zi4dian3] /to search / flip through a dictionary [colloquial]/ # adapted from cc-cedict";
            let expected_result = CedictEntry {
                traditional: "抄字典".into(),
                simplified: "抄字典".into(),
                pinyin: Some(vec![
                    Syllable::new("chao", "1"),
                    Syllable::new("zi", "4"),
                    Syllable::new("dian", "3"),
                ]),
                jyutping: None,
                definitions: Some(vec![
                    "to search".into(),
                    "flip through a dictionary [colloquial]".into(),
                ]),
            };

            match CedictEntry::new(line) {
                Err(_) => panic!(),
                Ok(result) => assert_eq!(result, expected_result),
            }
        }

        #[test]
        fn test_new_with_invalid_lines() {
            let line = "hi";
            match CedictEntry::new(line) {
                Ok(_) => panic!(),
                Err(err) => assert_eq!(err.to_string(), "invalid cedict entry input"),
            };

            let line = "你好";
            match CedictEntry::new(line) {
                Ok(_) => panic!(),
                Err(err) => assert_eq!(err.to_string(), "invalid cedict entry input"),
            };

            let line = "抄字典 [chao1 zi4dian3] /to search / flip through a dictionary [colloquial]/ # adapted from cc-cedict";
            match CedictEntry::new(line) {
                Ok(_) => panic!(),
                Err(err) => assert_eq!(err.to_string(), "invalid cedict entry input"),
            };
        }
    }

    #[cfg(test)]
    mod test_parsers {
        use super::*;
        use crate::errors::BoxError;

        #[test]
        fn test_not_whitespace() {
            assert_eq!(not_whitespace("你好 阿婆"), Ok((" 阿婆", "你好")));
            assert_eq!(not_whitespace("你好\t阿婆"), Ok(("\t阿婆", "你好")));
            assert_eq!(
                not_whitespace("\t你好阿婆"),
                Err(nom::Err::Error(nom::error::Error::new(
                    "\t你好阿婆",
                    nom::error::ErrorKind::IsNot
                )))
            );
        }

        #[test]
        fn test_parse_definitions() {
            assert_eq!(
                definitions("/watch a movie/three goals/card/(deck of playing cards)/"),
                Ok((
                    "",
                    Some(vec![
                        "watch a movie".into(),
                        "three goals".into(),
                        "card".into(),
                        "(deck of playing cards)".into(),
                    ])
                ))
            )
        }

        #[test]
        fn test_parse_definitions_with_comments() {
            assert_eq!(
                definitions("/watch a movie/three goals/card/(deck of playing cards)/ # hi"),
                Ok((
                    " # hi",
                    Some(vec![
                        "watch a movie".into(),
                        "three goals".into(),
                        "card".into(),
                        "(deck of playing cards)".into(),
                    ])
                ))
            );

            assert_eq!(
                definitions("/watch a movie/three goals/card/(deck of playing cards)/# hi"),
                Ok((
                    "# hi",
                    Some(vec![
                        "watch a movie".into(),
                        "three goals".into(),
                        "card".into(),
                        "(deck of playing cards)".into(),
                    ])
                ))
            )
        }

        #[test]
        fn test_parse_missing_definitions() {
            assert_eq!(definitions("//"), Ok(("", None)));
            assert_eq!(definitions(""), Ok(("", None)));
        }

        #[test]
        fn test_parse_definitions_are_trimmed() {
            assert_eq!(
                definitions("/  watch a movie  / three goals/card/(deck of playing cards) /"),
                Ok((
                    "",
                    Some(vec![
                        "watch a movie".into(),
                        "three goals".into(),
                        "card".into(),
                        "(deck of playing cards)".into(),
                    ])
                ))
            )
        }

        #[test]
        fn test_parse_pinyin() {
            assert_eq!(
                pinyin("[ni3 hao3]"),
                Ok((
                    "",
                    Some(vec![Syllable::new("ni", "3"), Syllable::new("hao", "3")])
                ))
            );
        }

        #[test]
        fn test_parse_pinyin_with_irregular_spacing() {
            assert_eq!(
                pinyin("[ni3hao3 ma5]"),
                Ok((
                    "",
                    Some(vec![
                        Syllable::new("ni", "3"),
                        Syllable::new("hao", "3"),
                        Syllable::new("ma", "5")
                    ])
                ))
            );

            assert_eq!(
                pinyin("[ ni3hao3 ma5 ]"),
                Ok((
                    "",
                    Some(vec![
                        Syllable::new("ni", "3"),
                        Syllable::new("hao", "3"),
                        Syllable::new("ma", "5")
                    ])
                ))
            );
        }

        #[test]
        fn test_parse_empty_pinyin() {
            assert_eq!(pinyin("[]   "), Ok(("   ", None)));
        }

        #[test]
        fn test_parse_pinyin_syllable() {
            assert_eq!(syllable("ni3"), Ok(("", Syllable::new("ni", "3"))));
            assert_eq!(syllable("hao3"), Ok(("", Syllable::new("hao", "3"))));
        }

        #[test]
        fn test_parse_pinyin_syllable_without_tone() {
            assert_eq!(syllable("ma"), Ok(("", Syllable::new("ma", ""))));
        }

        #[test]
        fn test_parse_syllables() {
            assert_eq!(
                syllables("ni3hao3"),
                Ok((
                    "",
                    vec![Syllable::new("ni", "3"), Syllable::new("hao", "3")]
                ))
            );
        }

        #[test]
        fn test_parse_jyutping() {
            assert_eq!(
                jyutping("{jat1 go3}"),
                Ok((
                    "",
                    vec![Syllable::new("jat", "1"), Syllable::new("go", "3")]
                ))
            );
            assert_eq!(
                jyutping("{jat1go3}"),
                Ok((
                    "",
                    vec![Syllable::new("jat", "1"), Syllable::new("go", "3")]
                ))
            )
        }

        #[test]
        fn test_comments() {
            assert_eq!(
                comment("# this is a comment"),
                Ok(("", ("#", " this is a comment")))
            )
        }

        #[test]
        fn test_cedict_entry() {
            let line = "抄字典 抄字典 [chao1 zi4dian3] {caau3 zi6 din2} /to search / flip through a dictionary [colloquial]/";
            assert_eq!(
                cedict_entry(line),
                Ok((
                    "",
                    CedictEntry {
                        traditional: "抄字典".into(),
                        simplified: "抄字典".into(),
                        pinyin: Some(vec![
                            Syllable::new("chao", "1"),
                            Syllable::new("zi", "4"),
                            Syllable::new("dian", "3"),
                        ]),
                        jyutping: Some(vec![
                            Syllable::new("caau", "3"),
                            Syllable::new("zi", "6"),
                            Syllable::new("din", "2"),
                        ]),
                        definitions: Some(vec![
                            "to search".into(),
                            "flip through a dictionary [colloquial]".into()
                        ])
                    }
                ))
            )
        }

        #[test]
        fn test_cedict_entry_without_jyutping() {
            let line =
            "抄字典 抄字典 [chao1 zi4dian3] /to search / flip through a dictionary [colloquial]/";
            assert_eq!(
                cedict_entry(line),
                Ok((
                    "",
                    CedictEntry {
                        traditional: "抄字典".into(),
                        simplified: "抄字典".into(),
                        pinyin: Some(vec![
                            Syllable::new("chao", "1"),
                            Syllable::new("zi", "4"),
                            Syllable::new("dian", "3"),
                        ]),
                        jyutping: None,
                        definitions: Some(vec![
                            "to search".into(),
                            "flip through a dictionary [colloquial]".into()
                        ])
                    }
                ))
            )
        }

        #[test]
        fn test_cedict_entry_with_comment() {
            let line = "抄字典 抄字典 [chao1 zi4dian3] /to search / flip through a dictionary [colloquial]/ # adapted from cc-cedict";
            assert_eq!(
                cedict_entry(line),
                Ok((
                    " # adapted from cc-cedict",
                    CedictEntry {
                        traditional: "抄字典".into(),
                        simplified: "抄字典".into(),
                        pinyin: Some(vec![
                            Syllable::new("chao", "1"),
                            Syllable::new("zi", "4"),
                            Syllable::new("dian", "3"),
                        ]),
                        jyutping: None,
                        definitions: Some(vec![
                            "to search".into(),
                            "flip through a dictionary [colloquial]".into()
                        ])
                    }
                ))
            )
        }

        #[test]
        fn test_parse_lines() -> Result<(), BoxError> {
            let lines = [
                "# this is a comment",
                "抄字典 抄字典 [chao1 zi4dian3] {caau3 zi6 din2} /to search / flip through a dictionary [colloquial]/",
                "以身作則 以身作则 [yi3 shen1 zuo4 ze2] /to set an example (idiom); to serve as a model/",
                "𠌥 𠆿 [] {wu1} /(verb) to lean over; to stoop/",
            ];

            for line in lines.iter() {
                parse_line(line)?;
            }

            Ok(())
        }
    }
}
