fn main() {
    loop {
        println!("pick a game:\n1. fahrenheit to celcius\n2. nth number of fobinaccis sequence\n3. lyrics to a song\n4. exit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("failed to read input");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number");
                continue;
            }
        };

        match choice {
            1 => {
                println!("fahrenheit: ");
                let guess: f32 = fahrenheit(polish_float());
                println!("converted to celcius: {guess}");
            },
            2 => {
                println!("nth fib num: ");
                let n = polish_int();
                let guess: u64 = fibonacci(n);
                println!("the {}th fib num is: {}", n, guess);
            },
            3 => {
                lyrics();
            },
            4 => break,
            _ => println!("invalid choice"),
        }
    }
}

fn fahrenheit(x: f32) -> f32 {
    (x - 32.0) * 5.0 / 9.0
}

fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}

fn polish_float() -> f32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("failed to read line");
    let s: f32 = match s.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("enter a valid number");
            return 0.0;
        }
    };
    s
}

fn polish_int() -> u32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("failed to read line");
    let s: u32 = match s.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("enter a valid number");
            return 0;
        }
    };
    s
}

fn lyrics() {
    let a: [&str; 2] = ["On the ", " day of Christmas my true love sent to me "];
    let b: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth", "tenth", "eleventh", "twelveth"];
    let c: [&str; 12] = ["a partridge in a pear tree", "two turtle doves", "three french hens", "four calling birds", "five gold rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];

    let mut count: usize = 0;
    for n in 0..12 {
        println!("{}{}{}", a[0], b[n], a[1]);
        while count < n+1 {
            if count == n && count != 0 {
                println!("and {}.", c[count]);
            }
            if n == 0 {
                println!("{}.", c[count]);
            } else if count != n  {
                println!("{},", c[count]);
            }
            count += 1;
        }
        count = 0;
    }
}
