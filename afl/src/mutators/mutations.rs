use crate::inputs::{HasBytesVec, Input};
use crate::mutators::Corpus;
use crate::mutators::*;
use crate::utils::Rand;
use crate::AflError;

pub enum MutationResult {
    Mutated,
    Skipped,
}

// TODO maybe the mutator arg is not needed
/// The generic function type that identifies mutations
pub type MutationFunction<M, C, I, R> =
    fn(&mut M, &mut R, &C, &mut I) -> Result<MutationResult, AflError>;

pub trait ComposedByMutations<C, I, R>
where
    C: Corpus<I, R>,
    I: Input,
    R: Rand,
{
    /// Get a mutation by index
    fn mutation_by_idx(&self, index: usize) -> MutationFunction<Self, C, I, R>;

    /// Get the number of mutations
    fn mutations_count(&self) -> usize;

    /// Add a mutation
    fn add_mutation(&mut self, mutation: MutationFunction<Self, C, I, R>);
}

const ARITH_MAX: u64 = 35;

const INTERESTING_8: [i8; 9] = [-128, -1, 0, 1, 16, 32, 64, 100, 127];
const INTERESTING_16: [i16; 19] = [
    -128, -1, 0, 1, 16, 32, 64, 100, 127, -32768, -129, 128, 255, 256, 512, 1000, 1024, 4096, 32767,
];
const INTERESTING_32: [i32; 27] = [
    -128,
    -1,
    0,
    1,
    16,
    32,
    64,
    100,
    127,
    -32768,
    -129,
    128,
    255,
    256,
    512,
    1000,
    1024,
    4096,
    32767,
    -2147483648,
    -100663046,
    -32769,
    32768,
    65535,
    65536,
    100663045,
    2147483647,
];

fn self_mem_move(data: &mut [u8], from: usize, to: usize, len: usize) {
    debug_assert!(from + len <= data.len());
    debug_assert!(to + len <= data.len());
    let ptr = data.as_mut_ptr();
    unsafe { core::ptr::copy(ptr.offset(from as isize), ptr.offset(to as isize), len) }
}

fn mem_move(dst: &mut [u8], src: &[u8], from: usize, to: usize, len: usize) {
    debug_assert!(from + len <= src.len());
    debug_assert!(to + len <= dst.len());
    let dst_ptr = dst.as_mut_ptr();
    let src_ptr = src.as_ptr();
    unsafe {
        core::ptr::copy(
            src_ptr.offset(from as isize),
            dst_ptr.offset(to as isize),
            len,
        )
    }
}

fn mem_set(data: &mut [u8], from: usize, len: usize, val: u8) {
    debug_assert!(from + len <= data.len());
    let ptr = data.as_mut_ptr();
    unsafe { core::ptr::write_bytes(ptr.offset(from as isize), val, len) }
}

/// Bitflip mutation for inputs with a bytes vector
pub fn mutation_bitflip<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    if input.bytes().len() == 0 {
        Ok(MutationResult::Skipped)
    } else {
        let bit = rand.below((input.bytes().len() << 3) as u64) as usize;
        unsafe {
            // moar speed, no bound check
            *input.bytes_mut().get_unchecked_mut(bit >> 3) ^= (128 >> (bit & 7)) as u8;
        }
        Ok(MutationResult::Mutated)
    }
}

pub fn mutation_byteflip<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    if input.bytes().len() == 0 {
        Ok(MutationResult::Skipped)
    } else {
        let idx = rand.below(input.bytes().len() as u64) as usize;
        unsafe {
            // moar speed, no bound check
            *input.bytes_mut().get_unchecked_mut(idx) ^= 0xff;
        }
        Ok(MutationResult::Mutated)
    }
}

pub fn mutation_byteinc<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    if input.bytes().len() == 0 {
        Ok(MutationResult::Skipped)
    } else {
        let idx = rand.below(input.bytes().len() as u64) as usize;
        unsafe {
            // moar speed, no bound check
            *input.bytes_mut().get_unchecked_mut(idx) += 1;
        }
        Ok(MutationResult::Mutated)
    }
}

