use std::io;
use std::iter::{empty, once};
use std::time::Instant;
use std::time::Duration;
use std::collections::HashMap;

pub fn my_sieve(grenze: usize) -> Vec<bool> {
    let mut vec = Vec::new();
    let mut liste = HashMap::new();

    liste.insert(2, true);
    liste.insert(3, true);

//   for (int n = 0, x = 1; x <= sqrt(grenze); x++)
//   {
//     for (int y = 1; y <= sqrt(grenze); y++)
//     {
//       n = 4 * x * x + y * y;
//       if (n <= grenze && (n % 12 == 1 || n % 12 == 5)) {
//         liste.at(n) = liste.at(n) ? 0 : 1;
//       }
   
//       n = 3 * x * x + y * y;
//       if (n <= grenze &&  n % 12 == 7) {
//         liste.at(n) = liste.at(n) ? 0 : 1;
//       }
   
//       n = 3 * x * x - y * y;
//       if (x > y && n <= grenze && n % 12 == 11) {
//         liste.at(n) = liste.at(n) ? 0 : 1;
//       }
//     }
//   }

//   for (int n = 5; n <= sqrt(grenze); n++)
//   {
//     if (liste.at(n))
//     {
//       for (int k = 1; k * n * n < grenze; k++)
//       {
//         liste.at(k * n * n) = 0;
//       }
//     }
//   }


    vec
}

pub fn simple_sieve(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit+1];
    is_prime[0] = false;
    if limit >= 1 { is_prime[1] = false }

    for num in 2..limit+1 {
        if is_prime[num] {
            let mut multiple = num*num;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += num;
            }
        }
    }

    is_prime.iter().enumerate()
        .filter_map(|(pr, &is_pr)| if is_pr {Some(pr)} else {None} )
        .collect()
}

pub fn basic_sieve(limit: usize) -> Box<Iterator<Item = usize>> {
    if limit < 2 { return Box::new(empty()) }
    
    let mut is_prime = vec![true; limit+1];
    is_prime[0] = false;
    if limit >= 1 { is_prime[1] = false }
    let sqrtlmt = (limit as f64).sqrt() as usize + 1; 

    for num in 2..sqrtlmt {
        if is_prime[num] {
            let mut multiple = num * num;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += num;
            }
        }
    }

    Box::new(
        is_prime.into_iter().enumerate()
        .filter_map(|(p, is_prm)| if is_prm { Some(p) } else { None })
    )
}

pub fn optimized_sieve(limit: usize) -> Box<Iterator<Item = usize>> {
    if limit < 3 {
        return if limit < 2 { Box::new(empty()) } else { Box::new(once(2)) }
    }

    let ndxlmt = (limit - 3) / 2 + 1;
    let bfsz = ((limit - 3) / 2) / 32 + 1;
    let mut cmpsts = vec![0u32; bfsz];
    let sqrtndxlmt = ((limit as f64).sqrt() as usize - 3) / 2 + 1;

    for ndx in 0..sqrtndxlmt {
        if (cmpsts[ndx >> 5] & (1u32 << (ndx & 31))) == 0 {
            let p = ndx + ndx + 3;
            let mut cullpos = (p * p - 3) / 2;
            while cullpos < ndxlmt {
                unsafe { // avoids array bounds check, which is already done above
    	            let cptr = cmpsts.get_unchecked_mut(cullpos >> 5);
	                *cptr |= 1u32 << (cullpos & 31);
                }
//                cmpsts[cullpos >> 5] |= 1u32 << (cullpos & 31); // with bounds check
                cullpos += p;
            }
        }
    }

    Box::new((-1 .. ndxlmt as isize).into_iter().filter_map(move |i| 
    {
        if i < 0 { Some(2) } else {
            if cmpsts[i as usize >> 5] & (1u32 << (i & 31)) == 0 {
                Some((i + i + 3) as usize) } else { None } }
    }))
}
