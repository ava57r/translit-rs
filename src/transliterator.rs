use super::CharsMapping;

use std::cmp::Ordering;

use crate::bulgarian;
use crate::gost779;
use crate::macedonian;
use crate::passport2013;

/// The contract for transliteration in the Latin alphabet
pub trait ToLatin {
    fn to_latin(&self, src: &str) -> String;
}

/// The contract for transliteration from Latin alphabet
pub trait FromLatin {
    fn from_latin(&self, src: &str) -> String;
}

/// Support Languages
pub enum Language {
    Ru,
    By,
    Ua,
}

/// The `Transliterator` struct allows for the transliteration
/// of the string of characters of Cyrillic alphabet UTF-8 to Latin alphabet
/// and back.
///
pub struct Transliterator {
    rules: CharsMapping,
}

impl Transliterator {
    /// Creates a new `Transliterator` with transliteration table
    ///
    /// Examples
    ///
    /// ```rust
    ///
    /// use translit::{Transliterator, CharsMapping};
    /// let table: CharsMapping =
    /// [
    /// ("а", "a"),
    /// ("с", "s"),
    /// ("д", "d"),
    /// ("ф", "f"),
    /// ].iter()
    ///   .cloned()
    ///   .collect();
    ///
    /// let trasliterator = Transliterator::new(table);
    /// let res = trasliterator.convert("фасад", false);
    /// assert_eq!("fasad", res);
    ///
    /// ```
    pub fn new(custom_rules: CharsMapping) -> Self {
        let mut table = custom_rules;
        fn compare_len(left: &str, right: &str) -> Ordering {
            if left.len() == right.len() {
                Ordering::Equal
            } else if left.len() > right.len() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
        // sort by Latin string
        table.sort_by(|a, b| compare_len(b.1, a.1));

        Self { rules: table }
    }

    /// Transliterate input string.
    pub fn convert(&self, src: &str, invert: bool) -> String {
        let mut input = src.to_owned();

        for elem in self.rules.iter() {
            let (source_char, translit_char) = (elem.0, elem.1);

            input = {
                if invert {
                    input.replace(translit_char, source_char)
                } else {
                    input.replace(source_char, translit_char)
                }
            }
        }

        input
    }
}

impl ToLatin for Transliterator {
    /// The wrapper on the method `convert` of transliteration in the Latin alphabet
    fn to_latin(&self, src: &str) -> String {
        self.convert(src, false)
    }
}

impl FromLatin for Transliterator {
    /// The wrapper on the method `convert` of transliteration from the Latin alphabet
    fn from_latin(&self, src: &str) -> String {
        self.convert(src, true)
    }
}

/// Cyrillic transliteration table.
/// implementation GOST 7.79 System B, modified ISO 9:1995.
/// more details:
/// [http://en.wikipedia.org/wiki/ISO_9](http://en.wikipedia.org/wiki/ISO_9).
///
/// Check the possibility of transliteration is carried out at compile time
///
/// # Examples
///
/// ```rust
///
/// use translit::{Gost779B, ToLatin, Language};
/// // transliteration GOST 7.79 System B
/// let trasliterator = Gost779B::new(Language::Ru);
/// let res = trasliterator.to_latin("Россия");
/// assert_eq!("Rossiya", res);
///
/// ```
pub struct Gost779B {
    translit: Transliterator,
}

impl Gost779B {
    pub fn new(lang: Language) -> Gost779B {
        let table = match lang {
            Language::Ru => gost779::gost779b_ru(),
            Language::By => gost779::gost779b_by(),
            Language::Ua => gost779::gost779b_ua(),
        };

        let translit = Transliterator::new(table);

        Gost779B { translit }
    }
}

impl ToLatin for Gost779B {
    fn to_latin(&self, src: &str) -> String {
        self.translit.to_latin(src)
    }
}

impl FromLatin for Gost779B {
    fn from_latin(&self, src: &str) -> String {
        self.translit.from_latin(src)
    }
}

/// Cyrillic Russian transliteration table.
/// implementation Passport (2013), ICAO.
/// more details:
/// [Romanization_of_Russian#After_2013](https://en.wikipedia.org/wiki/Romanization_of_Russian#After_2013)
///
/// Attention! Transliteration from Latin alphabet to Cyrillic text not supported.
/// In transliteration from the Cyrillic to the Latin alphabet excludes the letter `ь`.
/// Check the possibility of transliteration is carried out at compile time
///
/// # Examples
///
/// ```rust
///
/// use translit::{Passport2013, ToLatin};
/// // transliteration Passport (2013), ICAO
/// let trasliterator = Passport2013::new();
/// let res = trasliterator.to_latin("Москва, Тверская улица");
/// assert_eq!("Moskva, Tverskaia ulitsa", res);
///
/// ```
pub struct Passport2013 {
    translit: Transliterator,
}

impl Passport2013 {
    pub fn new() -> Passport2013 {
        let translit = Transliterator::new(passport2013::iternational_passport_2013_ru());

        Passport2013 { translit }
    }
}

impl ToLatin for Passport2013 {
    fn to_latin(&self, src: &str) -> String {
        self.translit.to_latin(src)
    }
}

/// Official system for transliterating Bulgarian
///
/// more details:
/// [Romanization of Bulgarian #Streamlined system](https://en.wikipedia.org/wiki/Romanization_of_Bulgarian#Streamlined_System)
///
/// Attention: Converting back from romanized cyrillic to cyrillic is ambiguous, thus not supported
///
/// # Examples
///
/// ```rust
///
/// use translit::{BulgarianOfficial, ToLatin};
/// let trasliterator = BulgarianOfficial::new();
/// let res = trasliterator.to_latin("Разни приказки");
/// assert_eq!("Razni prikazki", res);
///
/// ```
pub struct BulgarianOfficial {
    translit: Transliterator,
}

impl BulgarianOfficial {
    pub fn new() -> BulgarianOfficial {
        let translit = Transliterator::new(bulgarian::streamlined_system());

        BulgarianOfficial { translit }
    }
}

impl ToLatin for BulgarianOfficial {
    fn to_latin(&self, src: &str) -> String {
        self.translit.to_latin(src)
    }
}

/// Official system for transliterating Macedonian
///
/// more details:
/// [Romanization of Macedonian #Digraph system](https://en.wikipedia.org/wiki/Romanization_of_Macedonian#Digraph_system)
///
/// Attention: Converting back from romanized cyrillic to cyrillic is ambiguous, thus not supported
///
/// # Examples
///
/// ```rust
///
/// use translit::{MacedonianOfficial, ToLatin};
/// let trasliterator = MacedonianOfficial::new();
/// let res = trasliterator.to_latin("Надзор");
/// assert_eq!("Nadzor", res);
///
/// ```
pub struct MacedonianOfficial {
    translit: Transliterator,
}

impl MacedonianOfficial {
    pub fn new() -> MacedonianOfficial {
        let translit = Transliterator::new(macedonian::digraph_system());

        MacedonianOfficial { translit }
    }
}

impl ToLatin for MacedonianOfficial {
    fn to_latin(&self, src: &str) -> String {
        self.translit.to_latin(src)
    }
}
