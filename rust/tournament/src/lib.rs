use std::{cmp::Ordering, collections::HashMap};

#[derive(Default)]
struct Team {
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}
impl Team {
    // the first team
    pub fn add_first(&mut self, state: &str) {
        self.mp += 1;

        match state {
            "win" => {
                self.w += 1;
                self.p += 3;
            }
            "draw" => {
                self.d += 1;
                self.p += 1;
            }
            "loss" => {
                self.l += 1;
            }
            _ => return,
        };
    }

    // the second team
    pub fn add_another(&mut self, state: &str) {
        self.mp += 1;

        match state {
            "win" => self.l += 1,
            "draw" => {
                self.p += 1;
                self.d += 1;
            }
            "loss" => {
                self.p += 3;
                self.w += 1;
            }
            _ => return,
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let teams: Vec<&str> = match_results.split(&[';', '\n'][..]).collect();
    // use HashMap to storage the name and data
    let mut map: HashMap<String, Team> = HashMap::new();
    let mut output = String::from("Team                           | MP |  W |  D |  L |  P");
    if teams.len() < 3 {
        return output;
    }
    for i in (0..teams.len()).step_by(3) {
        // progress two teams
        let team1 = map.entry(teams[i].to_string()).or_default();
        team1.add_first(teams[i + 2]);
        let team2 = map.entry(teams[i + 1].to_string()).or_default();
        team2.add_another(teams[i + 2]);
    }

    // for sort the answers, change into Vector
    let mut vec = map.iter().collect::<Vec<(&String, &Team)>>();
    vec.sort_by(|a, b| {
        if a.1.p > b.1.p {
            return Ordering::Less;
        }
        if a.1.p < b.1.p {
            return Ordering::Greater;
        }
        return a.0.cmp(b.0);
    });

    for (k, v) in vec {
        output += &format!(
            "\n{:31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            k, v.mp, v.w, v.d, v.l, v.p
        );
    }

    output
}
