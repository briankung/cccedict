#![allow(dead_code, unused_variables, unused_imports)]

pub type BoxError = std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

#[derive(Debug, Clone, Default)]
pub struct CedictEntry<'a> {
    pub traditional: &'a str,
    pub simplified: &'a str,
    pub pinyin: Vec<Syllable<'a>>,
    pub jyutping: Vec<Syllable<'a>>,
    pub definitions: Vec<&'a str>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Syllable<'a> {
    pronunciation: &'a str,
    tone: &'a str,
}

pub(self) mod parsers {
    use super::*;

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
        nom::sequence::delimited(
            nom::bytes::complete::tag("["),
            nom::multi::separated_list1(
                nom::bytes::complete::tag(" "),
                nom::combinator::map_parser(nom::bytes::complete::is_not(" ]"), syllable),
            ),
            nom::bytes::complete::tag("]"),
        )(i)
    }

    fn jyutping(i: &str) -> nom::IResult<&str, Vec<Syllable>> {
        nom::sequence::delimited(
            nom::bytes::complete::tag("{"),
            nom::multi::separated_list1(
                nom::bytes::complete::tag(" "),
                nom::combinator::map_parser(nom::bytes::complete::is_not(" }"), syllable),
            ),
            nom::bytes::complete::tag("}"),
        )(i)
    }

    fn syllable(i: &str) -> nom::IResult<&str, Syllable> {
        let (rest, (pronunciation, tone)) = nom::sequence::tuple((
            nom::character::complete::alpha1,
            nom::character::complete::digit0,
        ))(i)?;

        Ok((
            rest,
            Syllable {
                pronunciation,
                tone,
            },
        ))
    }

    fn definitions(i: &str) -> nom::IResult<&str, Vec<&str>> {
        nom::sequence::delimited(
            nom::bytes::complete::tag("/"),
            nom::multi::separated_list0(
                nom::bytes::complete::tag("/"),
                nom::bytes::complete::is_not("/"),
            ),
            nom::bytes::complete::tag("/"),
        )(i)
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
        fn test_parse_pinyin() {
            assert_eq!(
                pinyin("[ni3 hao3]"),
                Ok((
                    "",
                    vec![
                        Syllable {
                            pronunciation: "ni",
                            tone: "3"
                        },
                        Syllable {
                            pronunciation: "hao",
                            tone: "3"
                        }
                    ]
                ))
            )
        }

        #[test]
        fn test_parse_pinyin_syllable() {
            assert_eq!(
                syllable("ni3"),
                Ok((
                    "",
                    Syllable {
                        pronunciation: "ni",
                        tone: "3"
                    }
                ))
            );
            assert_eq!(
                syllable("hao3"),
                Ok((
                    "",
                    Syllable {
                        pronunciation: "hao",
                        tone: "3"
                    }
                ))
            );
        }

        #[test]
        fn test_parse_jyutping() {
            assert_eq!(
                jyutping("{jat1 go3}"),
                Ok((
                    "",
                    vec![
                        Syllable {
                            pronunciation: "jat",
                            tone: "1"
                        },
                        Syllable {
                            pronunciation: "go",
                            tone: "3"
                        }
                    ]
                ))
            );
            assert_eq!(
                jyutping("{jat1go3}"),
                Ok((
                    "",
                    vec![
                        Syllable {
                            pronunciation: "jat",
                            tone: "1"
                        },
                        Syllable {
                            pronunciation: "go",
                            tone: "3"
                        }
                    ]
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

        // #[test]
        // #[ignore]
        // fn test_parse_line() {
        //     let line = "抄字典 抄字典 [chao1 zi4dian3] {caau3 zi6 din2} /to search / flip through a dictionary [colloquial]/";
        //     assert_eq!(
        //         parse_line(line),
        //         CedictEntry {
        //             traditional: "抄字典",
        //             simplified: "抄字典",
        //             pinyin: vec![
        //                 Syllable {
        //                     pronunciation: "chao",
        //                     tone: "1"
        //                 },
        //                 Syllable {
        //                     pronunciation: "zi",
        //                     tone: "4"
        //                 },
        //                 Syllable {
        //                     pronunciation: "dian",
        //                     tone: "3"
        //                 }
        //             ],
        //             jyutping: vec![
        //                 Syllable {
        //                     pronunciation: "caau",
        //                     tone: "3"
        //                 },
        //                 Syllable {
        //                     pronunciation: "zi",
        //                     tone: "6"
        //                 },
        //                 Syllable {
        //                     pronunciation: "din",
        //                     tone: "2"
        //                 }
        //             ],
        //             definitions: vec!["to search ", " flip through a dictionary [colloquial]"]
        //         }
        //     )
        // }
    }
}
