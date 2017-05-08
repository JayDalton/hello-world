use std::io;
use std::iter::{empty, once};
use std::time::Instant;
use std::time::Duration;
use std::collections::HashMap;

fn my_sieve(grenze: usize) -> Vec<bool> {
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

fn simple_sieve(limit: usize) -> Vec<usize> {
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

fn basic_sieve(limit: usize) -> Box<Iterator<Item = usize>> {
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

fn optimized_sieve(limit: usize) -> Box<Iterator<Item = usize>> {
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

fn get_duration(strt: Instant) -> u64 {
    
    let elpsd = strt.elapsed();

    let secs = elpsd.as_secs();
    let millis = (elpsd.subsec_nanos() / 1000000) as u64;
    let dur = secs * 1000 + millis;
    // println!("basic_sieve took {} milliseconds.", dur);
    dur
}

// do it 1000 times to get a reasonable execution time span...
fn compare_primealgos(){
    let n = 10000;

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
    println!("optimized_sieve {} took {} milliseconds.", rslt.count(), get_duration(strt));
}

fn primesums(grenze: usize, zeigen: bool) -> usize {
    let mut anzahl: usize = 0;
    let liste = simple_sieve(grenze);
    for n in 0..grenze
    {
        // if (liste[n] && liste[grenze - n])
        // {
        //     // if (zeigen) {
        //     //     cout << "\n " << grenze << " = " << n << " + " << grenze - n;
        //     // }
        //     anzahl++;
        // }
    }
    anzahl
}

fn get_number(msg: &str) -> u32 {

    println!("{}", msg);

    let mut select = String::new();
    io::stdin().read_line(&mut select).expect("Failed to read line!");
    let select: u32 = match select.trim().parse()
    {
        Ok(num) => num,
        Err(_) => 0
    };
    select
}

fn dialog_menu() -> u8 {
    
    println!("");
    println!("Bitte wählen:");
    println!("");
    println!("(1) Anzahl der Goldbach-Zerlegungen von n");
    println!("(2) Anzahl der Goldbach-Zerlegungen nach Zahlen von n bis m");
    println!("(3) Alle Goldbach-Zerlegungen von n");
    println!("(_) Programm beenden");
    println!("");
    println!("Ihre Wahl: ");
    
    let mut select = String::new();
    io::stdin().read_line(&mut select).expect("Failed to read line!");
    let select: u8 = match select.trim().parse()
    {
        Ok(num) => num,
        Err(_) => 0
    };
    select
}

fn dialog_1() {
    println!("Bitte geben sie eine gerade ganze Zahl n > 2 ein: ");

    println!("Es gibt ... Darstellungen von .... als Summe zweier Zahlen.");
}

fn dialog_2(){
    println!("Bitte geben sie eine gerade ganze Zahl n > 2 ein: ");
    println!("Bitte geben sie eine gerade ganze Zahl m > ... ein: ");
}

fn dialog_3(){

    let mut anzahl: u32 = 0;
    let mut grenze: u32 = 0;

    while grenze <= 2 || 0 != grenze % 2 {
        grenze = get_number("Bitte geben sie eine gerade ganze Zahl n > 2 ein: ");
    }

}

fn main() {

    compare_primealgos();

    // loop {

    //     match dialog_menu() {
    //         1 => dialog_1(),
    //         2 => dialog_2(),
    //         3 => dialog_3(),
    //         _ => break,
    //     }
    // }

}
