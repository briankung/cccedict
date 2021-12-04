/*!
A `Syllable` represents a single syllable containing one pronunciation and one tone.

# Usage:
```
use cccedict::syllable::*;

let syllable = Syllable::new("ni", "3");

assert_eq!(syllable.pronunciation, "ni");
assert_eq!(syllable.tone, "3");
```

Currently there's no validation that either pronunciation or tone are valid inputs:

```
# use cccedict::syllable::*;
let syllable = Syllable::new("life", "42");

assert_eq!(syllable.pronunciation, "life");
assert_eq!(syllable.tone, "42");
```
*/

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Syllable<'a> {
    pub pronunciation: &'a str,
    /// While both jyutping and pinyin use numbers to denote tones, we are not doing mathematical
    /// operations with them so they remain `str`s.
    pub tone: &'a str,
}

impl<'a> Syllable<'a> {
    pub fn new(pronunciation: &'a str, tone: &'a str) -> Self {
        Syllable {
            pronunciation,
            tone,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
