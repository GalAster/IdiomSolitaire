use pinyin::{to_pinyin_vec, Pinyin};
use serde::Deserialize;
use std::str::Chars;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct Idiom {
    #[serde(rename(deserialize = "Idiom"))]
    pub idiom: String,
    #[serde(rename(deserialize = "Pinyin"))]
    pub pinyin: String,
    #[serde(rename(deserialize = "Explanation"))]
    pub explanation: String,
}
impl From<&str> for Idiom {
    fn from(input: &str) -> Self {
        Self { idiom: String::from(input), pinyin: to_pinyin_vec(input, Pinyin::plain).join(" "), explanation: String::new() }
    }
}

impl Idiom {
    pub fn first_char(&self) -> char {
        self.idiom.chars().next().unwrap()
    }
    pub fn final_char(&self) -> char {
        self.idiom.chars().rev().next().unwrap()
    }
    pub fn first_sound(&self) -> String {
        Self::get_letter(self.pinyin.split(' ').next().unwrap().chars())
    }
    pub fn final_sound(&self) -> String {
        Self::get_letter(self.pinyin.split(' ').rev().next().unwrap().chars())
    }
    pub fn first_tone(&self) -> String {
        self.pinyin.split(' ').next().unwrap().to_owned()
    }
    pub fn final_tone(&self) -> String {
        self.pinyin.split(' ').rev().next().unwrap().to_owned()
    }
    fn get_letter(chars: Chars) -> String {
        let mut out = String::new();
        for c in chars {
            match c {
                'ā' | 'á' | 'ǎ' | 'à' => out.push('a'),
                'ē' | 'é' | 'ě' | 'è' => out.push('e'),
                'ī' | 'í' | 'ǐ' | 'ì' => out.push('i'),
                'ō' | 'ó' | 'ǒ' | 'ò' => out.push('o'),
                'ū' | 'ú' | 'ǔ' | 'ù' => out.push('u'),
                'ǖ' | 'ǘ' | 'ǚ' | 'ǜ' => out.push('u'),
                ' ' => break,
                _ => out.push(c),
            }
        }
        return out;
    }
}
