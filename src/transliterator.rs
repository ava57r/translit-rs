use super::CharsMapping;

use std::cmp::Ordering;

use gost779;
use passport2013;

/// The contract for transliteration in the Latin alphabet
pub trait ToLatin {
    fn to_latin(&self, src: &str) -> String;
}

/// The contract for transliteration from Latin alphabet
pub trait FromLatin {
    fn from_latin(&self, src: &str) -> String;
}

/// This enum contains the available variants of transliteration
#[allow(non_camel_case_types)]
pub enum TranslitMethod {
    /// Cyrillic Russian transliteration table.
    /// implementation GOST 7.79 System B, modified ISO 9:1995.
    /// more details:
    /// [http://en.wikipedia.org/wiki/ISO_9](http://en.wikipedia.org/wiki/ISO_9).
    gost779b_ru,
    // Cyrillic Belarusian transliteration table.
    /// implementation GOST 7.79 System B, modified ISO 9:1995.
    /// more details:
    /// [http://en.wikipedia.org/wiki/ISO_9](http://en.wikipedia.org/wiki/ISO_9).
    gost779b_by,
    /// Cyrillic Ukrainian transliteration table.
    /// implementation GOST 7.79 System B, modified ISO 9:1995.
    /// more details:
    /// [http://en.wikipedia.org/wiki/ISO_9](http://en.wikipedia.org/wiki/ISO_9).
    gost779b_ua,
    /// Cyrillic Russian transliteration table.
    /// implementation Passport (2013), ICAO.
    /// more details:
    /// [Romanization_of_Russian#After_2013](https://en.wikipedia.org/wiki/Romanization_of_Russian#After_2013)
    ///
    /// Attention! Transliteration from Latin alphabet to Cyrillic text not supported.
    /// In transliteration from the Cyrillic to the Latin alphabet excludes the letter `ь`.
    iternational_passport_2013_ru,
}

/// The `Transliterator` struct allows for the transliteration
/// of the string of characters of Cyrillic alphabet UTF-8 to Latin alphabet
/// and back.
///
/// # Examples
///
/// ```rust
///
/// use translit::{TranslitMethod, Transliterator};
/// // transliteration GOST 7.79 System B
/// let trasliterator = Transliterator::new(TranslitMethod::gost779b_ru);
/// let res = trasliterator.convert("Россия", false);
/// assert_eq!("Rossiya", res);
///
/// ```
pub struct Transliterator {
    rules: CharsMapping,
}

impl Transliterator {
    /// Creates a new `Transliterator`
    pub fn new(method: TranslitMethod) -> Self {
        let mut table = match method {
            TranslitMethod::gost779b_ru => gost779::gost779b_ru(),
            TranslitMethod::gost779b_by => gost779::gost779b_by(),
            TranslitMethod::gost779b_ua => gost779::gost779b_ua(),
            TranslitMethod::iternational_passport_2013_ru => {
                passport2013::iternational_passport_2013_ru()
            }
        };

        // sort by Latin string
        table.sort_by(|a, b| compare_len(b.1, a.1));

        Self { rules: table }
    }

    /// Creates a new `Transliterator` with special transliteration table
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
    /// let trasliterator = Transliterator::from_custom_transliteration_table(table);
    /// let res = trasliterator.convert("фасад", false);
    /// assert_eq!("fasad", res);
    ///
    /// ```
    pub fn from_custom_transliteration_table(custom_rules: CharsMapping) -> Self {
        let mut table = custom_rules;

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

fn compare_len(left: &str, right: &str) -> Ordering {
    if left.len() == right.len() {
        Ordering::Equal
    } else if left.len() > right.len() {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

/// Support Languages
pub enum Language {
    Ru,
    By,
    Ua,
}

/// The wrapper on the `Transliterator::new(TranslitMethod::gost779b_*)`.
/// Check the possibility of transliteration is carried out at compile time
///
/// # Examples
///
/// ```rust
///
/// use translit::{Gost779, ToLatin, Language};
/// // transliteration GOST 7.79 System B
/// let trasliterator = Gost779::new(Language::Ru);
/// let res = trasliterator.to_latin("Россия");
/// assert_eq!("Rossiya", res);
///
/// ```
pub struct Gost779 {
    translit: Transliterator,
}

impl Gost779 {
    pub fn new(lang: Language) -> Gost779 {
        let method = match lang {
            Language::Ru => TranslitMethod::gost779b_ru,
            Language::By => TranslitMethod::gost779b_by,
            Language::Ua => TranslitMethod::gost779b_ua,
        };

        let translit = Transliterator::new(method);

        Gost779 { translit }
    }
}

impl ToLatin for Gost779 {
    fn to_latin(&self, src: &str) -> String {
        self.translit.to_latin(src)
    }
}

impl FromLatin for Gost779 {
    fn from_latin(&self, src: &str) -> String {
        self.translit.from_latin(src)
    }
}

/// The wrapper on the `Transliterator::new(TranslitMethod::iternational_passport_2013_ru)`.
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
        let translit = Transliterator::new(TranslitMethod::iternational_passport_2013_ru);

        Passport2013 { translit }
    }
}

impl ToLatin for Passport2013 {
    fn to_latin(&self, src: &str) -> String {
        self.translit.to_latin(src)
    }
}
