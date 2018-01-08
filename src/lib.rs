mod gost779;

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
            _ => unimplemented!(),
        };

        // sort by Latin string
        table.sort_by(|a, b| b.1.cmp(a.1));

        Self { rules: table }
    }

    /// Creates a new `Transliterator` with special transliteration table
    pub fn from_custom_transliteration_table(custom_rules: CharsMapping) -> Self {
        let mut table = custom_rules;

        // sort by Latin string
        table.sort_by(|a, b| b.1.cmp(a.1));

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

#[cfg(test)]
mod tests {
    use super::{TranslitMethod, Transliterator};

    const SOURCE_RU: &'static str =
        "Везувий зев открыл — дым хлынул клубом — пламя \
         Широко развилось, как боевое знамя. \
         Земля волнуется — с шатнувшихся колонн \
         Кумиры падают! Народ, гонимый страхом, \
         Толпами, стар и млад, под воспаленным прахом, \
         Под каменным дождем бежит из града вон.";

    const TRANSLIT_RU: &'static str = "Vezuvij zev otkry`l — dy`m xly`nul klubom — plamya \
                                       Shiroko razvilos`, kak boevoe znamya. \
                                       Zemlya volnuetsya — s shatnuvshixsya kolonn \
                                       Kumiry` padayut! Narod, gonimy`j straxom, \
                                       Tolpami, star i mlad, pod vospalenny`m praxom, \
                                       Pod kamenny`m dozhdem bezhit iz grada von.";

    #[test]
    fn test_russian_to_latin_translit_gost779b_ru_1() {
        assert_eq!(
            Transliterator::new(TranslitMethod::gost779b_ru).convert("Терминал", false),
            "Terminal"
        );
    }

    #[test]
    fn test_russian_to_latin_translit_gost779b_ru_2() {
        assert_eq!(
            Transliterator::new(TranslitMethod::gost779b_ru).convert(SOURCE_RU, false),
            TRANSLIT_RU
        );
    }

    #[test]
    fn test_fn_to_latin_gost779b_ru_1() {
        assert_eq!(
            Transliterator::new(TranslitMethod::gost779b_ru).to_latin(SOURCE_RU),
            TRANSLIT_RU
        );
    }

    #[test]
    fn test_latin_to_russian_translit_gost779b_ru_1() {
        assert_eq!(
            Transliterator::new(TranslitMethod::gost779b_ru).convert("Terminal", true),
            "Терминал"
        );
    }

    #[test]
    fn test_latin_to_russian_translit_gost779b_ru_2() {
        assert_eq!(
            Transliterator::new(TranslitMethod::gost779b_ru).convert(TRANSLIT_RU, true),
            SOURCE_RU
        );
    }

    #[test]
    fn test_fn_from_latin_gost779b_ru12() {
        assert_eq!(
            Transliterator::new(TranslitMethod::gost779b_ru).from_latin(TRANSLIT_RU),
            SOURCE_RU
        );
    }

}
