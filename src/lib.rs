mod gost779;
mod passport2013;

#[cfg(test)]
mod tests;

use std::cmp::Ordering;

pub type CharsMapping = Vec<(&'static str, &'static str)>;

#[allow(non_camel_case_types)]
pub enum TranslitMethod {
    gost779b_ru,
    gost779b_by,
    gost779b_ua,
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
    /// use translit::{TranslitMethod, Transliterator, CharsMapping};
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

    /// The wrapper on the method `convert` of transliteration in the Latin alphabet
    pub fn to_latin(&self, src: &str) -> String {
        self.convert(src, false)
    }

    /// The wrapper on the method `convert` of transliteration from the Latin alphabet
    pub fn from_latin(&self, src: &str) -> String {
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
