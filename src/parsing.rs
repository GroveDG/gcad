use std::str::FromStr;

pub(crate) type Parsey<'a, O = &'a str> = Option<(&'a str, O)>;
pub(crate) type ParseyStr<'a> = Option<(&'a str, &'a str)>;

pub(crate) trait ParseInner<'a> {
    fn parse_inner<T: FromStr>(self) -> Parsey<'a, T>;
}

impl<'a> ParseInner<'a> for ParseyStr<'a> {
    fn parse_inner<T: FromStr>(self) -> Parsey<'a, T> {
        if let Some((i, o)) = self {
            o.parse().ok().map(|o| (i, o))
        } else {
            None
        }
    }
}

pub(crate) const fn take_while<'a>(
    mut f: impl FnMut(char) -> bool,
    min: usize,
    max: usize,
) -> impl FnMut(&'a str) -> ParseyStr<'a> {
    move |s: &str| {
        let mut split = 0;
        let mut chars = s.char_indices();
        loop {
            if let Some((i, c)) = chars.next() {
                if f(c) {
                    continue;
                }
                split = i;
                break;
            } else {
                split = s.len();
                break;
            }
        }
        if split < min || split >= max {
            return None;
        }
        Some((&s[split..], &s[..split]))
    }
}

pub(crate) const fn delimited_list<'a, O1, O2>(
    delimiter: impl Fn(&'a str) -> Parsey<'a, O1>,
    item: impl Fn(&'a str) -> Parsey<'a, O2>,
    min: usize,
    max: usize,
) -> impl Fn(&'a str) -> Parsey<'a, Vec<O2>> {
    move |mut i: &str| {
        let mut output = Vec::new();
        loop {
            if let Some((i_, o)) = item(i) {
                i = i_;
                output.push(o);
                if output.len() == max {
                    break;
                }
            } else {
                break;
            }
            if let Some((i_, _)) = delimiter(i) {
                i = i_;
            } else {
                break;
            }
        }
        if output.len() < min {
            return None;
        }
        return Some((i, output));
    }
}

pub(crate) const fn next_char<'a>(c: char) -> impl Fn(&'a str) -> Parsey<'a, char> {
    move |i: &str| {
        if let Some((i_, c_)) = i.char_indices().next() {
            if c_ == c {
                Some((&i[i_ + 1..], c_))
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub(crate) const fn next_str<'a>(s: &'a str) -> impl Fn(&'a str) -> ParseyStr<'a> {
    move |i: &'a str| {
        if s.len() > i.len() {
            return None;
        }
        if s == &i[..s.len()] {
            Some((&i[s.len()..], &i[..s.len()]))
        } else {
            None
        }
    }
}

pub(crate) const fn inner<'a, O1, O2, O3>(
    f: impl Fn(&'a str) -> Parsey<'a, O1>,
    g: impl Fn(&'a str) -> Parsey<'a, O2>,
    h: impl Fn(&'a str) -> Parsey<'a, O3>,
) -> impl Fn(&'a str) -> Parsey<'a, O2> {
    move |i: &'a str| {
        let Some((i, _)) = f(i) else { return None };
        let Some((i, o)) = g(i) else { return None };
        let Some((i, _)) = h(i) else { return None };
        Some((i, o))
    }
}

pub(crate) const fn after<'a, O1, O2>(
    f: impl Fn(&'a str) -> Parsey<'a, O1>,
    g: impl Fn(&'a str) -> Parsey<'a, O2>,
) -> impl Fn(&'a str) -> Parsey<'a, O2> {
    move |i: &'a str| {
        let Some((i, _)) = f(i) else { return None };
        let Some(o) = g(i) else { return None };
        Some(o)
    }
}

pub(crate) const fn before<'a, O1, O2>(
    f: impl Fn(&'a str) -> Parsey<'a, O1>,
    g: impl Fn(&'a str) -> Parsey<'a, O2>,
) -> impl Fn(&'a str) -> Parsey<'a, O1> {
    move |i: &'a str| {
        let Some((i, o)) = f(i) else { return None };
        let Some((i, _)) = g(i) else { return None };
        Some((i, o))
    }
}

pub(crate) const fn one_of<'a, O>(
    fs: Vec<impl Fn(&'a str) -> Parsey<'a, O>>,
) -> impl Fn(&'a str) -> Parsey<'a, O> {
    move |i: &'a str| {
        for f in fs.iter() {
            let o = f(i);
            if o.is_some() {
                return o;
            }
        }
        return None;
    }
}

pub(crate) fn opt_ws<'a>(s: &'a str) -> ParseyStr<'a> {
    take_while(char::is_whitespace, 0, usize::MAX)(s)
}

pub(crate) fn ws<'a>(s: &'a str) -> ParseyStr<'a> {
    take_while(char::is_whitespace, 1, usize::MAX)(s)
}

pub(crate) fn ident<'a>(s: &'a str) -> ParseyStr<'a> {
    take_while(char::is_alphabetic, 1, usize::MAX)(s)
}

pub(crate) fn flag<'a, O1, O2>(
    truthy: impl Fn(&'a str) -> Parsey<'a, O1>,
    falsy: impl Fn(&'a str) -> Parsey<'a, O2>,
) -> impl Fn(&'a str) -> Parsey<'a, bool> {
    move |s| {
        if let Some((i, _)) = truthy(s) {
            Some((i, true))
        } else if let Some((i, _)) = falsy(s) {
            Some((i, false))
        } else {
            None
        }
    }
}

pub(crate) fn opt_flag<'a, O>(
    truthy: impl Fn(&'a str) -> Parsey<'a, O>,
) -> impl Fn(&'a str) -> Parsey<'a, bool> {
    move |s| {
        if let Some((i, _)) = truthy(s) {
            Some((i, true))
        } else {
            Some((s, false))
        }
    }
}

pub(crate) const fn pair<'a, O1, O2>(
    f: impl Fn(&'a str) -> Parsey<'a, O1>,
    g: impl Fn(&'a str) -> Parsey<'a, O2>,
) -> impl Fn(&'a str) -> Parsey<'a, (O1, O2)> {
    move |i: &'a str| {
        let Some((i, o1)) = f(i) else { return None };
        let Some((i, o2)) = g(i) else { return None };
        Some((i, (o1, o2)))
    }
}
