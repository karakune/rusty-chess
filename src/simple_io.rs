// use std::io::stdin;
//
// fn simple_io() {
//     let mut is_executing = true;
//     while is_executing
//     {
//         println!("Which number is the coolest?");
//
//         let mut input_text = String::new();
//         stdin().read_line(&mut input_text).expect("failed to read from stdin");
//
//         let trimmed = input_text.trim();
//         match trimmed.parse::<i32>() {
//             Ok(i) => {
//                 println!("Your cool number is {}.", i);
//                 is_executing = false;
//             },
//             Err(..) => println!("this was not an integer: {}", trimmed)
//         }
//     }
// }
