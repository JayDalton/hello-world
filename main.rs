use std::io;
use std::cmp::Ordering;

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

fn dialog_1() {
    println!("Bitte geben sie eine gerade ganze Zahl n > 2 ein: ");

    println!("Es gibt ... Darstellungen von .... als Summe zweier Zahlen.");
}

fn dialog_2(){
    println!("Bitte geben sie eine gerade ganze Zahl n > 2 ein: ");
    println!("Bitte geben sie eine gerade ganze Zahl m > ... ein: ");
}

fn dialog_3(){
    println!("Bitte geben sie eine gerade ganze Zahl n > 2 ein: ");
}

fn main() {

    loop {

        println!("");
        println!("Bitte wählen:");
        println!("");
        println!("(1) Anzahl der Goldbach-Zerlegungen von n");
        println!("(2) Anzahl der Goldbach-Zerlegungen nach Zahlen von n bis m");
        println!("(3) Alle Goldbach-Zerlegungen von n");
        println!("(4) Programm beenden");
        println!("");
        println!("Ihre Wahl: ");
        
        let mut select = String::new();

        io::stdin().read_line(&mut select).expect("Failed to read line!");

        let select: u32 = match select.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You selected: {}", select);

        match select {
            1 => dialog_1(),
            2 => println!(" _2_ "),
            3 => println!(" _3_ "),
            4 => break,
            _ => println!(" unknown ")
        }
    }


    //println!("{:?}", simple_sieve(100));
}
