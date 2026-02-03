use memchr::memchr;

pub struct Control<'a> {
    source: &'a str,
}

impl<'a> Control<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }
}

#[derive(Debug)]
pub enum Kind {
    Single,
    Folded,
    Multiline,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Kind::Single => "Single",
            Kind::Folded => "Folded",
            Self::Multiline => "Multiline",
        })
    }
}

#[derive(Debug)]
pub struct Entry<'a> {
    pub key: &'a str,
    pub value: &'a str,
    pub kind: Kind,
}

impl<'a> Iterator for Control<'a> {
    type Item = Entry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (kend, vstart);
        let source = self.source.as_bytes();

        let mut vend = match memchr(b'\n', source) {
            Some(vend) => vend,
            None => {
                kend = memchr(b':', source)?;
                vstart = kend + 2;
                let key = &self.source[..kend];
                let value = &self.source[vstart..];
                self.source = "";
                return Some(Entry {
                    key,
                    value,
                    kind: Kind::Single,
                });
            }
        };

        kend = memchr(b':', &source[..vend])?;
        vstart = kend + 2;
        let mut eof = false;

        let (value, kind) = if kend == vend - 1 {
            vend += 1;

            let values = fetch_lines(source, vend);
            vend = values.0;
            eof = values.1;

            (&self.source[vstart..vend], Kind::Multiline)
        } else if is_space(source, vend + 1) {
            vend += 1;

            let values = fetch_lines(source, vend);
            vend = values.0;
            eof = values.1;

            (&self.source[vstart..vend], Kind::Folded)
        } else {
            (&self.source[vstart..vend], Kind::Single)
        };

        let key = &self.source[..kend];
        self.source = if eof { "" } else { &self.source[vend + 1..] };

        Some(Entry { key, value, kind })
    }
}

fn is_space(bytes: &[u8], pos: usize) -> bool {
    bytes
        .get(pos)
        .map_or(false, |&byte| byte == b' ' || byte == b'\t')
}

fn next_line(bytes: &[u8], start: usize) -> (usize, bool) {
    match memchr(b'\n', &bytes[start..]) {
        Some(pos) => (start + pos + 1, false),
        None => (bytes.len(), true),
    }
}

fn fetch_lines(source: &[u8], mut vend: usize) -> (usize, bool) {
    let mut eof = false;
    let (newv, was_eof) = next_line(source, vend);

    vend = newv;
    if was_eof {
        eof = true;
    }

    while is_space(&source, vend) {
        let (newv, was_eof) = next_line(source, vend);

        vend = newv;
        if was_eof {
            eof = true;
        }
    }

    if !eof {
        vend -= 1;
    }

    (vend, eof)
}
