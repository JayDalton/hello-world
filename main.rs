use std::io;

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

    loop {

        match dialog_menu() {
            1 => dialog_1(),
            2 => dialog_2(),
            3 => dialog_3(),
            _ => break,
        }
    }


    //println!("{:?}", simple_sieve(100));
}
