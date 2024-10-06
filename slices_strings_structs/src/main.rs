/*
    String -> Growable string stored in heap. It can be mutated.
    &string (String Slices) -> Immutable string that is stored somewhere on memory and can't be mutated.
*/

fn main() {
    let tweet = String::from("This is my tweet and it is very very long");
    let trimmed_tweet = trim_tweet(&tweet);

    let tweet2 = "This is my tweet and it id very very long";
    let trimmed_tweet2 = trim_tweet(tweet2);

    println!("Trimmed Tweet: {trimmed_tweet}");
    println!("Trimmed Tweet2: {trimmed_tweet2}");


    let s1 = "Hello World";
    let s2 = String::from("Hello World");
    let s3 = "Hello World".to_string();
    let s4 = "Hello World".to_owned();
    let s5 = &s4[..];

    let mut s = String::from("foo");

    s.push_str(" bar");
    println!("s is {s}");
    s.replace_range(.., "buzz");
    println!("s is {s}");

    let a = s2 + &s3;

    let crabs = "🦀🦀🦀🦀";
    println!("carbs {crabs}");

    let only_one_crab = &crabs[0..4];
    println!("only_one_crab: {only_one_crab}"); 

    let two_crabs = &crabs[0..8];
    println!("two_crabs: {two_crabs}");

    let hindi = "आप कैसे हैं?";

    for b in hindi.bytes() {
        println!("{b}");
    }

    for c in hindi.chars() {
        println!("{c}")
    }
}

fn trim_tweet(tweet: &str) -> &str {
    &tweet[..15]
}