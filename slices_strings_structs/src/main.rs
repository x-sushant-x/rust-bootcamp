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

    let mut book = Product {
        name: "Book".to_string(),
        price: 249.99,
        in_stock: true
    };

    println!("book: {:?}", book);

    let sales_tax = book.calculate_tax();
    println!("sales tax: {sales_tax}");
    
    book.set_price(1.2);
    book.buy();

    // book.set_price(1.2);

    let sword = Product::new("Sword".to_string(), 12.9, true);
    sword.print();
}

fn trim_tweet(tweet: &str) -> &str {
    &tweet[..15]
}

#[derive(Debug)]
struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

impl Product {
    fn new(name: String, price: f32, in_stock: bool) -> Product {
        Product{
            name,
            price,
            in_stock
        }
    }

    // &self -> reference to Product
    fn calculate_tax(&self) -> f32 {
        return self.price * 0.1
    }

    // mutable reference to Product
    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    // self -> Owned form of Product. If this function is called it will be moved and can't be used by anyone afterwords.
    fn buy(self) -> i32 {
        let name = self.name;
        println!("bought book with name: {name}");
        123
    }

    fn print(&self) {
        println!("{:?}", self)
    }
}