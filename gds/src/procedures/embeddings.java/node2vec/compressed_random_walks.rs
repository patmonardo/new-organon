use crate::collections::{HugeIntArray, HugeObjectArray};

/// ZigZag encode signed i64 to unsigned u64.
#[inline]
fn zigzag_encode_i64(value: i64) -> u64 {
    ((value << 1) ^ (value >> 63)) as u64
}

/// ZigZag decode unsigned u64 to signed i64.
#[inline]
fn zigzag_decode_u64(value: u64) -> i64 {
    ((value >> 1) as i64) ^ (-((value & 1) as i64))
}

/// Encode a u64 as LEB128/varint.
fn encode_var_u64(mut value: u64, out: &mut Vec<u8>) {
    while value >= 0x80 {
        out.push((value as u8) | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
}

/// Decode a u64 varint from `data` starting at `offset`, returning (value, new_offset).
fn decode_var_u64(data: &[u8], mut offset: usize) -> Option<(u64, usize)> {
    let mut result: u64 = 0;
    let mut shift = 0;
    for _ in 0..10 {
        let byte = *data.get(offset)?;
        offset += 1;

        result |= ((byte & 0x7F) as u64) << shift;
        if (byte & 0x80) == 0 {
            return Some((result, offset));
        }
        shift += 7;
    }
    None
}

/// Stores random walks in a compressed delta+zigzag+varint encoding.
///
/// Java: `CompressedRandomWalks`
pub struct CompressedRandomWalks {
    compressed_walks: HugeObjectArray<Vec<u8>>,
    walk_lengths: HugeIntArray,
    max_walk_length: usize,
    size: usize,
}

impl CompressedRandomWalks {
    pub fn new(max_walk_count: usize) -> Self {
        Self {
            compressed_walks: HugeObjectArray::new(max_walk_count),
            walk_lengths: HugeIntArray::new(max_walk_count),
            max_walk_length: 0,
            size: 0,
        }
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn set_max_walk_length(&mut self, max_walk_length: usize) {
        self.max_walk_length = max_walk_length;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn walk_length(&self, index: usize) -> usize {
        self.walk_lengths.get(index) as usize
    }

    /// Add a walk at the given index.
    ///
    /// Input node IDs must be non-negative (mapped IDs).
    pub fn add(&mut self, current_index: usize, walk: &[i64]) {
        let mut prev = 0i64;
        let mut compressed = Vec::new();
        for &node in walk {
            let delta = node - prev;
            prev = node;
            let encoded = zigzag_encode_i64(delta);
            encode_var_u64(encoded, &mut compressed);
        }

        self.compressed_walks.set(current_index, compressed);
        self.walk_lengths
            .set(current_index, walk.len() as i32);
    }

    /// Create a walk iterator over a contiguous chunk.
    pub fn iterator(&self, start_index: usize, length: usize) -> CompressedWalkIterator<'_> {
        let end_index = start_index + length - 1;
        if start_index >= self.size || end_index >= self.size {
            panic!(
                "Requested iterator chunk exceeds the number of stored random walks. Requested {}-{}, actual size {}",
                start_index,
                end_index,
                self.size
            );
        }

        CompressedWalkIterator::new(
            start_index,
            end_index,
            &self.compressed_walks,
            &self.walk_lengths,
            self.max_walk_length,
        )
    }
}

/// Iterator that yields decompressed walks using a reused output buffer.
///
/// Mirrors Java's `CompressedWalkIterator` reuse semantics.
pub struct CompressedWalkIterator<'a> {
    current_index: usize,
    end_index: usize,
    compressed_walks: &'a HugeObjectArray<Vec<u8>>,
    walk_lengths: &'a HugeIntArray,
    output_buffer: Vec<i64>,
}

impl<'a> CompressedWalkIterator<'a> {
    fn new(
        start_index: usize,
        end_index: usize,
        compressed_walks: &'a HugeObjectArray<Vec<u8>>,
        walk_lengths: &'a HugeIntArray,
        max_walk_length: usize,
    ) -> Self {
        Self {
            current_index: start_index,
            end_index,
            compressed_walks,
            walk_lengths,
            output_buffer: vec![-1; max_walk_length],
        }
    }

    /// Returns the next walk slice. The returned slice is backed by an internal buffer
    /// reused on subsequent calls and must not be retained.
    pub fn next_walk(&mut self) -> Option<&[i64]> {
        if self.current_index > self.end_index {
            return None;
        }

        let compressed = self.compressed_walks.get(self.current_index);
        let walk_len = self.walk_lengths.get(self.current_index) as usize;

        // Fill with -1 padding.
        for v in self.output_buffer.iter_mut() {
            *v = -1;
        }

        let mut offset = 0usize;
        let mut prev = 0i64;
        for i in 0..walk_len {
            let (encoded, new_offset) =
                decode_var_u64(compressed.as_slice(), offset).expect("invalid varint stream");
            offset = new_offset;
            let delta = zigzag_decode_u64(encoded);
            let value = prev + delta;
            prev = value;
            self.output_buffer[i] = value;
        }

        self.current_index += 1;
        Some(self.output_buffer.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_single_walk() {
        let mut walks = CompressedRandomWalks::new(10);
        walks.set_max_walk_length(8);
        walks.set_size(1);

        let walk = vec![3i64, 10, 11, 1000];
        walks.add(0, &walk);

        let mut it = walks.iterator(0, 1);
        let decoded = it.next_walk().unwrap();
        assert_eq!(&decoded[..walk.len()], walk.as_slice());
        assert_eq!(decoded[walk.len()], -1);
    }
}


