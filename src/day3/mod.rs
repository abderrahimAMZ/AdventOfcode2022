use std::collections::HashMap;
pub fn item_types_day3(input : &str)-> i32 {
    input.lines().map(|line| {
        let get_lenth = line.len() / 2;
        let mut result: i32 = 0;
        let bytes = line.as_bytes();
        let first_half = &bytes[..get_lenth];
        let last_half = &bytes[get_lenth..];
        for byte in last_half {
            if first_half.contains(byte) {
                println!("{} as char and as ascci {}", *byte as char, *byte);
                result = (if (*byte > 96) { byte - 96 } else { byte - 38 }) as i32;
            }
        }
        result
    }).sum()
}

pub fn item_types_day3_2(input : &str)-> i32 {
    let input_v:Vec<&str> = input.split("\n").collect();
    let input_lenth = input_v.len()-3;
    let mut result = 0;
    let mut i:usize = 0;
    while (i  <=input_lenth){
        let (Some(&a),Some(&b),Some(&c)) = (input_v.get(i),input_v.get(i+1),input_v.get(i+2)) else {panic!("what the hell is wrong!")};
        for byte in a.as_bytes() {
                let b_bytes = b.as_bytes();
                let c_bytes = c.as_bytes();
            if b_bytes.contains(byte) && c_bytes.contains(byte){
                println!("{}",byte);
                result += if (*byte  > 96) {byte+1} else {byte+2} ;
            }
        }

        i+=3;
        }

    result.into()
}