pub fn mutation_bytedec<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    if input.bytes().len() == 0 {
        Ok(MutationResult::Skipped)
    } else {
        let idx = rand.below(input.bytes().len() as u64) as usize;
        unsafe {
            // moar speed, no bound check
            *input.bytes_mut().get_unchecked_mut(idx) -= 1;
        }
        Ok(MutationResult::Mutated)
    }
}

pub fn mutation_byteneg<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    if input.bytes().len() == 0 {
        Ok(MutationResult::Skipped)
    } else {
        let idx = rand.below(input.bytes().len() as u64) as usize;
        unsafe {
            // moar speed, no bound check
            *input.bytes_mut().get_unchecked_mut(idx) = !(*input.bytes().get_unchecked(idx));
        }
        Ok(MutationResult::Mutated)
    }
}

pub fn mutation_byterand<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    if input.bytes().len() == 0 {
        Ok(MutationResult::Skipped)
    } else {
        let idx = rand.below(input.bytes().len() as u64) as usize;
        unsafe {
            // moar speed, no bound check
            *input.bytes_mut().get_unchecked_mut(idx) = rand.below(256) as u8;
        }
        Ok(MutationResult::Mutated)
    }
}

pub fn mutation_byteadd<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    if input.bytes().len() <= 1 {
        Ok(MutationResult::Skipped)
    } else {
        let idx = rand.below(input.bytes().len() as u64 - 1) as usize;
        unsafe {
            // moar speed, no bound check
            let ptr = input.bytes_mut().get_unchecked_mut(idx) as *mut u8;
            let num = 1 + rand.below(ARITH_MAX) as u8;
            match rand.below(2) {
                0 => *ptr += num,
                _ => *ptr -= num,
            };
        }
        Ok(MutationResult::Mutated)
    }
}

pub fn mutation_wordadd<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    if input.bytes().len() <= 1 {
        Ok(MutationResult::Skipped)
    } else {
        let idx = rand.below(input.bytes().len() as u64 - 1) as usize;
        unsafe {
            // moar speed, no bound check
            let ptr = input.bytes_mut().get_unchecked_mut(idx) as *mut _ as *mut u16;
            let num = 1 + rand.below(ARITH_MAX) as u16;
            match rand.below(4) {
                0 => *ptr += num,
                1 => *ptr -= num,
                2 => *ptr = ((*ptr).swap_bytes() + num).swap_bytes(),
                _ => *ptr = ((*ptr).swap_bytes() - num).swap_bytes(),
            };
        }
        Ok(MutationResult::Mutated)
    }
}

pub fn mutation_dwordadd<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    if input.bytes().len() <= 1 {
        Ok(MutationResult::Skipped)
    } else {
        let idx = rand.below(input.bytes().len() as u64 - 1) as usize;
        unsafe {
            // moar speed, no bound check
            let ptr = input.bytes_mut().get_unchecked_mut(idx) as *mut _ as *mut u32;
            let num = 1 + rand.below(ARITH_MAX) as u32;
            match rand.below(4) {
                0 => *ptr += num,
                1 => *ptr -= num,
                2 => *ptr = ((*ptr).swap_bytes() + num).swap_bytes(),
                _ => *ptr = ((*ptr).swap_bytes() - num).swap_bytes(),
            };
        }
        Ok(MutationResult::Mutated)
    }
}

pub fn mutation_qwordadd<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    if input.bytes().len() <= 1 {
        Ok(MutationResult::Skipped)
    } else {
        let idx = rand.below(input.bytes().len() as u64 - 1) as usize;
        unsafe {
            // moar speed, no bound check
            let ptr = input.bytes_mut().get_unchecked_mut(idx) as *mut _ as *mut u64;
            let num = 1 + rand.below(ARITH_MAX) as u64;
            match rand.below(4) {
                0 => *ptr += num,
                1 => *ptr -= num,
                2 => *ptr = ((*ptr).swap_bytes() + num).swap_bytes(),
                _ => *ptr = ((*ptr).swap_bytes() - num).swap_bytes(),
            };
        }
        Ok(MutationResult::Mutated)
    }
}

