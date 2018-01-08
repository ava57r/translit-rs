mod gost779;
mod passport2013;

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

#[cfg(test)]
mod tests {
    use super::{TranslitMethod, Transliterator};

    // Russian
    const SOURCE_RU: &'static str =
        "Везувий зев открыл — дым хлынул клубом — пламя \
         Широко развилось, как боевое знамя. \
         Земля волнуется — с шатнувшихся колонн \
         Кумиры падают! Народ, гонимый страхом, \
         Толпами, стар и млад, под воспаленным прахом, \
         Под каменным дождем бежит из града вон.";

    const TRANSLIT_GOST779B_RU: &'static str =
        "Vezuvij zev otkry`l — dy`m xly`nul klubom — plamya \
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
            TRANSLIT_GOST779B_RU
        );
    }

    #[test]
    fn test_fn_to_latin_gost779b_ru_1() {
        assert_eq!(
            Transliterator::new(TranslitMethod::gost779b_ru).to_latin(SOURCE_RU),
            TRANSLIT_GOST779B_RU
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
            Transliterator::new(TranslitMethod::gost779b_ru).convert(TRANSLIT_GOST779B_RU, true),
            SOURCE_RU
        );
    }

    #[test]
    fn test_fn_from_latin_gost779b_ru_2() {
        assert_eq!(
            Transliterator::new(TranslitMethod::gost779b_ru).from_latin(TRANSLIT_GOST779B_RU),
            SOURCE_RU
        );
    }

    const SOURCE_PASSPORT_2013_RU_1: &'static str = "Большое преимущество получает тот, \
    кто достаточно рано сделал ошибки на которых можно учиться.© Уинстон Черчилль";

    const TRANSLIT_PASSPORT_2013_RU_1: &'static str =
        "Bolshoe preimushchestvo poluchaet tot, \
         kto dostatochno rano sdelal oshibki na kotorykh mozhno uchitsia.© Uinston Cherchill";

    #[test]
    fn test_fn_to_latin_iternational_passport_2013_ru_1() {
        assert_eq!(
            Transliterator::new(TranslitMethod::iternational_passport_2013_ru)
                .to_latin(SOURCE_PASSPORT_2013_RU_1),
            TRANSLIT_PASSPORT_2013_RU_1
        );
    }

    // Belarusian
    const SOURCE_BY: &'static str =
        "У рудога вераб'я ў сховішчы \
         пад фатэлем ляжаць нейкія гаючыя зёлкі.";

    const TRANSLIT_BY: &'static str = "U rudoha verab'ya u` sxovishchy` \
                                       pad fate`lem lyazhac` nejkiya hayuchy`ya zyolki.";

    #[test]
    fn test_fn_to_latin_gost779b_by_1() {
        assert_eq!(
            Transliterator::new(TranslitMethod::gost779b_by).to_latin(SOURCE_BY),
            TRANSLIT_BY
        );
    }

    #[test]
    fn test_fn_from_latin_gost779b_by_1() {
        assert_eq!(
            Transliterator::new(TranslitMethod::gost779b_by).from_latin(TRANSLIT_BY),
            SOURCE_BY
        );
    }

    // Ukrainian
    const SOURCE_UA: &'static str =
        "Гей, хлопці, не вспію - на ґанку \
         ваша файна їжа знищується бурундучком.";

    const TRANSLIT_UA: &'static str = "Gej, xlopci, ne vspiyu - na g`anku \
                                       vasha fajna yizha zny`shhuyet`sya burunduchkom.";

    #[test]
    fn test_fn_to_latin_gost779b_ua_1() {
        assert_eq!(
            Transliterator::new(TranslitMethod::gost779b_ua).to_latin(SOURCE_UA),
            TRANSLIT_UA
        );
    }

    #[test]
    fn test_fn_from_latin_gost779b_ua_1() {
        assert_eq!(
            Transliterator::new(TranslitMethod::gost779b_ua).from_latin(TRANSLIT_UA),
            SOURCE_UA
        );
    }
}
