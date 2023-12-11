use std::collections::HashMap;

pub fn elf_counter(input_str: String) -> HashMap<&'static str, usize> {
    let n_elves_on_shelves = input_str.matches("elf").count();
    HashMap::from([
        ("elf", input_str.matches("elf").count()),
        ("elf on a shelf", n_elves_on_shelves),
        ("shelf with no elf on it", input_str.matches("shelf").count() - n_elves_on_shelves)
    ])
}