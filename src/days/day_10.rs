use std::io::{BufWriter, Write};

use aoclib_rs::{
    fold_while,
    iter::selector_iter,
    matrix::{Matrix, RowVec},
    option_min_max::OptionMinMax,
    prep_io, printwriteln,
};

use num_rational::Rational64 as R64;

#[derive(Debug, Clone)]
struct Machine {
    lights_actual: Vec<Light>,
    lights_goal: Vec<Light>,
    buttons: Vec<Button>,
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
        let mut min = OptionMinMax::NONE;
        for presses in selector_iter(self.buttons.len()) {
            let num_presses = self.press_buttons(&presses);
            if self.goal_achieved() {
                min = min.min(num_presses);
            }
            self.reset();
        }
        min.unwrap()
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
    part2(&mut writer, machines);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, machines: Vec<Machine>) {
    let mut total = 0;
    for mut m in machines {
        total += m.min_button_presses();
    }

    printwriteln!(writer, "{}", total).unwrap();
}

const VERBOSE: bool = false;

// very slow...
fn part2<W: Write>(writer: &mut BufWriter<W>, machines: Vec<Machine>) {
    let mut total = R64::ZERO;
    for m in &machines {
        if VERBOSE {
            println!("{:?}", m.buttons);
            println!("{:?}", m.joltage_reqs);
        }

        // make a matrix representing the relationship between joltages and button presses.
        // For an example scenario, an input of "(1, 3) (4) (2, 4, 5) (0) (3) {4,3,1,8,3,1}" will give a
        // matrix that initially looks like:
        //
        // [ [ 0 0 0 1 0 1 0 ]
        //   [ 1 0 0 0 0 1 0 ]
        //   [ 0 0 1 0 0 0 0 ]
        //   [ 1 0 0 0 1 0 0 ]
        //   [ 0 1 1 0 0 0 0 ]
        //   [ 0 0 1 0 0 0 0 ]
        //
        // Where each column except the rightmost encodes the effects of one button.
        let mut mat = Matrix::zeros(m.joltage_reqs.len(), m.buttons.len() + 1);
        for (i, b) in m.buttons.iter().enumerate() {
            for c in b {
                mat[(*c, i)] = R64::ONE;
            }
        }

        // finish the matrix from above by putting the joltage values in the last column:
        //
        // [ [ 0 0 0 1 0 1 6 ]
        //   [ 1 0 0 0 0 1 5 ]
        //   [ 0 0 1 0 0 0 1 ]
        //   [ 1 0 0 0 1 0 8 ]
        //   [ 0 1 1 0 0 0 3 ]
        //   [ 0 0 1 0 0 0 1 ] ]
        //
        // This matrix now represents a series of linear equations:
        //
        // * row 0:  b3 + b5 = 6
        // * row 1:  b0 + b5 = 5
        // * row 2:     b2   = 1
        // * row 3:  b0 + b4 = 8
        // * row 4:  b1 + b2 = 3
        // * row 5:     b2   = 1
        //
        // where each bi is the number of presses of button i.
        for (i, j) in m.joltage_reqs.iter().enumerate() {
            mat[(i, m.buttons.len())] = R64::from_integer(*j);
        }

        if VERBOSE {
            println!("{}", &mat);
            println!();
        }

        let original_mat = mat.clone();

        // put the matrix into reduced row echelon form to "solve" it.
        //
        // [ [ 1 0 0 0 0 1 5 ]
        //   [ 0 1 0 0 0 0 2 ]
        //   [ 0 0 1 0 0 0 1 ]
        //   [ 0 0 0 1 0 1 6 ]
        //   [ 0 0 0 0 1 -1 3 ]
        //   [ 0 0 0 0 0 0 0 ] ]
        mat.rref();
        if VERBOSE {
            println!("{}", &mat);
            println!();
        }

        // but there is no unique solution in these problems, so we have to find the columns that
        // have more than one non-zero value - these represent variables that we'll have to iterate
        // through to find specific solutions.
        let iec = inexact_columns(&mat);
        if VERBOSE {
            println!("{:?}", iec);
        }

        // for each new variable, we add a row like [ 0 0 0 0 0 1 0 ], which essentially says "bi = 0".
        // It's important that we keep this unsorted for now so we know which rows are the ones we
        // can iterate through.
        //
        // [ [ 1 0 0 0 0 1 5 ]
        //   [ 0 1 0 0 0 0 2 ]
        //   [ 0 0 1 0 0 0 1 ]
        //   [ 0 0 0 1 0 1 6 ]
        //   [ 0 0 0 0 1 -1 3 ]
        //   [ 0 0 0 0 0 0 0 ]
        //   [ 0 0 0 0 0 1 0 ] ]
        for col in &iec {
            let width = mat.width();
            mat.append_row(RowVec::zeros(width));
            let row = mat.height() - 1;
            mat[(row, *col)] = R64::ONE;
        }

        // remove all-zero rows, they are useless
        //
        // [ [ 1 0 0 0 0 1 5 ]
        //   [ 0 1 0 0 0 0 2 ]
        //   [ 0 0 1 0 0 0 1 ]
        //   [ 0 0 0 1 0 1 6 ]
        //   [ 0 0 0 0 1 -1 3 ]
        //   [ 0 0 0 0 0 1 0 ] ]
        let mut i = 0;
        loop {
            if i >= mat.height() {
                break;
            }
            if mat[i].is_zeros() {
                mat.remove_row(i);
                i = 0;
            } else {
                i += 1;
            }
        }

        if mat.width() != mat.height() + 1 {
            panic!(
                "matrix should be essentially square but with one extra column for after the = sign. Got height {} and width {}",
                mat.height(),
                mat.width()
            );
        }

        if VERBOSE {
            println!("{}", &mat);
            println!();
        }

        let min =
            find_min_solution(&mat, &m.buttons, &m.joltage_reqs, &original_mat, iec.len()).unwrap();
        println!("MIN: {}", min);
        total += min;

        if VERBOSE {
            println!();
        }
    }
    printwriteln!(writer, "{}", total).unwrap();
}

