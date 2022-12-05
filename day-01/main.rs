mod helpers;
use std::collections::HashMap;

fn calories_per_elf() -> HashMap<i32, i32>{
    let calories = helpers::read_calories_input();
    let mut calories_per_elf = HashMap::new();
    let mut elf = 1;

    for calorie in calories.iter() {

        if calorie.len() == 0 {
            elf = elf + 1;
            continue;
        }

        let calorie_casted = calorie.parse::<i32>().unwrap();

        match calories_per_elf.get(&elf) {
            Some(current_elf) => calories_per_elf.insert(elf, current_elf + calorie_casted),
            None => calories_per_elf.insert(elf, calorie_casted)
        };

    }

    calories_per_elf

}

fn main() {
    let calories_per_elf = calories_per_elf();
    let mut hash_vec: Vec<_> = calories_per_elf.iter().collect();
    hash_vec.sort_by(|elf, calories| calories.1.cmp(elf.1));
    println!("The elf with most calories have {:?}", part_one(hash_vec.clone()));
    println!("The top three elfs with most calories have {:?}", part_two(hash_vec.clone()));
}

fn part_one<'a>(hash_vec: Vec<(&'a i32, &'a i32)>) -> &'a i32 {
    hash_vec[0].1
}

fn part_two<'a>(hash_vec: Vec<(&'a i32, &'a i32)>) -> i32 {
    let mut sum = 0;
    for t in &hash_vec[0..3] {
        sum += t.1
    }
    sum
}
