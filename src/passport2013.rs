use super::CharsMapping;

/// Cyrillic Russian transliteration table.
/// implementation Passport (2013), ICAO.
/// more details:
/// [Romanization_of_Russian#After_2013](https://en.wikipedia.org/wiki/Romanization_of_Russian#After_2013)
///
/// Attention! Transliteration from Latin alphabet to Cyrillic text not supported.
/// In transliteration from the Cyrillic to the Latin alphabet excludes the letter `ь`.
pub fn iternational_passport_2013_ru() -> CharsMapping {
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
        ("Ъ", "Ie"),
        ("Ы", "Y"),
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
        ("ъ", "ie"),
        ("ы", "y"),
        ("ь", ""),
        ("э", "e"),
        ("ю", "iu"),
        ("я", "ia"),
        ("№", "#"),
    ]
    .iter()
    .cloned()
    .collect()
}
