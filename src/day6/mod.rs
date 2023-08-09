pub fn get_marker(input :&str,) -> usize {
    let mut marker_index:usize = 13;
    let chars :Vec<char>= input.chars().collect();
    while has_multiple(&chars[marker_index.clone()-13..=marker_index]) {
       marker_index +=1;
    }
    marker_index+1
}
pub fn has_multiple(chars : &[char]) -> bool{
    let mut has_multiple =false ;
    let mut i = 0;
    for char in chars {
        let (first_part,last_part) = chars.clone().split_at(i);
        let (ch,last_part) = last_part.split_at(1);
       has_multiple = first_part.contains(char)  || last_part.contains(char);
        if (has_multiple){
            break;
        }
        i+=1;
    }
    has_multiple
}
