// use std::io;
use std::iter::{empty, once};
use std::time::Instant;
// use std::time::Duration;
// use std::collections::HashMap;

pub fn int_sqrt(n:usize) -> usize {
    if n < (1 << 52) {
        (n as f64).sqrt() as usize
    } else {
        (n as f64).sqrt() as usize
    }
}

pub fn stupid_sieve(max_num: usize) -> Vec<bool> {

    let mut vec = vec![false; max_num];
    vec[2] = true;
    vec[3] = true;

    for x in 1..int_sqrt(max_num)+1 {
        for y in 1..int_sqrt(max_num)+1 {

            // let n = 4 * x * x + y * y;
            match (4 * x * x).checked_add(y * y){
                Some(n) => 
                    if n <= max_num && (n % 12 == 1 || n % 12 == 5) {
                        vec[n] = if vec[n] {false} else {true};
                    }
                ,
                None => continue
            }

            // let n = 3 * x * x + y * y;
            match (3 * x * x).checked_add(y * y){
                Some(n) => 
                    if n <= max_num && (n % 12 == 7) {
                        vec[n] = if vec[n] {false} else {true};
                    }
                ,
                None => continue
            }

            // let n = 3 * x * x - y * y;
            match (3 * x * x).checked_sub(y * y){
                Some(n) => 
                    if x > y && n <= max_num && (n % 12 == 11) {
                        vec[n] = if vec[n] {false} else {true};
                    }
                ,
                None => continue
            }
        }
    }

    for n in 5..int_sqrt(max_num)+1 {
        if vec[n] {
            let mut k = 1;
            while k * n * n < max_num {
                vec[k * n * n] = false;
                k+=1;
            }
        }
    }
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

pub fn get_duration(strt: Instant) -> u64 {
    let elpsd = strt.elapsed();
    let secs = elpsd.as_secs();
    let millis = (elpsd.subsec_nanos() / 1000000) as u64;
    secs * 1000 + millis
}

// do it 1000 times to get a reasonable execution time span...
pub fn compare_primealgos(){
    let n = 10000;

    let vrslt = stupid_sieve(100);
    println!("stupid {:?}", vrslt.iter().filter(|&n| *n == true).collect::<Vec<_>>());
    let strt = Instant::now();
    let rslt = (1..1000).map(|_| stupid_sieve(n)).last().unwrap();
    println!("stupid_sieve {} took {} milliseconds.", rslt.iter().filter(|&n| *n == true).count(), get_duration(strt));

    let vrslt = simple_sieve(100);
    println!("simple {:?}", vrslt);
    let strt = Instant::now();
    let rslt = (1..1000).map(|_| simple_sieve(n)).last().unwrap();
    println!("simple_sieve {} took {} milliseconds.", rslt.len(), get_duration(strt));

    let vrslt = basic_sieve(100).collect::<Vec<_>>();
    println!("basics {:?}", vrslt);
    let strt = Instant::now();
    let rslt = (1..1000).map(|_| basic_sieve(n)).last().unwrap();
    println!("basic_sieve {} took {} milliseconds.", rslt.count(), get_duration(strt));

    let vrslt = optimized_sieve(100).collect::<Vec<_>>();
    println!("optimi {:?}", vrslt);
    let strt = Instant::now();
    let rslt = (1..1000).map(|_| optimized_sieve(n)).last().unwrap();
    println!("optimi_sieve {} took {} milliseconds.", rslt.count(), get_duration(strt));
}

pub fn primesums(grenze: usize, zeigen: bool) -> usize {
    let mut anzahl: usize = 0;
    let liste = stupid_sieve(grenze);

    for n in 0..grenze / 2
    {
        if liste[n] && liste[grenze - n]
        {
            if zeigen {
                println!(" {} = {} + {}", grenze, n, grenze - n);
            }
            anzahl += 1;
        }
    }
    anzahl
}

