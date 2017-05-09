use std::io;
use std::f64;
use std::iter::{empty, once};
use std::time::Instant;
use std::time::Duration;
use std::collections::HashMap;

mod source;
use source::primes as prime;

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

    let vrslt = prime::stupid_sieve(100);
    println!("stupid {:?}", vrslt.iter().filter(|&n| *n == true).collect::<Vec<_>>());
    let strt = Instant::now();
    let rslt = (1..1000).map(|_| prime::stupid_sieve(n)).last().unwrap();
    println!("stupid_sieve {} took {} milliseconds.", rslt.iter().filter(|&n| *n == true).count(), get_duration(strt));

    let vrslt = prime::simple_sieve(100);
    println!("simple {:?}", vrslt);
    let strt = Instant::now();
    let rslt = (1..1000).map(|_| prime::simple_sieve(n)).last().unwrap();
    println!("simple_sieve {} took {} milliseconds.", rslt.len(), get_duration(strt));

    let vrslt = prime::basic_sieve(100).collect::<Vec<_>>();
    println!("basics {:?}", vrslt);
    let strt = Instant::now();
    let rslt = (1..1000).map(|_| prime::basic_sieve(n)).last().unwrap();
    println!("basic_sieve {} took {} milliseconds.", rslt.count(), get_duration(strt));

    let vrslt = prime::optimized_sieve(100).collect::<Vec<_>>();
    println!("optimi {:?}", vrslt);
    let strt = Instant::now();
    let rslt = (1..1000).map(|_| prime::optimized_sieve(n)).last().unwrap();
    println!("optimi_sieve {} took {} milliseconds.", rslt.count(), get_duration(strt));
}

fn primesums(grenze: usize, zeigen: bool) -> usize {
    let mut anzahl: usize = 0;
    let liste = source::primes::simple_sieve(grenze);
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

    // println!("{}", f64::MIN_POSITIVE);
    // println!("{}", f64::MAX);

    // let num = 1000000000;

    // let mut res = prime::stupid_sieve(num);
    // // for (a, b) in res.iter().enumerate() {
    // //     if *b {
    // //         println!("a {} b {}", a, b);
    // //     }
    // // }
    // res.retain(|&n| n == true );
    // println!("Stupid: {}", res.len());

    // let res = prime::simple_sieve(num);
    // println!("Simple: {}", res.len());

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
