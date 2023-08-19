mod variables;

fn variables(){
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 7;

    const MIUNTES: u32 = 60 * 60;

    let x = 5;
    let x = x + 2;

    {
        let x = x + 3;

        let spaces = "   ";
        let spaces = spaces.len();
    }

    let guess: u32 = "45".parse().expect("Not a number");
}