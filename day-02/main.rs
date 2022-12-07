mod helpers;
use std::collections::HashMap;

fn round(me_str: &str, opponent_str: &str) -> i32 {

    let match_moves = HashMap::from([
        ("Rock", ["A", "X"]),
        ("Paper", ["B", "Y"]), 
        ("Scissors", ["C", "Z"])
    ]);

    let mut score = 0;

    if match_moves["Rock"].contains(&me_str){
        score = 1
    }
    else if match_moves["Paper"].contains(&me_str) {
        score = 2
    }
    else if match_moves["Scissors"].contains(&me_str) {
        score = 3
    }

    // Draw
    if match_moves["Rock"].contains(&me_str) && match_moves["Rock"].contains(&opponent_str) || 
    match_moves["Paper"].contains(&me_str) && match_moves["Paper"].contains(&opponent_str) || 
    match_moves["Scissors"].contains(&me_str) && match_moves["Scissors"].contains(&opponent_str) {
        return score + 3;
    }

    // I won
    if match_moves["Rock"].contains(&me_str) && match_moves["Scissors"].contains(&opponent_str) ||
    match_moves["Scissors"].contains(&me_str) && match_moves["Paper"].contains(&opponent_str) ||
    match_moves["Paper"].contains(&me_str) && match_moves["Rock"].contains(&opponent_str)
    {
        return score + 6;
    }

    // I lose
    if match_moves["Rock"].contains(&me_str) && match_moves["Paper"].contains(&opponent_str) ||
    match_moves["Paper"].contains(&me_str) && match_moves["Scissors"].contains(&opponent_str) ||
    match_moves["Scissors"].contains(&me_str) && match_moves["Rock"].contains(&opponent_str)
    {
        return score;
    }

    0

}


fn main() {
    let game = helpers::read_choices();

    let mut total_score = 0;

    for g in game.iter() {
        let me = g.chars().nth(0).unwrap().to_string();
        let opponent = g.chars().nth(2).unwrap().to_string();
        let opponent_str = opponent.as_str();
        let me_str = me.as_str();

        let round = round(opponent_str, me_str);
        total_score += round;
                
    }

    println!("{}", total_score);
}
