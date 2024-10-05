fn main() {
    let s1 = String::from("Rust");
    let s2 = s1.clone();
    println!("s1 is {s2}");

    let s3 = s2.clone();
    println!("s3 is {s3}");

    /*
        Primitives that are stored entirely on stack are clone by default such as
        int
        floating point
        boolean
        characters

        as they are cheap to clone
    */

    let x = 10;
    let _y = x;
    println!("x is {x}");

    let name = String::from("Sushant");
    print_string(name.clone());
    println!("name: {name}");

    let generated_string = generate_str();
    println!("generated string: {generated_string}");

    let s4 = add_to_string(s2);
    println!("s4: {s4}")

}

fn print_string(a: String) {
    println!("a: {a}")
}

fn generate_str() -> String {
    String::from("Hello World")
}

fn add_to_string(mut a: String) -> String {
    a.push_str(" is awesome.");
    a
}