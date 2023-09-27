pub(crate) struct BitWriter {
    pub res: Vec<u8>,
    pub bp : usize,
}

impl BitWriter {
    pub(crate) fn new() -> Self {
        Self {
            res: Vec::new(),
            bp : 0,
        }
    }

    pub(crate) fn write_bit(&mut self, b: bool) {
        if self.bp == 0 {
            self.res.push(0);
        }

        *self.res.last_mut().unwrap() |= (b as u8) << (7-self.bp);

        self.bp += 1;
        if self.bp == 8 {
            self.bp = 0;
        }
    }

    pub(crate) fn write_bits(&mut self, b: &Vec<bool>) {
        for i in b.iter() {
            self.write_bit(*i);
        }
    }

    pub(crate) fn trim_end_zeros(&mut self) {
        let mut trim_to = self.res.len();
        for (i, el) in self.res.iter().enumerate().rev() {
            if *el == 0 {
                trim_to = i;
            } else {
                break;
            }
        }
        self.res.truncate(trim_to)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct BitReader {
    pub buf: Vec<u8>,
    pub idx: usize,
    pub bp : u8,
}

impl BitReader {
    pub(crate) fn new(b: Vec<u8>) -> Self {
        Self {
            buf: b, idx: 0, bp: 0,
        }
    }

    pub(crate) fn next(&mut self) -> bool {
        let ret = self.buf.get(self.idx).unwrap_or(&0) >> (7-self.bp) & 1 != 0;

        self.bp  += 1;
        self.idx += self.bp as usize >> 3;
        self.bp  &= 0b111;

        ret
    }
}
