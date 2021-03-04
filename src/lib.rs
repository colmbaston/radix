pub fn radix_sort<T : Clone>(a : &mut [T], get_key : impl Fn(&T) -> u64)
{
    const RADIX      : u64   = 8;
    const ITERATIONS : u64   = 64 / RADIX;
    const BUCKETS    : usize = 256;
    const MASK       : u64   = BUCKETS as u64 - 1;

    let mut buckets = Vec::with_capacity(BUCKETS);
    let mut buffer  = Vec::with_capacity(a.len());

    unsafe
    {
        buckets.set_len(BUCKETS);
        buffer.set_len(a.len());
    }

    let mut input  = &mut *a;
    let mut output = &mut *buffer;

    for shift in (0 .. ITERATIONS).map(|i| i * RADIX)
    {
        buckets.iter_mut().for_each(|x| *x = 0);
        input.iter().for_each(|k| buckets[(get_key(k) >> shift & MASK) as usize] += 1);
        buckets.iter_mut().scan(0, |a, x| { *a += *x; *x = *a; Some(()) }).last();

        for k in input.iter().rev()
        {
            let index = &mut buckets[(get_key(k) >> shift & MASK) as usize];
            *index -= 1;
            output[*index] = k.clone();
        }

        std::mem::swap(&mut input, &mut output);
    }
}

#[cfg(test)]
mod tests
{
    #[test]
    fn random_u64()
    {
        test((0 .. 1_000_000).map(|_| rand::random()).collect::<Vec<u64>>(), |k| *k);
    }

    #[test]
    fn random_u32()
    {
        test((0 .. 1_000_000).map(|_| rand::random()).collect::<Vec<u32>>(), |k| *k as u64);
    }

    #[test]
    fn random_u8()
    {
        test((0 .. 1_000_000).map(|_| rand::random()).collect::<Vec<u8>>(), |k| *k as u64);
    }

    #[test]
    fn random_bool()
    {
        test((0 .. 1_000_000).map(|_| rand::random()).collect::<Vec<bool>>(), |k| *k as u64);
    }

    fn test<T>(mut v : Vec<T>, get_key : impl Fn(&T) -> u64)
    where T : Clone + Ord + std::fmt::Debug
    {
        let mut w = v.clone();

        super::radix_sort(&mut v, get_key);
        w.sort();

        assert_eq!(v, w);
    }
}
