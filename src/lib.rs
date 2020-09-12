pub fn radix_sort(input : &mut Vec<u64>)
{
    const RADIX      : usize = 8;
    const ITERATIONS : usize = 64 / RADIX;
    const BUCKETS    : usize = 256;
    const MASK       : usize = BUCKETS - 1;

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
        input.iter().for_each(|k| buckets[*k as usize >> shift & MASK] += 1);
        buckets.iter_mut().scan(0, |a, x| { *a += *x; *x = *a; Some(()) }).last();

        for k in input.iter().rev()
        {
            let index = &mut buckets[*k as usize >> shift & MASK];
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
        let mut v = (0 .. 1_000_000).map(|_| rand::random()).collect::<Vec<u64>>();
        let mut w = v.clone();

        super::radix_sort(&mut v);
        w.sort();

        assert_eq!(v, w);
    }
}
