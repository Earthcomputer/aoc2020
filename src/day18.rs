use crate::util;

pub fn run_easy() {
    run_task(false);
}

pub fn run_hard() {
    run_task(true);
}

fn run_task(with_precedence: bool) {
    let mut result = 0;
    for expr in get_input() {
        result += evaluate_expr(expr, with_precedence);
    }
    println!("{}", result);
}

fn evaluate_expr(mut expression: Vec<String>, with_precedence: bool) -> i64 {
    // special case for single integer
    if expression.len() == 1 {
        return expression[0].parse().unwrap();
    }

    // get left expression
    let mut index = expression.len() as i32 - 1;
    index = skip_part(&expression, index);

    if index < 0 {
        // whole expression is wrapped in parentheses
        return evaluate_expr(expression[1..expression.len() - 1].to_owned(), with_precedence);
    }

    if with_precedence {
        let mut multiply_index = index;
        while multiply_index >= 0 && expression[multiply_index as usize] != "*" {
            multiply_index -= 1;
            multiply_index = skip_part(&expression, multiply_index);
        }
        if multiply_index >= 0 {
            index = multiply_index;
        }
    }

    let left_operand = evaluate_expr(expression[..index as usize].to_owned(), with_precedence);
    let operator = expression[index as usize].clone();
    let right_operand = evaluate_expr(expression[index as usize+1..].to_owned(), with_precedence);

    return match &*operator {
        "+" => left_operand + right_operand,
        "*" => left_operand * right_operand,
        _ => unreachable!()
    }
}

fn skip_part(expression: &Vec<String>, mut index: i32) -> i32 {
    if expression[index as usize] == ")" {
        index -= 1;
        let mut paren_count = 1;
        while paren_count > 0 {
            if expression[index as usize] == ")" {
                paren_count += 1;
            } else if expression[index as usize] == "(" {
                paren_count -= 1;
            }
            index -= 1;
        }
    } else {
        index -= 1;
    }
    return index;
}

fn tokenize(mut expr: String) -> Vec<String> {
    expr = expr.replace(" ", "");
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < expr.len() {
        let mut token = String::new();
        while i < expr.len() && expr.chars().nth(i).unwrap().is_numeric() {
            token += &*expr.chars().nth(i).unwrap().to_string();
            i += 1;
        }
        if !token.is_empty() {
            tokens.push(token);
        }
        if i < expr.len() {
            tokens.push(String::from(expr.chars().nth(i).unwrap()));
            i += 1;
        }
    }
    return tokens;
}

fn get_input() -> impl Iterator<Item = Vec<String>> {
    util::get_input_lines()
        .map(|expr| tokenize(expr))
}
