use std::collections::HashMap;

use fancy_regex::Regex;

pub fn elf_counter(input_str: String) -> HashMap<&'static str, usize> {
    let n_elves_on_shelves = Regex::new("(?=(elf on a shelf))").unwrap().find_iter(&input_str).count();
    HashMap::from([
        ("elf", input_str.matches("elf").count()),
        ("elf on a shelf", n_elves_on_shelves),
        ("shelf with no elf on it", input_str.matches("shelf").count().saturating_sub(n_elves_on_shelves))
    ])
}