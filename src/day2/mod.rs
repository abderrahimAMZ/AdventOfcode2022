
pub fn score(round:&str) -> i32{
    let mut result:i32;
    let players = round.as_bytes();
    println!("{:?}",players);
    if players.is_empty() {
        result = 0;
    }
    else {
        let player1 = convertChoiceToNumber(*players.get(0).unwrap() as char);
        let player2 = convertChoiceToNumber(*players.get(2).unwrap() as char);
        result = player2;
        if player2 == player1+1 || player2 == player1.clone() -2 {
            result += 6;
        }
        else if player2 == player1 {
            result += 3
        }
    }
    result
}
pub fn convertChoiceToNumber(choice: char) -> i32 {
    match choice {
        'A'| 'X'=> 1,
        'B'| 'Y'=> 2,
        'C'| 'Z'=> 3,
        _ => 0
    }
}
pub fn score_2(round:&str) -> i32{
    let mut result:i32 = 0;
    let players = round.as_bytes();
    println!("{:?}",players);
    if !players.is_empty() {
        let player1 = convertChoiceToNumber(*players.get(0).unwrap() as char);
        let player2 = convertChoiceToNumber(*players.get(2).unwrap() as char);
        if player2 == 3 {
            result += 6;
            result += if player1 ==3 {1} else {player1+1}
        }
        else if player2 == 2 {
            result += 3;
            result += player1;
        }
        else {
            result += 0;
            result += if player1 ==1 {3} else {player1-1}
        }
    }
    result
}
pub fn playing_rock_paper_scissor_1(input :&str) -> i32 {
    let result:i32  = input.split("\n").map(|slice| score(slice)).sum();

    result
}
pub fn playing_rock_paper_scissor_2(input :&str) -> i32 {
    let result:i32  = input.split("\n").map(|slice| score_2(slice)).sum();

     result
}
