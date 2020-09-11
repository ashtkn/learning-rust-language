// fn get_int_from_file() -> i32 {
//     let path = "number.txt";
//     let num_str = std::fs::read_to_string(path).expect("failed to open file");
//     let ret = num_str
//         .trim()
//         .parse::<i32>()
//         .expect("failed to parse string to a number");

//     ret * 2
// }

fn get_int_from_file() -> Result<i32, String> {
    let path = "number.txt";
    let num_str = std::fs::read_to_string(path)
        .map_err( |e| e.to_string())?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(|e| e.to_string())
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e)
    }
}
