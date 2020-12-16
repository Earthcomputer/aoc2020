use crate::util;

pub fn run_easy() {
    let document = parse_input();
    let error_rate: i32 = document.nearby_tickets.iter()
        .flat_map(|v| v.iter())
        .filter(|i| document.fields.iter()
            .flat_map(|(_, ranges)| ranges.iter())
            .all(|(min, max)| **i < *min || **i > *max))
        .sum();
    println!("{}", error_rate);
}

pub fn run_hard() {
    let mut document = parse_input();

    // Filter invalid tickets
    document.nearby_tickets = document.nearby_tickets.iter()
        .filter(|ticket| !ticket.iter()
            .any(|i| document.fields.iter()
                .flat_map(|(_, ranges)| ranges.iter())
                .all(|(min, max)| *i < *min || *i > *max)))
        .map(|ticket| ticket.clone())
        .collect();

    let mut ticket_to_field = vec![-1; document.fields.len()];
    let mut field_to_ticket = vec![-1; document.fields.len()];
    let mut num_found = 0;
    while num_found < ticket_to_field.len() {
        let unmatched: Vec<_> = (0..document.fields.len()).filter(|i| ticket_to_field[*i] == -1).collect();
        for out in unmatched {
            let possible_fields: Vec<_> = document.fields.iter()
                .enumerate()
                .filter(|(i, _)| field_to_ticket[*i] == -1)
                .filter(|(_, (_, ranges))| document.nearby_tickets.iter()
                    .all(|ticket| ranges.iter()
                        .any(|(min, max)| ticket[out] >= *min && ticket[out] <= *max)))
                .map(|(i, _)| i)
                .collect();
            if possible_fields.len() == 1 {
                ticket_to_field[out] = possible_fields[0] as i32;
                field_to_ticket[possible_fields[0]] = out as i32;
                num_found += 1;
            }
        }
    }

    let product: i64 = document.fields.iter()
        .enumerate()
        .filter(|(_, (name, _))| name.starts_with("departure"))
        .map(|(i, _)| document.your_ticket[field_to_ticket[i] as usize] as i64)
        .product();
    println!("{}", product);
}

fn parse_input() -> Document {
    let lines: Vec<Vec<String>> = util::get_input_lines().collect::<Vec<String>>().as_slice().split(|s| s.is_empty()).map(|s| s.iter().map(|s| s.clone()).collect()).collect();
    let fields = lines[0].iter().map(|s| {
        let (name, mut rest) = s.split_at(s.find(": ").unwrap());
        rest = rest.strip_prefix(": ").unwrap();
        (String::from(name), rest.split(" or ").map(|s| {
            let (a, mut b) = s.split_at(s.find("-").unwrap());
            b = b.strip_prefix("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        }).collect())
    }).collect();
    let your_ticket = lines[1][1].split(",").map(|s| s.parse().unwrap()).collect();
    let nearby_tickets = lines[2].iter().skip(1).map(|ls| ls.split(",").map(|s| s.parse().unwrap()).collect()).collect();
    return Document {
        fields,
        your_ticket,
        nearby_tickets
    };
}

struct Document {
    fields: Vec<(String, Vec<(i32, i32)>)>,
    your_ticket: Vec<i32>,
    nearby_tickets: Vec<Vec<i32>>
}
