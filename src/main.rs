fn main() {
    
    //A) Does not compile, since we're attempting to mutate an immutable reference.
    
    //let s = String::from("hello");
    // try_to_mutate_via_an_immutable_reference(&s);


    let mut mutable_string = String::from("hello");

    //B) Works as expected, we are passing a mutable reference,
    // then using it to mutate the value it is pointing to.

    mutate_via_a_mutable_reference(&mut mutable_string);

    println!("{}", mutable_string);

    //C) You can have only one mutable reference to a particular piece of data 
    // in scope at any time... the code below will not compile.
    
    // let mutable_reference_1 = &mut mutable_string;
    // let mutable_reference_2 = &mut mutable_string;

    // println!("{}, {}", mutable_reference_1, mutable_reference_2);

    //D) However, if we use a mutable reference for the last time, 
    // then create a new mutable reference for the same data, that is fine.
    // Note that mutable_reference_1 goes out of scope after it's last usage,
    // not when the code block ends... this is an example of Non-Lexical-Lifetimes,
    // which is a pretty handy feature! 

    let mutable_reference_1 = &mut mutable_string;
    println!("{}", mutable_reference_1);

    let mutable_reference_2 = &mut mutable_string;
    println!("{}", mutable_reference_2);

    //E) Similarly, if we a mutable reference and immutable references
    // to the same piece of data, that's a big problem! Users of immutable
    // references don't expect the data to change, so while the immutable 
    // reference is in scope, no mutable references are allowed... period!

    // let immutable_reference_1 = &mutable_string; // no problem
    // let immutable_reference_2 = &mutable_string; // no problem
    // let mutable_reference = &mut mutable_string; // BIG PROBLEM

    // println!("{}, {}, and {}", immutable_reference_1, immutable_reference_2, mutable_reference);

    //F)If the immutable references go out of scope, then a mutable reference 
    // is created, that's a-okay! Having mutliple immutable references at one time is fine,
    // since th edata being references can't be mutated, causing unusual behaviour.

    let immutable_reference_1 = &mutable_string; // no problem
    let immutable_reference_2 = &mutable_string; // no problem
    
    println!("{}, {}", immutable_reference_1, immutable_reference_2);

    let mutable_reference = &mut mutable_string; // no problem
    println!("{}", mutable_reference);

    //G) Protection against Dangling. If something is created in a subscope,
    // and is destroyed, yet we try to return a reference to it... we've made a dangling pointer, bad.
    // Rust's compiler protects us against this <3. Instead, we can simply pass the data from inside the method
    // back, rather than it's reference. This transferrs ownership from the subscope to the 
    // parent scope, good.

    //let reference_to_nothing = dangle();    
}

// fn try_to_mutate_vai_an_immutable_reference(some_string: &String) {
//     some_string.push_str(", world");
// }

fn mutate_via_a_mutable_reference(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!