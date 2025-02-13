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

pub fn print_heading(s: &str) {
    let style = { ansi_term::Style::new().underline() };
    println!(
        "\n\n{}\n",
        style.paint(
            [
                s,
                " ".repeat(term_size::dimensions().unwrap().0 - s.len())
                    .as_str(),
            ]
            .concat()
        )
    );
}
