const MAX_PLAYERS: i32 = 10;
static CASINO_NAME: &str = "Sushant's Casino";

fn main() {
    println!("Hello, world!");

    let _a: i16 = 5;

    /*
     * bool
     * signed integers
     * unsigned integers
     * floating point numbers
     * uszie
     * chars, &str, String
     * array -> [T, n]
     * tuples -> ()
     * Type Alias -> type age = u8;
     */

    my_function(100);

    let mut b: i8 = 0;

    'outer: loop {
        if b >= 10 {
            break 'outer;
        }
        b += 1;
        println!("b: {}", b);
    }

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    for el in arr {
        println!("Element: {}", el);
    }
}

fn my_function(x: i32) -> i32 {
    println!("my function called with: {}", x);
    x
}
