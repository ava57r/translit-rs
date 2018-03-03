extern crate translit;

use translit::*;

fn main() {
    let v: CharsMapping = [
        (" ", "-"),
        (",", ""),
        (":", ""),
        (";", ""),
        ("№", ""),
        ("ь", ""),
        ("ъ", ""),
        ("Ы", "Y"),
        ("Э", "E"),
        ("ы", "y"),
        ("э", "e"),
    ].iter()
        .cloned()
        .collect();

    let mut custom_table = gost779b_ru();
    custom_table.retain(|&x| !x.1.contains("`"));
    custom_table.retain(|&x| !x.1.contains("#"));
    custom_table.extend(v.iter());

    let trn = Transliterator::new(custom_table);

    let source =
        "Общие вопросы по языку, получение помощи".to_lowercase();

    assert_eq!(
        "obshhie-voprosy-po-yazyku-poluchenie-pomoshhi",
        trn.to_latin(&source)
    );
}
