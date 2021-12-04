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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Syllable {
    pub pronunciation: String,
    /// While both jyutping and pinyin use numbers to denote tones, we are not doing mathematical
    /// operations with them so they remain `String`s.
    pub tone: String,
}

impl Syllable {
    pub fn new(pronunciation: &str, tone: &str) -> Self {
        Syllable {
            pronunciation: pronunciation.to_string(),
            tone: tone.to_string(),
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
                pronunciation: "ni".to_string(),
                tone: "3".to_string()
            }
        )
    }
}
