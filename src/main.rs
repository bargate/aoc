use aoc::{Runner, Selector, days::Aoc2023_01};

fn main() {
    run_2023(Selector::All);
}

fn run_2023(which: Selector) {
    let mut day01 = Aoc2023_01::new();

    let mut days: Vec<&mut dyn Runner> = vec![ &mut day01 ];

    match which {
        Selector::Last => {
            let last = days.len() - 1;
            let d = &mut days[last];
            aoc::run_solution(*d);
        }
        Selector::All => {
            for d in days {
                aoc::run_solution(d);
            }
        }
        Selector::One(num) => {
            let d = &mut days[num - 1];
            aoc::run_solution(*d);
        }
    }
}