pub fn mutation_byteinteresting<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    if input.bytes().len() <= 1 {
        Ok(MutationResult::Skipped)
    } else {
        let idx = rand.below(input.bytes().len() as u64 - 1) as usize;
        let val = INTERESTING_8[rand.below(INTERESTING_8.len() as u64) as usize] as u8;
        unsafe {
            // moar speed, no bound check
            *input.bytes_mut().get_unchecked_mut(idx) = val;
        }
        Ok(MutationResult::Mutated)
    }
}

pub fn mutation_wordinteresting<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    if input.bytes().len() <= 1 {
        Ok(MutationResult::Skipped)
    } else {
        let idx = rand.below(input.bytes().len() as u64 - 1) as usize;
        let val = INTERESTING_16[rand.below(INTERESTING_8.len() as u64) as usize] as u16;
        unsafe {
            // moar speed, no bound check
            let ptr = input.bytes_mut().get_unchecked_mut(idx) as *mut _ as *mut u16;
            if rand.below(2) == 0 {
                *ptr = val;
            } else {
                *ptr = val.swap_bytes();
            }
        }
        Ok(MutationResult::Mutated)
    }
}

pub fn mutation_dwordinteresting<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    if input.bytes().len() <= 1 {
        Ok(MutationResult::Skipped)
    } else {
        let idx = rand.below(input.bytes().len() as u64 - 1) as usize;
        let val = INTERESTING_32[rand.below(INTERESTING_8.len() as u64) as usize] as u32;
        unsafe {
            // moar speed, no bound check
            let ptr = input.bytes_mut().get_unchecked_mut(idx) as *mut _ as *mut u32;
            if rand.below(2) == 0 {
                *ptr = val;
            } else {
                *ptr = val.swap_bytes();
            }
        }
        Ok(MutationResult::Mutated)
    }
}

pub fn mutation_bytesdelete<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    let size = input.bytes().len();
    if size <= 2 {
        return Ok(MutationResult::Skipped);
    }

    let off = rand.below(size as u64) as usize;
    let len = rand.below((size - off) as u64) as usize;
    input.bytes_mut().drain(off..len);

    Ok(MutationResult::Mutated)
}

pub fn mutation_bytesexpand<M, C, I, R>(
    mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R> + HasMaxSize,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    let size = input.bytes().len();
    let off = if size == 0 {
        0
    } else {
        rand.below(size as u64 - 1)
    } as usize;
    let len = rand.below(core::cmp::min(16, mutator.max_size() as u64)) as usize;

    input.bytes_mut().resize(size + len, 0);
    self_mem_move(input.bytes_mut(), off, off + len, len);
    Ok(MutationResult::Mutated)
}

pub fn mutation_bytesinsert<M, C, I, R>(
    mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R> + HasMaxSize,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    let size = input.bytes().len();
    let off = if size == 0 {
        0
    } else {
        rand.below(size as u64 - 1)
    } as usize;
    let len = rand.below(core::cmp::min(16, mutator.max_size() as u64)) as usize;

    let val = input.bytes()[rand.below(size as u64) as usize];
    input.bytes_mut().resize(size + len, 0);
    self_mem_move(input.bytes_mut(), off, off + len, len);
    mem_set(input.bytes_mut(), off, len, val);
    Ok(MutationResult::Mutated)
}

pub fn mutation_bytesrandinsert<M, C, I, R>(
    mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R> + HasMaxSize,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    let size = input.bytes().len();
    let off = if size == 0 {
        0
    } else {
        rand.below(size as u64 - 1)
    } as usize;
    let len = rand.below(core::cmp::min(16, mutator.max_size() as u64)) as usize;

    let val = rand.below(256) as u8;
    input.bytes_mut().resize(size + len, 0);
    self_mem_move(input.bytes_mut(), off, off + len, len);
    mem_set(input.bytes_mut(), off, len, val);
    Ok(MutationResult::Mutated)
}

