pub mod primes;

mod input {
    use std::process;
    use std::fmt::Display;
    use std::io::{self, Write};

    pub fn exit_err<T: Display>(msg: T, code: i32) -> ! {
        let _ = writeln!(&mut io::stderr(), "Error: {}", msg);
        process::exit(code)
    }

    pub fn grab_input(msg: &str) -> io::Result<String> {
        let mut buf = String::new();
        print!("{}: ", msg);
        try!(io::stdout().flush());
        try!(io::stdin().read_line(&mut buf));
        Ok(buf)
    }

    pub fn get_string(msg: &str) -> String{
        grab_input(msg).unwrap_or_else(|e| exit_err(&e, e.raw_os_error().unwrap_or(-1)))
    }

    pub fn get_number(msg: &str) -> usize {
        match get_string(msg).trim().parse()
        {
            Ok(num) => num,
            Err(_) => 0
        }
    }

    pub fn get_double(msg: &str) -> f64 {
        match get_string(msg).trim().parse()
        {
            Ok(num) => num,
            Err(_) => 0f64
        }
    }
}

pub mod dialog {

    pub fn dialog_menu() -> usize {
        println!("");
        println!("Bitte wählen:");
        println!("");
        println!("(1) Anzahl der Goldbach-Zerlegungen von n");
        println!("(2) Anzahl der Goldbach-Zerlegungen nach Zahlen von n bis m");
        println!("(3) Alle Goldbach-Zerlegungen von n");
        println!("(_) Programm beenden");
        println!("");
        super::input::get_number("Ihre Wahl")
    }

    pub fn dialog_1() {
        let mut lower: usize = 0;
        while lower <= 2 || 0 != lower % 2 {
            lower = super::input::get_number("Bitte geben sie eine gerade ganze Zahl n > 2 ein");
        }

        let number = super::primes::primesums(lower, false);
        println!("Es gibt {} Darstellungen von {} als Summe zweier Primenzahlen.", number, lower);
    }

    pub fn dialog_2()
    {
        let mut lower: usize = 0;
        while lower <= 2 || 0 != lower % 2 {
            lower = super::input::get_number("Bitte geben sie eine gerade ganze Zahl n > 2 ein");
        }

        let mut upper: usize = 0;
        while upper <= lower || 0 != upper % 2 {
            let msg = format!("Bitte geben sie eine gerade ganze Zahl m > {} ein", lower);
            upper = super::input::get_number(&msg);
        }

        println!("");
        println!("Anzahl Goldbachzerlegungen");
        println!("");

        while lower <= upper {
            let number = super::primes::primesums(lower, false);
            println!("{} = {} ", lower, number);
            lower += 2;
        }
    }

    pub fn dialog_3()
    {
        let mut grenze: usize = 0;
        while grenze <= 2 || 0 != grenze % 2 {
            grenze = super::input::get_number("Bitte geben sie eine gerade ganze Zahl n > 2 ein");
        }

        let anzahl = super::primes::primesums(grenze, true);
        println!("Es gibt {} Darstellungen von {} als Summe zweier Primenzahlen.", anzahl, grenze);
    }
}
