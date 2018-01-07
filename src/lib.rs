mod gost779;

use std::collections::HashMap;

pub type CharsMapping = HashMap<&'static str, &'static str>;

#[allow(non_camel_case_types)]
pub enum TranslitMethod {
    gost779b_ru,
    gost779b_by,
    gost779b_ua,
    iternational_passport_2013_ru,
    Custom,
}

pub struct Transliterator {
    rules: Option<CharsMapping>,
    translit_method: TranslitMethod,
}

impl Transliterator {
    pub fn new(method: TranslitMethod) -> Self {
        Transliterator {
            rules: None,
            translit_method: method,
        }
    }

    pub fn from_custom_transliteration_table(custom_rules: CharsMapping) -> Self {
        Transliterator {
            rules: Some(custom_rules),
            translit_method: TranslitMethod::Custom,
        }
    }

    pub fn convert(&self, src: &str, invert: bool) -> String {
        let mut input = src.to_owned();
        let table = match self.translit_method {
            TranslitMethod::gost779b_ru => gost779::gost779b_ru(),
            TranslitMethod::Custom => match self.rules {
                Some(ref v) => v.clone(),
                None => gost779::gost779b_ru(),
            },
            _ => unimplemented!(),
        };

        for (source_char, translit_char) in table.iter() {
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

#[cfg(test)]
mod tests {
    use super::{TranslitMethod, Transliterator};
    #[test]
    fn test_russian_to_latin_translit() {
        assert_eq!(
            Transliterator::new(TranslitMethod::gost779b_ru).convert("Терминал", false),
            "Terminal"
        );
    }

    #[test]
    fn test_latin_to_russian_translit() {
        assert_eq!(
            Transliterator::new(TranslitMethod::gost779b_ru).convert("Terminal", true),
            "Терминал"
        );
    }
}
