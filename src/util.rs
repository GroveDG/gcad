pub(crate) fn locate<'a, T: PartialEq>(
    iter: impl IntoIterator<Item = &'a T>,
    item: &'a T,
) -> Option<usize> {
    for (i, t) in iter.into_iter().enumerate() {
        if t == item {
            return Some(i);
        }
    }
    None
}

pub fn print_header(s: &str) {
    println!(
        "\n\n{}\n",
        ansi_term::Style::new().underline().paint(
            [
                s,
                " ".repeat(term_size::dimensions().unwrap().0 - s.len())
                    .as_str(),
            ]
            .concat()
        )
    );
}
