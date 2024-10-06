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
    println!("Trimmed Tweet2: {trimmed_tweet2}")
}

fn trim_tweet(tweet: &str) -> &str {
    &tweet[..15]
}