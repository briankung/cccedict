pub type BoxError = std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CedictEntry<'a> {
    pub traditional: &'a str,
    pub simplified: &'a str,
    pub pinyin: Vec<Syllable<'a>>,
    pub jyutping: Option<Vec<Syllable<'a>>>,
    pub definitions: Vec<&'a str>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Syllable<'a> {
    pronunciation: &'a str,
    tone: &'a str,
}

impl<'a> Syllable<'a> {
    fn new(pronunciation: &'a str, tone: &'a str) -> Self {
        Syllable {
            pronunciation,
            tone,
        }
    }
}

pub(self) mod parsers {
    use super::*;

    pub fn parse_line(i: &str) -> nom::IResult<&str, CedictEntry> {
        let (i, traditional) = not_whitespace(i)?;
        let (i, _) = nom::bytes::complete::tag(" ")(i)?;
        let (i, simplified) = not_whitespace(i)?;
        let (i, _) = nom::bytes::complete::tag(" ")(i)?;
        let (i, pinyin) = pinyin(i)?;
        let (i, _) = nom::bytes::complete::tag(" ")(i)?;
        let (i, jyutping) = nom::combinator::opt(jyutping)(i)?;
        let (i, _) = nom::bytes::complete::tag(" ")(i)?;
        let (i, definitions) = definitions(i)?;

        Ok((
            i,
            CedictEntry {
                traditional,
                simplified,
                pinyin,
                jyutping,
                definitions,
            },
        ))
    }

    fn comment(i: &str) -> nom::IResult<&str, (&str, &str)> {
        nom::sequence::tuple((
            nom::bytes::complete::tag("#"),
            nom::character::complete::not_line_ending,
        ))(i)
    }

    fn not_whitespace(i: &str) -> nom::IResult<&str, &str> {
        nom::bytes::complete::is_not(" \t")(i)
    }

    fn pinyin(i: &str) -> nom::IResult<&str, Vec<Syllable>> {
        let (rest, delimited_syllables) = nom::sequence::delimited(
            nom::bytes::complete::tag("["),
            nom::bytes::complete::is_not("]"),
            nom::bytes::complete::tag("]"),
        )(i)?;

        let (_, syllables) = syllables(delimited_syllables)?;

        Ok((rest, syllables))
    }

    fn jyutping(i: &str) -> nom::IResult<&str, Vec<Syllable>> {
        let (rest, delimited_syllables) = nom::sequence::delimited(
            nom::bytes::complete::tag("{"),
            nom::bytes::complete::is_not("}"),
            nom::bytes::complete::tag("}"),
        )(i)?;

        let (_, syllables) = syllables(delimited_syllables)?;

        Ok((rest, syllables))
    }

    /// takes a series of undelimited syllables such as "ni3hao3" and returns a Vec of Syllables
    fn syllables(i: &str) -> nom::IResult<&str, Vec<Syllable>> {
        nom::multi::many1(syllable)(i)
    }

    fn syllable(i: &str) -> nom::IResult<&str, Syllable> {
        let (rest, (_, pronunciation, tone)) = nom::sequence::tuple((
            nom::character::complete::space0,
            nom::character::complete::alpha1,
            nom::character::complete::digit0,
        ))(i)?;

        Ok((rest, Syllable::new(pronunciation, tone)))
    }

    fn definitions(i: &str) -> nom::IResult<&str, Vec<&str>> {
        let (rest, untrimmed_defs) = nom::sequence::delimited(
            nom::bytes::complete::tag("/"),
            nom::multi::separated_list0(
                nom::bytes::complete::tag("/"),
                nom::bytes::complete::is_not("/"),
            ),
            nom::bytes::complete::tag("/"),
        )(i)?;

        Ok((rest, untrimmed_defs.iter().map(|x| x.trim()).collect()))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

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
                    vec![
                        "watch a movie",
                        "three goals",
                        "card",
                        "(deck of playing cards)",
                    ]
                ))
            )
        }

        #[test]
        fn test_parse_definitions_are_trimmed() {
            assert_eq!(
                definitions("/  watch a movie  / three goals/card/(deck of playing cards) /"),
                Ok((
                    "",
                    vec![
                        "watch a movie",
                        "three goals",
                        "card",
                        "(deck of playing cards)",
                    ]
                ))
            )
        }

        #[test]
        fn test_syllable_init() {
            assert_eq!(
                Syllable::new("ni", "3"),
                Syllable {
                    pronunciation: "ni",
                    tone: "3"
                }
            )
        }

        #[test]
        fn test_parse_pinyin() {
            assert_eq!(
                pinyin("[ni3 hao3]"),
                Ok((
                    "",
                    vec![Syllable::new("ni", "3"), Syllable::new("hao", "3")]
                ))
            );

            assert_eq!(
                pinyin("[ni3hao3 ma5]"),
                Ok((
                    "",
                    vec![
                        Syllable::new("ni", "3"),
                        Syllable::new("hao", "3"),
                        Syllable::new("ma", "5")
                    ]
                ))
            );

            assert_eq!(
                pinyin("[ ni3hao3 ma5 ]"),
                Ok((
                    "",
                    vec![
                        Syllable::new("ni", "3"),
                        Syllable::new("hao", "3"),
                        Syllable::new("ma", "5")
                    ]
                ))
            );
        }

        #[test]
        fn test_parse_pinyin_syllable() {
            assert_eq!(syllable("ni3"), Ok(("", Syllable::new("ni", "3"))));
            assert_eq!(syllable("hao3"), Ok(("", Syllable::new("hao", "3"))));
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
        fn test_parse_line() {
            let line = "抄字典 抄字典 [chao1 zi4dian3] {caau3 zi6 din2} /to search / flip through a dictionary [colloquial]/";
            assert_eq!(
                parse_line(line),
                Ok((
                    "",
                    CedictEntry {
                        traditional: "抄字典",
                        simplified: "抄字典",
                        pinyin: vec![
                            Syllable::new("chao", "1"),
                            Syllable::new("zi", "4"),
                            Syllable::new("dian", "3"),
                        ],
                        jyutping: Some(vec![
                            Syllable::new("caau", "3"),
                            Syllable::new("zi", "6"),
                            Syllable::new("din", "2"),
                        ]),
                        definitions: vec!["to search", "flip through a dictionary [colloquial]"]
                    }
                ))
            )
        }
    }
}
