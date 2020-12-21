use crate::util;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

pub fn run_easy() {
    let foods = get_input();
    let all_ingredients: HashSet<_> = foods.iter().flat_map(|food| food.ingredients.iter()).collect();
    let all_allergens: HashSet<_> = foods.iter().flat_map(|food| food.allergens.iter()).collect();
    let mut possible_allergen_ingredients = HashSet::new();
    for allergen in all_allergens {
        let my_possible_allergen_ingredients = get_possible_allergen_ingredients(&foods, &all_ingredients, allergen);
        possible_allergen_ingredients = possible_allergen_ingredients.union(&my_possible_allergen_ingredients).map(|s| *s).collect();
    }

    let mut occurrences = 0;
    for food in &foods {
        for ingredient in &food.ingredients {
            if !possible_allergen_ingredients.contains(&ingredient) {
                occurrences += 1;
            }
        }
    }

    println!("{}", occurrences);
}

pub fn run_hard() {
    let foods = get_input();
    let all_ingredients: HashSet<_> = foods.iter().flat_map(|food| food.ingredients.iter()).collect();
    let mut possible_allergen_ingredients: HashMap<_, _> = foods.iter()
        .flat_map(|food| food.allergens.iter())
        .map(|allergen| (allergen, get_possible_allergen_ingredients(&foods, &all_ingredients, allergen)))
        .collect();
    let mut remaining_ingredients: HashSet<_> = possible_allergen_ingredients.values()
        .fold(HashSet::new(), |a, b| a.union(&b).map(|s| *s).collect());
    let mut allergen_assignments = HashMap::new();
    while !possible_allergen_ingredients.is_empty() {
        let mut result: Option<(&String, &String)> = None;
        for ingredient in &remaining_ingredients {
            let possible_allergens: Vec<_> = possible_allergen_ingredients.iter()
                .flat_map(|(allergen, ingredients)| ingredients.iter().map(move |ingr| (allergen, ingr)))
                .filter(|(_, ingr)| *ingr == ingredient)
                .map(|(allergen, _)| *allergen)
                .collect();
            if possible_allergens.len() == 1 {
                let allergen = *possible_allergens.iter().next().unwrap();
                result = Some((ingredient, allergen));
                break;
            }
        }
        if result.is_some() {
            let (ingredient, allergen) = result.unwrap();
            allergen_assignments.insert(ingredient, allergen);
            possible_allergen_ingredients.remove(allergen);
            remaining_ingredients.remove(ingredient);
        }
    }

    let mut ingredients: Vec<_> = allergen_assignments.keys().map(|s| s.clone().clone()).collect();
    ingredients.sort_by_key(|ingredient| allergen_assignments.get(ingredient));
    println!("{}", ingredients.join(","));
}

fn get_possible_allergen_ingredients<'a>(foods: &'a Vec<Food>, all_ingredients: &'a HashSet<&String>, allergen: &'a String) -> HashSet<&'a String> {
    foods.iter()
        .filter(|food| food.allergens.contains(allergen))
        .map(|food| HashSet::from_iter(food.ingredients.iter()))
        .fold(all_ingredients.clone(), |a, b| a.intersection(&b).map(|s| *s).collect())
}

fn get_input() -> Vec<Food> {
    util::get_input_lines()
        .map(|line| {
            let (ingredients, mut allergens) = line.split_at(line.find(" (").unwrap());
            allergens = allergens.strip_prefix(" (contains ").unwrap().strip_suffix(")").unwrap();
            Food {
                ingredients: ingredients.split(" ").map(|s| String::from(s)).collect(),
                allergens: allergens.split(", ").map(|s| String::from(s)).collect()
            }
        })
        .collect()
}

struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>
}
