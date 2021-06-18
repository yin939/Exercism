use std::{
    borrow::Borrow,
    io::{Read, Result, Write},
};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    index: usize,
    len: usize,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a>
    where
        Key: AsRef<[u8]> + ?Sized,
    {
        let key = key.as_ref();
        Self {
            key,
            index: 0,
            len: key.len(),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut()
            .for_each(|val| *val = *val ^ self.xor_by_step())
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8>
    where
        Data: IntoIterator,
        Data::Item: Borrow<u8>,
    {
        data.into_iter()
            .map(|x| *x.borrow() ^ self.xor_by_step())
            .collect::<Vec<u8>>()
            .into_iter()
    }

    fn xor_by_step(&mut self) -> u8 {
        let rel = self.key[self.index];

        self.index = (self.index + 1) % self.len;

        rel
    }

    pub fn reader<'i, Input>(self, input: Input) -> Reader<'a, Input>
    where
        Input: Read + 'i,
    {
        Reader::new(self, input)
    }

    pub fn writer<'i, Input>(self, input: Input) -> Writer<'a, Input>
    where
        Input: Write + 'i,
    {
        Writer::new(self, input)
    }
}

pub struct Reader<'key, Input> {
    xorcism: Xorcism<'key>,
    buf: Input,
}

impl<'key, Input> Reader<'key, Input>
where
    Input: Read,
{
    pub fn new(xorcism: Xorcism<'key>, buf: Input) -> Self {
        Reader { xorcism, buf }
    }
}

impl<Input> Read for Reader<'_, Input>
where
    Input: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.buf.read(buf);
        self.xorcism.munge_in_place(buf);
        result
    }
}

pub struct Writer<'key, Input> {
    xorcism: Xorcism<'key>,
    buf: Input,
}

impl<'key, Input> Writer<'key, Input>
where
    Input: Write,
{
    pub fn new(xorcism: Xorcism<'key>, buf: Input) -> Self {
        Writer { xorcism, buf }
    }
}

impl<Input> Write for Writer<'_, Input>
where
    Input: Write,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut new_buf = buf.to_vec();
        self.xorcism.munge_in_place(&mut new_buf);
        self.buf.write(&new_buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.buf.flush()
    }
}
