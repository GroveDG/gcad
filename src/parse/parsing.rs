pub(crate) const fn take_while<'a>(
    mut f: impl FnMut(char) -> bool,
    min: usize,
    max: usize,
) -> impl FnMut(&mut &'a str) -> Option<&'a str> {
    move |input: &mut &'a str| {
        let mut chars = input.char_indices();
        let split = loop {
            if let Some((i, c)) = chars.next() {
                if f(c) {
                    continue;
                }
                break i;
            } else {
                break input.len();
            }
        };
        if split < min || split >= max {
            return None;
        }
        let result = &input[..split];
        *input = &input[split..];
        Some(result)
    }
}

pub(crate) fn space<'a>(input: &mut &'a str) -> Option<&'a str> {
    take_while(char::is_whitespace, 1, usize::MAX)(input)
}

pub(crate) fn word<'a>(input: &mut &'a str) -> Option<&'a str> {
    take_while(char::is_alphabetic, 1, usize::MAX)(input)
}

pub(crate) const fn literal<'a>(pattern: &'a str) -> impl Fn(&mut &'a str) -> Option<&'a str> {
    move |i: &mut &'a str| {
        *i = i.strip_prefix(pattern)?;
        Some(pattern)
    }
}
