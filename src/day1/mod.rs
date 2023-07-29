
pub fn elves_max_calories_day1(input_string:&str) -> i32 {
    let mut _max = 0;
    let numbers: Vec<i32> = input_string
        .split("\n") // Split the input string by newline characters
        .map(|s| s.trim().parse().unwrap_or(-1)) // Parse each part to an integer or use -1 for empty lines
        .collect();
    let mut elves_calories :Vec<i32> = vec![];
    println!("{:?}",numbers);
    for number in numbers {
        if number == -1 {
            elves_calories.push(_max);
            _max = 0
        }
        else {
            _max +=number;
        }
    }

    elves_calories.push(_max);
    elves_calories.sort();
    println!("sorted elves calories {:?}",elves_calories);
    let most_3:i32 = elves_calories[elves_calories.len()-3..elves_calories.len()].to_vec().iter().sum();
    most_3

}
mod tests {
    #[cfg(test)]
    fn it_works() {
        use std::fs;
        use super::elves_max_calories_day1;
       let input  = fs::read_to_string("input_pr1.txt").expect("expect a file here");
       let result = elves_max_calories_day1(&input);
        assert_eq!(result, 204837);
    }
}