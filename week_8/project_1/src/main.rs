use std::collections::HashMap;

fn main() {
    let mut roles_by_level: HashMap<&str, Vec<&str>> = HashMap::new();

    roles_by_level.insert("APS 1-2", vec!["Intern", "Paralegal", "Placement"]);
    roles_by_level.insert("APS 3-5", vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"]);
    roles_by_level.insert("APS 5-8", vec!["Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"]);
    roles_by_level.insert("EL1 8-10", vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"]);
    roles_by_level.insert("EL2 10-13", vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"]);
    roles_by_level.insert("SES", vec!["CEO", "Dean", "Partner", "Principal"]);

    let staff_role = "Associate";
    let years_experience = 6;

    let aps_level = determine_aps_level(&roles_by_level, staff_role, years_experience);
    match aps_level {
        Some(level) => println!("Staff with role '{}' and {} years of experience is at level: {}", staff_role, years_experience, level),
        None => println!("No matching APS level found for role '{}' with {} years of experience.", staff_role, years_experience),
    }
}

fn determine_aps_level<'a>(
    roles: &'a HashMap<&'a str, Vec<&'a str>>,
    role: &'a str,
    years: u32
) -> Option<&'a str> {
    let ordered_levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    for level in ordered_levels {
        if let Some(titles) = roles.get(level) {
            if titles.contains(&role) {
                match level {
                    "APS 1-2" if years <= 2 => return Some(level),
                    "APS 3-5" if years >= 3 && years <= 5 => return Some(level),
                    "APS 5-8" if years >= 5 && years <= 8 => return Some(level),
                    "EL1 8-10" if years >= 8 && years <= 10 => return Some(level),
                    "EL2 10-13" if years >= 10 && years <= 13 => return Some(level),
                    "SES" if years > 13 => return Some(level),
                    _ => continue,
                }
            }
        }
    }
    None
}