use std::any::type_name;
use nameof::name_of;

/*
    This code demonstrates the basic use of traits:
    1. Defining a trait.
    2. Implementing a trait.
    4. Trait bound syntax.
    5. Trait bound syntactic sugar syntax.
    6. Conditional methods based on which traits a generic type implements.
*/

trait Speak {
    fn speak(&self) -> &str {
        "Animal sound."
    }
}

trait Recall {
    fn recall(&self) -> &str {
        "I'm coming back."
    }
}

struct Dog {}

struct Cat {}

impl Speak for Dog {
    fn speak(&self) -> &str {
        "Bark"
    }
}

impl Speak for Cat {
    fn speak(&self) -> &str {
        "Meow"
    }
}

impl Recall for Dog {}

// Using traits to bind param type.
// Limitation here is if we need multiple params to implement a trait,
// but also be of different types.
fn take_to_park(animal: &impl Recall) {
    println!("Taking a {} to the park.", name_of!(animal))
}

fn take_to_park_trait_bound_syntax<T: Recall>(animal: &T) {
    println!("Taking a {} to the park.", type_name::<T>())
}

fn take_to_play(animal: &(impl Speak + Recall)) {
    println!("Taking a {} to play.", name_of!(animal))
}

fn take_to_play_trait_bound_syntax<T: Speak + Recall>(animal: &T) {
    println!("Taking a {} to play.", type_name::<T>())
}

// Let's say we need multiple parameters of different types.
// Syntactic sugar approach would not work here because it forces
// the types to be the same. So we need to be explicit with the
// trait bound syntax.
fn take_two_to_play_trait_bound_syntax<T: Speak + Recall, U: Speak + Recall>(animal_one: &T, animal_two: &U) {
    println!("Taking a {} and a {} to play.", type_name::<T>(), type_name::<U>());
}

// That easily becomes a mess so let's use the where clause instead.
fn take_two_to_play_trait_bound_syntax_where<T, U>(animal_one: &T, animal_two: &U)
where T: Speak + Recall, U: Speak + Recall {
    println!("Taking a {} and a {} to play.", type_name::<T>(), type_name::<U>());
}

struct Pet<T> {
    animal: T
}

impl<T: Recall> Pet<T> {
    fn call_back(&self) -> &str {
        self.animal.recall()
    }
}

// Only define a method on pet if T implements a specific trait.
impl<T: Speak> Pet<T> {
    fn provoke(&self) -> &str {
        self.animal.speak()
    }
}

fn main() {
    let dog = Dog {};
    let dog2 = Dog {};
    let cat = Cat {};

    println!("{}", dog.speak());
    println!("{}", dog.recall());
    println!("{}", cat.speak());

    //println!("{}", cat.recall()); Doesn't exist, doesn't compile.
    //take_to_park(&cat); Cat doesn't implement recall, doesn't compile.
    take_to_park(&dog);
    take_to_park_trait_bound_syntax(&dog);

    take_to_play(&dog);
    take_to_play_trait_bound_syntax(&dog);

    //take_two_to_play_trait_bound_syntax(&dog, &cat); Does not compile because of cat.
    take_two_to_play_trait_bound_syntax_where(&dog2, &dog);

    let pet = Pet {
        animal: dog // move dog ownership to struct.
    };

    //println!("{}", name_of!(dog)); Can't use dog variable anymore as the value has been moved.

    pet.call_back();
    pet.provoke();

    let pet2 = Pet {
        animal: cat // move cat ownership here.
    };

    pet2.provoke(); // This is the only method there is visibility for.

}