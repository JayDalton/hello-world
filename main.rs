mod source;
use source::dialog as dial;

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

    // compare_primealgos();

    loop {

        match dial::dialog_menu() {
            1 => dial::dialog_1(),
            2 => dial::dialog_2(),
            3 => dial::dialog_3(),
            _ => break,
        }
    }

}
