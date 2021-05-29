#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut bytes = Vec::<u8>::new();
    for values in values.iter() {
        let mut temp = Vec::<u8>::new();
        let mut t = *values;
        if t == 0 {
            bytes.push(0);
            continue;
        }
        while t > 0 {
            temp.insert(0, (t & 0b1111111) as u8);
            t >>= 7;
        }
        for index in 0..temp.len() {
            if index < temp.len() - 1 {
                temp[index] += 0b10000000;
            }
        }
        for v in temp.into_iter() {
            bytes.push(v);
        }
    }
    bytes
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut rel = Vec::<u32>::new();
    let mut sum = 0_u32;
    for value in bytes.iter() {
        let t = *value as u32;

        if t > 0x7f {
            if sum & 0x200_007f != 0 {
                return Err(Error::Overflow);
            }
            sum += t & 0x7f;
            sum <<= 7;
        } else {
            sum += t;
            // if sum & 0xffff_ffff != 0 {
            //     return Err(Error::Overflow);
            // }
            rel.push(sum);
            sum = 0;
        }
    }

    if rel.is_empty() {
        return Err(Error::IncompleteNumber);
    }

    Ok(rel)
}
