use super::CharsMapping;

/// Cyrillic Russian transliteration table for driver license.
/// more details:
/// [Order of the Ministry of Internal Affairs of the Russian Federation](https://normativ.kontur.ru/document?moduleId=1&documentId=388198)
///
/// Since April 2016, transliteration in driver's licenses is carried out in accordance with ICAO recommendations
///
/// Attention! Transliteration from Latin alphabet to Cyrillic text not supported.
/// In transliteration from the Cyrillic to the Latin alphabet excludes the letter `ь`.
pub fn order_n_995_ru() -> CharsMapping {
    [
        ("А", "A"),
        ("Б", "B"),
        ("В", "V"),
        ("Г", "G"),
        ("Д", "D"),
        ("Е", "E"),
        ("Ё", "E"),
        ("Ж", "Zh"),
        ("З", "Z"),
        ("И", "I"),
        ("Й", "I"),
        ("К", "K"),
        ("Л", "L"),
        ("М", "M"),
        ("Н", "N"),
        ("О", "O"),
        ("П", "P"),
        ("Р", "R"),
        ("С", "S"),
        ("Т", "T"),
        ("У", "U"),
        ("Ф", "F"),
        ("Х", "Kh"),
        ("Ц", "Ts"),
        ("Ч", "Ch"),
        ("Ш", "Sh"),
        ("Щ", "Shch"),
        ("Ы", "Y"),
        ("Ъ", "Ie"),
        ("Ь", ""),
        ("Э", "E"),
        ("Ю", "Iu"),
        ("Я", "Ia"),
        ("а", "a"),
        ("б", "b"),
        ("в", "v"),
        ("г", "g"),
        ("д", "d"),
        ("е", "e"),
        ("ё", "e"),
        ("ж", "zh"),
        ("з", "z"),
        ("и", "i"),
        ("й", "i"),
        ("к", "k"),
        ("л", "l"),
        ("м", "m"),
        ("н", "n"),
        ("о", "o"),
        ("п", "p"),
        ("р", "r"),
        ("с", "s"),
        ("т", "t"),
        ("у", "u"),
        ("ф", "f"),
        ("х", "kh"),
        ("ц", "ts"),
        ("ч", "ch"),
        ("ш", "sh"),
        ("щ", "shch"),
        ("ы", "y"),
        ("ъ", "ie"),
        ("ь", ""),
        ("э", "e"),
        ("ю", "iu"),
        ("я", "ia"),
    ]
    .iter()
    .cloned()
    .collect()
}
