use std::fs;

struct Constraint {
    less_than_pointing_cell_in_row: i32,
    greater_than_pointing_cell_in_row: i32,
    less_than_pointing_cell_in_col: i32,
    greater_than_pointing_cell_in_col: i32,
}
pub struct DCNF {
    n: i32,
    vars: i32,
    clauses: i32,
    cnf: Vec<String>,
}

fn main() {
    let mut input_number = String::new();
    let num;
    println!("enter the size of Futoshiki board ?");
    match std::io::stdin().read_line(&mut input_number) {
        Ok(_) => {
            num = input_number.trim().parse().unwrap();
        }

        Err(_) => {
            println!("The input was in wrong format");
            num = 4;
        }
    }

    println!("The size of the board is {}x{}", num, num);
    let mut dcnf = DCNF {
        n: num,
        vars: num * num * num,
        clauses: 0,
        cnf: vec![],
    };

    dcnf.generate();
    dcnf.write_generated_dimacs_to_file("test.cnf");
}

impl DCNF {
    fn generate(&mut self) {
        self.pre_assigned_values();
        self.assign_digits_on_all_cells();
        self.each_digit_only_once_in_row();
        self.each_digit_only_once_in_column();
        self.add_constraints(true);
    }
    fn pre_assigned_values(&mut self) {
        self.cnf.push("c preassigned entries".to_string());
        //make facts = 2 and uncomment for unsatisfiable condition
        let facts = 1;
        self.cnf.push(format!("{} 0", self.to_variable(2, 2, 2)));
        // unsatisfiable condition
        // self.cnf.push(format!("{} 0", self.to_variable(3, 1, 2)));

        self.clauses += facts;
    }

    fn to_variable(&self, digit: i32, row: i32, col: i32) -> i32 {
        self.n * self.n * (digit - 1) + self.n * (row - 1) + (col - 1) + 1
    }

    fn add_comment(&mut self, ins: &str) {
        self.cnf.push(format!("c {} ", ins));
    }

    fn assign_digits_on_all_cells(&mut self) {
        self.cnf.push("c Atleast one digit on cell".to_string());
        let mut my_str: String;
        for row in 1..=self.n {
            for col in 1..=self.n {
                my_str = "".to_string();
                for digit in 1..=self.n {
                    my_str = format!(
                        "{} {} ",
                        my_str,
                        self.to_variable(digit, row, col).to_string()
                    );
                }
                self.cnf.push(format!("{}0", my_str));
                self.clauses += 1;
            }
        }
    }

    fn each_digit_only_once_in_row(&mut self) {
        self.cnf
            .push("c Each Digit should appear only once in each row".to_string());
        for digit in 1..=self.n {
            for row in 1..=self.n {
                for column_low in 1..=(self.n - 1) {
                    for column_high in column_low + 1..=self.n {
                        self.cnf.push(format!(
                            "-{} -{} 0",
                            self.to_variable(digit, row, column_low),
                            self.to_variable(digit, row, column_high)
                        ));
                        self.clauses += 1;
                    }
                }
            }
        }
    }
    fn each_digit_only_once_in_column(&mut self) {
        self.cnf
            .push("c each digit should appear only once in each column".to_string());
        for digit in 1..=self.n {
            for col in 1..=self.n {
                for row_low in 1..self.n {
                    for row_high in row_low + 1..=self.n {
                        self.cnf.push(format!(
                            "-{} -{} 0",
                            self.to_variable(digit, row_low, col),
                            self.to_variable(digit, row_high, col)
                        ));
                        self.clauses += 1;
                    }
                }
            }
        }
    }

    fn get_constraints(&self, with_solution: bool) -> Vec<Constraint> {
        let mut cons = vec![
            Constraint {
                less_than_pointing_cell_in_row: 2,
                less_than_pointing_cell_in_col: 2,
                greater_than_pointing_cell_in_row: 1,
                greater_than_pointing_cell_in_col: 2,
            },
            Constraint {
                less_than_pointing_cell_in_row: 3,
                less_than_pointing_cell_in_col: 1,
                greater_than_pointing_cell_in_row: 2,
                greater_than_pointing_cell_in_col: 1,
            },
            Constraint {
                less_than_pointing_cell_in_row: 1,
                less_than_pointing_cell_in_col: 4,
                greater_than_pointing_cell_in_row: 2,
                greater_than_pointing_cell_in_col: 4,
            },
            Constraint {
                less_than_pointing_cell_in_row: 4,
                less_than_pointing_cell_in_col: 2,
                greater_than_pointing_cell_in_row: 4,
                greater_than_pointing_cell_in_col: 1,
            },
        ];
        if !with_solution {
            // This is contradictory constraint
            cons.push(Constraint {
                less_than_pointing_cell_in_row: 4,
                less_than_pointing_cell_in_col: 1,
                greater_than_pointing_cell_in_row: 3,
                greater_than_pointing_cell_in_col: 1,
            })
        }
        cons
    }

    fn add_constraints(&mut self, satisfy: bool) {
        //get constraints with solution
        let constraints: Vec<Constraint> = self.get_constraints(satisfy);
        self.cnf.push("c Adding Constraints ".to_string());
        let mut counter = 0;
        for constraint in constraints.iter() {
            for digit_low in 1..self.n {
                for digit_high in (digit_low + 1)..=self.n {
                    counter += 1;
                    println!("test {}", counter);
                    self.cnf.push(format!(
                        "-{} -{} 0",
                        self.to_variable(
                            digit_low,
                            constraint.less_than_pointing_cell_in_row,
                            constraint.less_than_pointing_cell_in_col
                        ),
                        self.to_variable(
                            digit_high,
                            constraint.greater_than_pointing_cell_in_row,
                            constraint.greater_than_pointing_cell_in_col
                        )
                    ));
                    self.clauses += 1;
                }
            }
        }
    }

    fn write_generated_dimacs_to_file(&mut self, filename: &str) {
        let mut big_string: String = String::new();
        big_string.push_str(format!("p cnf {} {}\n", self.vars, self.clauses).as_str());
        for st in &self.cnf {
            big_string.push_str(st);
            big_string.push_str("\n");
        }

        //write to file
        match fs::write(filename, big_string) {
            Ok(_) => {
                println!("generated Successfully");
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
