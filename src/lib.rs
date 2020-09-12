pub fn radix_sort<T : Copy + Into<u64>>(input : &mut Vec<T>)
{
    const RADIX      : u64   = 8;
    const ITERATIONS : u64   = 64 / RADIX;
    const BUCKETS    : usize = 256;
    const MASK       : u64   = BUCKETS as u64 - 1;

    debug_assert!(RADIX * ITERATIONS == 64);
    debug_assert!(ITERATIONS % 2 == 0);
    debug_assert!(BUCKETS == 2usize.pow(RADIX as u32));

    let mut buckets = Vec::with_capacity(BUCKETS);
    let mut output  = Vec::with_capacity(input.len());

    unsafe
    {
        buckets.set_len(BUCKETS);
        output.set_len(input.len());
    }

    for shift in (0 .. ITERATIONS).map(|i| i * RADIX)
    {
        buckets.iter_mut().for_each(|x| *x = 0);
        input.iter().for_each(|k| buckets[(Into::<u64>::into(*k) >> shift & MASK) as usize] += 1);
        buckets.iter_mut().scan(0, |a, x| { *a += *x; *x = *a; Some(()) }).last();

        for k in input.iter().rev()
        {
            let index = &mut buckets[(Into::<u64>::into(*k) >> shift & MASK) as usize];
            *index -= 1;
            output[*index] = *k;
        }

        std::mem::swap(input, &mut output);
    }
}

#[cfg(test)]
mod tests
{
    #[test]
    fn random_u64()
    {
        test((0 .. 1_000_000).map(|_| rand::random()).collect::<Vec<u64>>());
    }

    #[test]
    fn random_u32()
    {
        test((0 .. 1_000_000).map(|_| rand::random()).collect::<Vec<u32>>());
    }

    #[test]
    fn random_u8()
    {
        test((0 .. 1_000_000).map(|_| rand::random()).collect::<Vec<u8>>());
    }

    #[test]
    fn random_bool()
    {
        test((0 .. 1_000_000).map(|_| rand::random()).collect::<Vec<bool>>());
    }

    fn test<T>(mut v : Vec<T>)
    where T : Copy + Into<u64> + Ord + std::fmt::Debug
    {
        let mut w = v.clone();

        super::radix_sort(&mut v);
        w.sort();

        assert_eq!(v, w);
    }
}
