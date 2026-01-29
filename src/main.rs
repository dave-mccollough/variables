fn main() {

    // Variable creation
    let a = 5;
    let peak = "Longs Peak";
    println!("The value of a is: {}", a);
    println!("Colorado's northernmost 14er is {}", peak);

    // Variable mutability
    // Add mut keyword to make variable mutable
    let mut b = 10;
    b = 15;
    println!("The value of b is: {}", b);

    let mut next_peak = "Pikes Peak";
    next_peak = "Mount Blue Sky";
    println!("Another front range 14er is {}", next_peak);

    // Variable shadowing
    // Both variables are in the same scope with the same name, the second variable will shadow the first.
    let c = 10;
    let c = 20;
    println!("The value of c is: {}", c);

    // Variable scope
    // Variable bindings have a scope, and are constrained to live in a block. A block is a collection of statements enclosed by braces {}
    let d = 30;

    {
        let e = 40;
    }
    // THe value of e can't be found in the scope.
    println!("The value of d is: {}", e);
}