pub fn mutation_bytesset<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    let size = input.bytes().len();
    if size == 0 {
        return Ok(MutationResult::Skipped);
    }

    let val = input.bytes()[rand.below(size as u64) as usize];
    let start = if size == 1 {
        0
    } else {
        rand.below(size as u64 - 1) as usize
    };
    let end = rand.below((size - start) as u64) as usize;
    mem_set(input.bytes_mut(), start, end - start, val);
    Ok(MutationResult::Mutated)
}

pub fn mutation_bytesrandset<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    let size = input.bytes().len();
    if size == 0 {
        return Ok(MutationResult::Skipped);
    }

    let val = rand.below(256) as u8;
    let start = if size == 1 {
        0
    } else {
        rand.below(size as u64 - 1) as usize
    };
    let end = rand.below((size - start) as u64) as usize;
    mem_set(input.bytes_mut(), start, end - start, val);
    Ok(MutationResult::Mutated)
}

pub fn mutation_bytescopy<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    let size = input.bytes().len();
    if size <= 1 {
        return Ok(MutationResult::Skipped);
    }

    let from = rand.below(input.bytes().len() as u64 - 1) as usize;
    let to = rand.below(input.bytes().len() as u64 - 1) as usize;
    let len = rand.below((size - core::cmp::max(from, to)) as u64) as usize;

    self_mem_move(input.bytes_mut(), from, to, len);
    Ok(MutationResult::Mutated)
}

pub fn mutation_bytesswap<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    _corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    let size = input.bytes().len();
    if size <= 1 {
        return Ok(MutationResult::Skipped);
    }

    let first = rand.below(input.bytes().len() as u64 - 1) as usize;
    let second = rand.below(input.bytes().len() as u64 - 1) as usize;
    let len = rand.below((size - core::cmp::max(first, second)) as u64) as usize;

    let tmp = input.bytes()[first..len].to_vec();
    self_mem_move(input.bytes_mut(), second, first, len);
    mem_move(input.bytes_mut(), &tmp, 0, second, len);
    Ok(MutationResult::Mutated)
}

/// Returns the first and last diff position between the given vectors, stopping at the min len
fn locate_diffs(this: &[u8], other: &[u8]) -> (i64, i64) {
    let mut first_diff: i64 = -1;
    let mut last_diff: i64 = -1;
    for (i, (this_el, other_el)) in this.iter().zip(other.iter()).enumerate() {
        if this_el != other_el {
            if first_diff < 0 {
                first_diff = i as i64;
            }
            last_diff = i as i64;
        }
    }

    (first_diff, last_diff)
}

/// Splicing mutator
pub fn mutation_splice<M, C, I, R>(
    _mutator: &mut M,
    rand: &mut R,
    corpus: &C,
    input: &mut I,
) -> Result<MutationResult, AflError>
where
    M: Mutator<C, I, R>,
    C: Corpus<I, R>,
    I: Input + HasBytesVec,
    R: Rand,
{
    // We don't want to use the testcase we're already using for splicing
    let (other_testcase, _) = corpus.random_entry(rand)?.clone();
    // TODO: Load let other = Testcase::load_from_disk(other_test)?;
    // println!("Input: {:?}, other input: {:?}", input.bytes(), other.bytes());
    let other = match other_testcase.input() {
        Some(i) => i,
        None => return Ok(MutationResult::Skipped), //TODO
    };

    let mut counter = 0;
    let (first_diff, last_diff) = loop {
        let (f, l) = locate_diffs(input.bytes(), other.bytes());
        // println!("Diffs were between {} and {}", f, l);
        if f != l && f >= 0 && l >= 2 {
            break (f, l);
        }
        if counter == 3 {
            return Ok(MutationResult::Skipped);
        }
        counter += 1;
    };

    let split_at = rand.between(first_diff as u64, last_diff as u64) as usize;

    // println!("Splicing at {}", split_at);

    input
        .bytes_mut()
        .splice(split_at.., other.bytes()[split_at..].iter().cloned());

    // println!("Splice result: {:?}, input is now: {:?}", split_result, input.bytes());

    Ok(MutationResult::Mutated)
}