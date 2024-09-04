use std::env;
 
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <algorithm> [arguments...]", args[0]);
        return;
    }

    match args[1].as_str() {
        "bubble_sort" => {
            if args.len() < 3 {
                println!("Please provide a list of numbers to sort");
                return;
            }

            // TODO: bubble sort algo
        },
        "binary_search" => {
            if args.len() < 4 {
                println!("Please provide a sorted list of numbers and a target to search for");
                return;
            }

            // TODO: binary search
        },
        "quick_sort" => {
            if args.len() < 3 {
                println!("Please provide a list of numbers to sort");
                return;
            }

            // TODO: quick sort
        },
        "pre-order_traversal" => {
            if args.len() < 3 {
                println!("Please provide a tree structure to traverse");
                return;
            }
            
            // TODO: tree traversal
        },
        _ => println!("Unknown algorithm: {}", args[1]),
    }
}