const DOUBLE_CHECK: bool = false;

fn find_min_solution(
    mat: &Matrix,
    buttons: &Vec<Button>,
    joltage_reqs: &Vec<Joltage>,
    original_mat: &Matrix,
    free_vars: usize,
) -> Option<R64> {
    // base case - there are no free variables remaining
    if free_vars == 0 {
        let mut mat = mat.clone();

        // solve the matrix and check if it's a valid solution for this problem (ie, if it contains
        // no negative or fractional values)
        mat.rref();
        if !is_valid(&mat) {
            return None;
        }

        // double-check that this actually is a real solution to the matrix and there's not a bug
        // in the code
        if DOUBLE_CHECK && !is_real_solution(&mat, original_mat, joltage_reqs) {
            println!("{}", &mat);
            panic!("incorrect solution found ^^");
        }
        if VERBOSE {
            println!("{}", &mat);
        }

        // the final total number of button presses for this solution is the sum of the rightmost
        // column.
        let mut total = R64::ZERO;
        for i in 0..mat.height() {
            total += mat[(i, mat.width() - 1)];
        }
        if VERBOSE {
            println!("{}", total);
            println!();
        }
        return Some(total);
    }

    // if we do have free variables left, then pick a relevant row to work with for now and find
    // its leader column
    let variable_row = mat.height() - free_vars;
    let leader_col = mat[variable_row].leader_col().unwrap();

    // for that column, we need to iterate from 0 up to (and including) the magnitude of the
    // smallest joltage that this column (button) affects.
    let mut max_req_button_presses = OptionMinMax::NONE;
    for j in &buttons[leader_col] {
        max_req_button_presses = max_req_button_presses.min(joltage_reqs[*j]);
    }
    let max_req_button_presses = max_req_button_presses.unwrap();

    // iterate and recurse to "count" up all the possible solutions with these free variables, and
    // keep track of the minimum number of button presses per solution as we go.
    let mut min = OptionMinMax::NONE;
    let mut mat = mat.clone();
    for i in 0..=max_req_button_presses {
        let width = mat.width() - 1;
        mat[(variable_row, width)] = R64::from_integer(i); // set the free variable to a sample value
        let m = find_min_solution(&mat, buttons, joltage_reqs, original_mat, free_vars - 1);
        min = min.min_option(m);
    }

    min.get()
}

fn is_valid(m: &Matrix) -> bool {
    fold_while(m.iter(), true, |_, row| {
        let r = fold_while(row.iter(), true, |_, v| {
            let r = *v < R64::ZERO || *v.denom() != 1;
            (!r, !r)
        });
        (r, r)
    })
}

fn inexact_columns(m: &Matrix) -> Vec<usize> {
    let mut inexact = Vec::new();
    for c in 0..(m.width() - 1) {
        if col_is_inexact(m, c) {
            inexact.push(c);
        }
    }
    inexact
}

fn col_is_inexact(m: &Matrix, c: usize) -> bool {
    m.iter().filter(|v| v[c] != R64::ZERO).count() != 1
}

fn is_real_solution(mat: &Matrix, original_mat: &Matrix, joltage_reqs: &[Joltage]) -> bool {
    // extract the rightmost column of the solution into its own column-vector, then add an extra 0
    // entry at the bottom.
    let mut solution = mat.get_column_copy(mat.width() - 1);
    solution.append_row(RowVec::zeros(1));

    // we expect the result of matrix-multiplying the original matrix by the solution column-vector
    // to equal a column-vector of the joltage requirements. For example, for the original matrix
    // above:
    //
    //     [ [ 0 0 0 1 0 1 6 ]
    //       [ 1 0 0 0 0 1 5 ]
    // A =   [ 0 0 1 0 0 0 1 ]
    //       [ 1 0 0 0 1 0 8 ]
    //       [ 0 1 1 0 0 0 3 ]
    //       [ 0 0 1 0 0 0 1 ] ]
    //
    // and a given solution of the column-matrix (with an extra zero at the bottom):
    //     [ [ 4 ]
    //       [ 2 ]
    // x =   [ 1 ]
    //       [ 5 ]
    //       [ 4 ]
    //       [ 1 ]
    //       [ 0 ] ]
    //
    // we see that
    //
    //         [ [ 6 ]
    //           [ 5 ]
    // A * x =   [ 1 ]
    //           [ 8 ]
    //           [ 3 ]
    //           [ 1 ] ]
    original_mat.matrix_mul(&solution).unwrap()
        == Matrix::new(
            joltage_reqs
                .iter()
                .map(|j| vec![R64::from_integer(*j)])
                .collect(),
        )
}
