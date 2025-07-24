fn main() {
    let s1 = get_string(); // transfer ownership s1
    print!(" This is s1:{}", s1); // s1 is owner

    let s2 = String::from("world"); // s2 is owner of world
    let s3 = send_get_string(s2); //s2 owner >>s3 is owner send_get_string of world
    // s2 no more owner>> value dropped
    println!(" >>>>> This is s3:{}", s3);
}

fn get_string() -> String {
    let new_string = String::from("hello");
    return new_string;
}

fn send_get_string(received_string: String) -> String {
    return received_string; // to own from s2 // s3
}
