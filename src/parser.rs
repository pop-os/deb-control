use memchr::memchr;

#[derive(new)]
pub struct Control<'a> {
    source: &'a str,
}

#[derive(Debug, Display)]
pub enum Field<'a> {
    Single(&'a str),
    Folded(&'a str),
    Multiline(&'a str),
}

impl<'a> Iterator for Control<'a> {
    type Item = (&'a str, Field<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let (kend, vstart);
        let source = self.source.as_bytes();

        let mut vend = match memchr(b'\n', source) {
            Some(vend) => vend,
            None => {
                kend = memchr(b':', source)?;
                vstart = kend + 2;
                let key = &self.source[..kend];
                let value = Field::Single(&self.source[vstart..]);
                self.source = "";
                return Some((key, value));
            }
        };

        kend = memchr(b':', &source[..vend])?;
        vstart = kend + 2;
        let mut eof = false;

        let value = if kend == vend - 1 {
            vend += 1;

            let values = fetch_lines(source, vend);
            vend = values.0;
            eof = values.1;

            Field::Multiline(&self.source[vstart..vend])
        } else if is_space(source, vend + 1) {
            vend += 1;

            let values = fetch_lines(source, vend);
            vend = values.0;
            eof = values.1;

            Field::Folded(&self.source[vstart..vend])
        } else {
            Field::Single(&self.source[vstart..vend])
        };

        let key = &self.source[..kend];
        self.source = if eof { "" } else { &self.source[vend + 1..] };

        Some((key, value))
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
