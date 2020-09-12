pub fn radix_sort(input : &mut Vec<u64>)
{
    const RADIX      : usize = 8;
    const ITERATIONS : usize = 64 / RADIX;
    const BUCKETS    : usize = 256;
    const MASK       : usize = BUCKETS - 1;

    let mut buckets = vec![0usize ; BUCKETS];
    let mut output  = vec![0u64   ; input.len()];

    for shift in (0 .. ITERATIONS).map(|i| i * RADIX)
    {
        input.iter().for_each(|k| buckets[*k as usize >> shift & MASK] += 1);
        buckets.iter_mut().scan(0, |a, x| { *a += *x; *x = *a; Some(()) }).last();

        for k in input.iter().rev()
        {
            let index = &mut buckets[(*k as usize >> shift) & MASK];
            *index -= 1;
            output[*index] = *k;
        }

        std::mem::swap(input, &mut output);
        buckets.iter_mut().for_each(|x| *x = 0);
    }
}

#[cfg(test)]
mod tests
{
    #[test]
    fn random()
    {
        let mut v = (0 .. 1_000_000).map(|_| rand::random()).collect();
        super::radix_sort(&mut v);
        assert!(is_sorted(&v));
    }

    fn is_sorted(v : &[u64]) -> bool
    {
        v.iter().try_fold(u64::MIN, |a, x| if a <= *x { Some(*x) } else { None }).is_some()
    }
}
