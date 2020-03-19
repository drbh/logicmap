mod degree_audit;
use crate::degree_audit::*;

fn main() {
    let david = Student {
        name: String::from("David"),
        majors: vec![String::from("Marketing"), String::from("Design")],
        classes: vec![
            ClassExperience {
                class: Class {
                    hours: 20,
                    level: 100,
                    subject: "English".to_string(),
                    group: vec!["HU".to_string()],
                },
                when: 0,
                grade: 86,
            },
            ClassExperience {
                class: Class {
                    hours: 4,
                    level: 300,
                    subject: "Math".to_string(),
                    group: vec!["MA".to_string()],
                },
                when: 0,
                grade: 90,
            },
            ClassExperience {
                class: Class {
                    hours: 40,
                    level: 400,
                    subject: "Math".to_string(),
                    group: vec!["MA".to_string(), "SB".to_string()],
                },
                when: 0,
                grade: 80,
            },
            ClassExperience {
                class: Class {
                    hours: 40,
                    level: 400,
                    subject: "Science".to_string(),
                    group: vec!["LA".to_string()],
                },
                when: 0,
                grade: 80,
            },
        ],
    };

    let mut card_configs: Vec<Vec<&dyn Config>> = Vec::new();

    let brick1 = ExactMatch {
        subject: "English".to_string(),
        level: 100,
    };

    let brick2 = ExactMatch {
        subject: "Science".to_string(),
        level: 100,
    };
    // statement 1
    card_configs.push(vec![&brick1, &brick2]);

    let brick3 = SubjectMatch {
        subject: "Math".to_string(),
    };

    let brick4 = GroupMatch {
        group: "MA".to_string(),
    };
    // statement 1
    card_configs.push(vec![&brick3, &brick4]);

    let brick5 = ExactMatch {
        subject: "Science".to_string(),
        level: 200,
    };

    let brick6 = GroupMatch {
        group: "LA".to_string(),
    };
    // statement 1
    card_configs.push(vec![&brick5, &brick6]);

    let card = build_card(david, card_configs);
    println!("{:#?}", card);
    let _finished_card = card.report();
    // println!("{:#?}", david);

    // println!("{:#?}", _finished_card);
}
