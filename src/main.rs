// fn main() {
//     let s1 = get_string(); // transfer ownership s1
//     print!(" This is s1:{}", s1); // s1 is owner

//     let s2 = String::from("world"); // s2 is owner of world
//     let s3 = send_get_string(s2); //s2 owner >>s3 is owner send_get_string of world
//     // s2 no more owner>> value dropped
//     println!(" >>>>> This is s3:{}", s3);
// }

// fn get_string() -> String {
//     let new_string = String::from("hello");
//     return new_string;
// }

// fn send_get_string(received_string: String) -> String {
//     return received_string; // to own from s2 // s3
// }

// I changed this again

// clone method

// fn main() {
//     let s1 = String::from("hello"); // O T to s2
//     let len = cal_length(s1.clone());

//     println!("The length of {}, {}", s1, len);
// }

// fn cal_length(s: String) -> usize {
//     let length: usize = s.len();
//     return length;
// }

// Borrow operation

// fn main() {
//     let s1 = String::from("hello"); // s1 is owner
//     let len: usize = calc_length(&s1);

//     println!("The length of {} is {}", s1, len);
// }

// fn calc_length(s2: &String) -> usize {
//     return s2.len();
// }

fn main() {
    let arr: [&str; 3] = ["hello", "bello", "Kilo"];
    write_arr(arr); // array directly pass as argument

    println!("arr ={:?}", arr);
}

fn write_arr(mut arr1: [&str; 3]) {
    arr1[0] = "fellow";
    println!("arr1 ={:?}", arr1);
}
