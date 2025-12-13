use std::io::{BufWriter, Write};

use aoclib_rs::{option_min_max::OptionMinMax, prep_io, printwriteln};

#[derive(Debug, Clone)]
struct Machine {
    lights_actual: Vec<Light>,
    lights_goal: Vec<Light>,
    buttons: Vec<Button>,

    #[allow(dead_code)]
    joltage_reqs: Vec<Joltage>,
}

impl Machine {
    fn new(lights_goal: Vec<Light>, buttons: Vec<Button>, joltage_reqs: Vec<Joltage>) -> Self {
        Self {
            lights_actual: Self::default_lights(lights_goal.len()),
            lights_goal,
            buttons,
            joltage_reqs,
        }
    }

    fn default_lights(len: usize) -> Vec<Light> {
        vec![Light::default(); len]
    }

    fn goal_achieved(&self) -> bool {
        self.lights_actual == self.lights_goal
    }

    fn reset(&mut self) {
        self.lights_actual = Self::default_lights(self.lights_actual.len());
    }

    fn min_button_presses(&mut self) -> i64 {
        let mut presses: Presses = vec![false; self.buttons.len()];
        let mut min = OptionMinMax(None);
        loop {
            let num_presses = self.press_buttons(&presses);
            if self.goal_achieved() {
                min = min.min(num_presses);
            }
            self.reset();

            if increment_presses(&mut presses) {
                break;
            }
        }
        min.0.unwrap()
    }

    fn press_buttons(&mut self, presses: &Presses) -> i64 {
        let mut num_presses = 0;
        for (i, press) in presses.iter().enumerate() {
            if *press {
                self.press_button(i);
                num_presses += 1;
            }
        }
        num_presses
    }

    fn press_button(&mut self, i: usize) {
        for b in &self.buttons[i] {
            self.lights_actual[*b].toggle();
        }
    }
}

impl From<&str> for Machine {
    // "[.#.#] (0) (1,2) {2,4,6,8}"
    fn from(line: &str) -> Self {
        // ["[.#.#", "(0) (1,2) {2,4,6,8}"]
        let mut line_split = line.split("] ");

        // ".#.#"
        let lights_str = &line_split.next().unwrap()[1..];
        let lights: Vec<Light> = lights_str.chars().map(Light::from).collect();

        // ["(0) (1,2) ", "2,4,6,8}"]
        let mut line_split_2 = line_split.next().unwrap().split("{");

        // "(0) (1,2) "
        let buttons_str = line_split_2.next().unwrap();

        // ["(0", "(1,2", ""]
        let buttons_split = buttons_str.split(") ");
        let buttons: Vec<Button> = buttons_split
            .filter(|b| !b.is_empty())
            .map(|b| {
                // ["0"]
                // ["1", "2"]
                let b_split = b[1..].split(",");
                b_split.map(|i| i.parse().unwrap()).collect()
            })
            .collect();

        // "2,4,6,8}"
        let joltages_str = line_split_2.next().unwrap();

        // ["2", "4", "6", "8"]
        let joltages_split = joltages_str[..(joltages_str.len() - 1)].split(",");
        let joltages: Vec<Joltage> = joltages_split.map(|j| j.parse().unwrap()).collect();

        Machine::new(lights, buttons, joltages)
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Light {
    On,
    Off,
}

impl Light {
    fn default() -> Self {
        Light::Off
    }

    fn toggle(&mut self) {
        *self = match self {
            Light::On => Light::Off,
            Light::Off => Light::On,
        };
    }
}

impl From<char> for Light {
    fn from(c: char) -> Self {
        match c {
            '#' => Light::On,
            '.' => Light::Off,
            _ => panic!("invalid input"),
        }
    }
}

type Button = Vec<usize>;
type Joltage = i64;
type Presses = Vec<bool>;

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 10).unwrap();
    let machines: Vec<Machine> = contents.iter().map(|line| Machine::from(*line)).collect();

    part1(&mut writer, machines.clone());
}

fn part1<W: Write>(writer: &mut BufWriter<W>, machines: Vec<Machine>) {
    let mut total = 0;
    for mut m in machines {
        total += m.min_button_presses();
    }

    printwriteln!(writer, "{}", total).unwrap();
}

// TODO make an iterator for this in aoclib-rs
fn increment_presses(presses: &mut Presses) -> bool {
    let mut last = presses.len() - 1;
    loop {
        presses[last] = !presses[last];

        if presses[last] {
            break;
        }

        match last.checked_sub(1) {
            None => return true,
            Some(l) => last = l,
        }
    }
    false
}
