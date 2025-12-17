use std::{
    cmp::Ordering,
    fmt,
    fmt::Formatter,
    io::{BufWriter, Write},
    ops::{Deref, DerefMut, Index, IndexMut, Mul, MulAssign},
};

use aoclib_rs::{option_min_max::OptionMinMax, prep_io, printwriteln};

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
    part2(&mut writer, machines);
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

#[derive(Clone, Debug, PartialEq)]
struct RowVec(Vec<R64>);

impl RowVec {
    fn zeros(len: usize) -> Self {
        Self(vec![R64::ZERO; len])
    }

    fn add_assign(&mut self, rhs: &Self) -> anyhow::Result<()> {
        if self.0.len() != rhs.0.len() {
            anyhow::bail!(
                "addition of RowVecs of different sizes: {} vs {}",
                self.0.len(),
                rhs.0.len()
            );
        }

        self.0
            .iter_mut()
            .enumerate()
            .for_each(|(i, e)| *e += rhs.0[i]);

        Ok(())
    }

    fn normalize(&mut self) {
        let Some(leader_col) = self.leader_col() else {
            return;
        };

        if self.0[leader_col] != R64::ONE {
            let factor = self.0[leader_col].recip();
            *self *= factor;
        }
    }

    fn is_zeros(&self) -> bool {
        fold_while(self.0.iter(), true, |_, v| {
            let r = *v == R64::ZERO;
            (r, r)
        })
    }

    fn leader_col(&self) -> Option<usize> {
        self.0.iter().position(|&e| e != R64::ZERO)
    }
}

impl Deref for RowVec {
    type Target = Vec<R64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RowVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl MulAssign<R64> for RowVec {
    fn mul_assign(&mut self, rhs: R64) {
        for cell in &mut self.0 {
            *cell *= rhs;
        }
    }
}

impl Mul<R64> for RowVec {
    type Output = RowVec;

    fn mul(mut self, rhs: R64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl fmt::Display for RowVec {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[ ")?;
        for v in &self.0 {
            write!(f, "{} ", *v)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Matrix(Vec<RowVec>);

impl Matrix {
    fn new(m: Vec<Vec<R64>>) -> Self {
        Self(m.into_iter().map(RowVec).collect())
    }

    fn zeros(rows: usize, cols: usize) -> Self {
        if rows == 0 || cols == 0 {
            Self(Vec::new())
        } else {
            Self(vec![RowVec::zeros(cols); rows])
        }
    }

    fn rref(&mut self) {
        self.r#ref();
        for row in 0..self.0.len() {
            self.eliminate_above_leader(row);
        }
        self.leader_sort();
        self.normalize();
    }

    fn r#ref(&mut self) {
        self.leader_sort();
        for row in 0..self.0.len() {
            self.eliminate_below_leader(row);
        }
        self.leader_sort();
    }

    fn leader_sort(&mut self) {
        self.0.sort_by(|a, b| {
            for (i, ai) in a.iter().enumerate() {
                let bi = b[i];
                if *ai == R64::ZERO && bi != R64::ZERO {
                    return Ordering::Greater;
                } else if bi == R64::ZERO && *ai != R64::ZERO {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        });
    }

    fn eliminate_below_leader(&mut self, row: usize) {
        let Some(leader_col) = self.0[row].leader_col() else {
            return;
        };

        for i in (row + 1)..self.0.len() {
            self.eliminate(row, i, leader_col);
        }
    }

    fn eliminate(&mut self, selected_row: usize, other_row: usize, leader_col: usize) {
        if self.0[other_row][leader_col] == R64::ZERO {
            return;
        }

        let factor = -self.0[other_row][leader_col] / self.0[selected_row][leader_col];
        let term = self.0[selected_row].clone() * factor;
        self.0[other_row].add_assign(&term).unwrap();
    }

    fn normalize(&mut self) {
        self.0.iter_mut().for_each(|row| row.normalize());
    }

    fn eliminate_above_leader(&mut self, row: usize) {
        let Some(leader_col) = self.0[row].leader_col() else {
            return;
        };

        for i in 0..row {
            self.eliminate(row, i, leader_col);
        }
    }

    fn matrix_mul(&self, rhs: &Self) -> anyhow::Result<Self> {
        if (self.0.is_empty() || self.0[0].is_empty()) && (rhs.0.is_empty() || rhs.0[0].len() == 1)
        {
            return Ok(Self(Vec::new()));
        }

        if self.0[0].len() != rhs.0.len() {
            anyhow::bail!(
                "multiplication of incompatible matrices: lhs width {} vs rhs height {}",
                self.0[0].len(),
                rhs.0.len()
            );
        }

        let mut new = Self::zeros(self.0.len(), rhs.0[0].len());
        for i in 0..rhs.0[0].len() {
            for j in 0..self.0.len() {
                for k in 0..self.0[0].len() {
                    new.0[j][i] += self.0[j][k] * rhs.0[k][i];
                }
            }
        }
        Ok(new)
    }

    fn append_row(&mut self, r: RowVec) {
        self.0.push(r);
    }

    fn remove_row(&mut self, i: usize) {
        self.0.remove(i);
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    fn iter(&self) -> impl Iterator<Item = &RowVec> {
        self.0.iter()
    }

    fn get_column_copy(&self, c: usize) -> Self {
        Self::new(self.iter().map(|r| vec![r[c]]).collect())
    }
}

impl MulAssign<R64> for Matrix {
    fn mul_assign(&mut self, rhs: R64) {
        for row in &mut self.0 {
            *row *= rhs;
        }
    }
}

impl Mul<R64> for Matrix {
    type Output = Self;

    fn mul(mut self, rhs: R64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Index<usize> for Matrix {
    type Output = RowVec;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = R64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

impl IntoIterator for Matrix {
    type Item = RowVec;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[ ")?;
        let mut first = true;
        for v in &self.0 {
            if !first {
                write!(f, "\n  ")?;
            }
            first = false;
            write!(f, "{} ", *v)?;
        }
        write!(f, "]")?;
        Ok(())
    }
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
            mat.append_row(RowVec(vec![R64::ZERO; width]));
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
    let mut max_req_button_presses = OptionMinMax(None);
    for j in &buttons[leader_col] {
        max_req_button_presses = max_req_button_presses.min(joltage_reqs[*j]);
    }
    let max_req_button_presses = max_req_button_presses.0.unwrap();

    // iterate and recurse to "count" up all the possible solutions with these free variables, and
    // keep track of the minimum number of button presses per solution as we go.
    let mut min = None;
    let mut mat = mat.clone();
    for i in 0..=max_req_button_presses {
        let width = mat.width() - 1;
        mat[(variable_row, width)] = R64::from_integer(i); // set the free variable to a sample value
        let m = find_min_solution(&mat, buttons, joltage_reqs, original_mat, free_vars - 1);

        // TODO: make OptionMinMax able to handle this scenario more nicely
        if min.is_none() || (m.is_some() && m.unwrap() < min.unwrap()) {
            min = m;
        }
    }

    min
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
    solution.append_row(RowVec(vec![R64::ZERO]));

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

// TODO: move to aoclib-rs
fn fold_while<I, B, F>(it: I, init: B, mut f: F) -> B
where
    I: Iterator,
    F: FnMut(B, I::Item) -> (B, bool),
{
    let mut b = init;
    for v in it {
        let (new_b, should_continue) = f(b, v);
        b = new_b;
        if !should_continue {
            break;
        }
    }
    b
}
