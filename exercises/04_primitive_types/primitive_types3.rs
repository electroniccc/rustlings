fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    let mut a = String::from("");
    let mut i = 0;
    loop {
        if i > 100 {
            break;
        }
        i += 1;
        a += "a";
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
