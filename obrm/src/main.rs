use std::rc::Rc;

struct Car {}

fn memory_example() {
    // car variable is the owner of Car object.
    let car = Box::new(Car {});

    // Now car2 is the owner of Car object.
    let car2 = car;

    // Now rcCar and car3 are poiting to same memory pointer.
    let rcCar = Rc::new(Car {});
    let car3 = rcCar.clone();
}

fn main() {}
