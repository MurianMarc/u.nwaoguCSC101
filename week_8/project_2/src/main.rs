struct Candidate {
    name: String,
    years_experience: u32,
}

fn main() {
    let candidates = vec![
        Candidate { name: String::from("uchenna"), years_experience: 1 },
        Candidate { name: String::from("Timothy"), years_experience: 6 },
        Candidate { name: String::from("miss. riri"), years_experience: 7 },
        Candidate { name: String::from("dr. moru"), years_experience: 11 },
        Candidate { name: String::from("mr. Chudi"), years_experience: 10 },
    ];

    let mut most_experienced = &candidates[0];

    for candidate in &candidates {
        if candidate.years_experience > most_experienced.years_experience {
            most_experienced = candidate;
        }
    }

    println!(
        "The most experienced candidate is {} with {} years of experience.",
        most_experienced.name, most_experienced.years_experience
    );
}