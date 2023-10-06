fn main() {
    // let mut i = 0;

    /* cycle 1 to 10
    loop {
        i += 1;

        if i == 7 {
            continue;
        }

        if i > 10 {
            break;
        }
        
        println!("Variable i value is {}", i);
    }
    */

    /* cycle 1 to 50 and show only multiple of 5
    while i < 50 {
        // println!("Variable i value is {}", i);
        i += 1;

        if i % 5 == 0 {
            println!("{} (Multiple by 5)", i);
        } else {
            println!("{} (Not Multiple by 5)", i);
        }

    }
    */

    // FOR LOOP: does't need init variable, 1..11 is the range/iteration, is the same ad 1 < 11, iterates 1 to 10 exclude the top value of the range
    for n in 1..11 {
        println!("The number is {}", n);
    }


    let numbers = 30..51; // variable of type range, as FOR LOOP require a range
    for n in numbers {
        println!("The number is {}", n);
    }

    let animals = vec!["Cat", "Dog", "Horse"];
    for (index, a) in animals.iter().enumerate() {
        println!("The index is {} and Animal name: {}", index, a);
    }

}
