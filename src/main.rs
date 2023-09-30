/*
  Input:
    - puzzle constraints? (just classic 9x9 for now?)
    - grid
    - technique (naked_single, hidden_single etc.)
    - user_solution_steps (could be optional and if not passed generate our own)
  Output: list of (grid, [cell position + value that would apply the technique])
*/
use std::{env, rc::Rc};
use itertools::Itertools;
use serde::Deserialize;
use lisudoku_solver::{types::{CellPosition, Grid, FixedNumber, SudokuConstraints, SolutionType, SudokuGrid}, solver::{Solver, logical_solver::technique::Technique}};

#[derive(Debug, Deserialize)]
struct UserSolutionStep {
    r#type: String,
    value: Option<u32>,
    cells: Vec<CellPosition>,
    #[allow(dead_code)]
    time: u32,
}

type ResultItem = (String, Vec<FixedNumber>);

fn main() {
  let args: Vec<String> = env::args().collect();
  assert_eq!(args.len(), 4, "Usage: initial_grid technique user_solution");
  let initial_grid_str = &args[1];
  let technique_str = &args[2];
  let user_solution_steps_str = &args[3];

  // TODO: make user_solution_steps optional, use solver to generate steps
  let result = run(initial_grid_str, technique_str, user_solution_steps_str);

  let result_str = serde_json::to_string(&result).unwrap();

  println!("{}", result_str);
}

fn run(initial_grid_str: &String, technique_str: &String, user_solution_steps_str: &String) -> Vec<ResultItem> {
  let mut grid = SudokuGrid::from_string(initial_grid_str.clone()).values;

  let technique = Solver::default_techniques()
    .into_iter()
    .find(|technique| technique.get_rule().to_string() == technique_str.clone())
    .expect("Invalid technique passed");

  let user_solution_steps: Vec<UserSolutionStep> = serde_json::from_str(&user_solution_steps_str).expect("JSON was not well-formatted");

  let mut result = vec![];

  // Also run for the initial grid
  find_grid_steps(grid.to_vec(), technique.clone(), &mut result);

  for step in user_solution_steps {
    match step.r#type.as_str() {
      "digit" => {
        for cell in step.cells {
          assert_eq!(grid[cell.row][cell.col], 0);
          grid[cell.row][cell.col] = step.value.unwrap();
        }
      },
      "delete" => {
        for cell in step.cells {
          assert_ne!(grid[cell.row][cell.col], 0);
          grid[cell.row][cell.col] = 0;
        }
      },
      _ => { continue },
    };

    find_grid_steps(grid.to_vec(), technique.clone(), &mut result);
  }

  result.into_iter()
    .unique_by(|item| item.0.clone())
    .collect()
}

fn find_grid_steps(grid: Grid, technique: Rc<dyn Technique>, result: &mut Vec<(String, Vec<FixedNumber>)>) {
  let min_empty_cell_count = grid.len() * grid.len() / 4;
  if count_empty_cells(&grid) < min_empty_cell_count {
    return
  }
  let solver_steps = run_solver(grid.to_vec(), technique.clone());
  if solver_steps.is_empty() {
    return
  }

  result.push((SudokuGrid::new(grid).to_string(None), solver_steps));
}

fn count_empty_cells(grid: &Grid) -> usize {
  grid.iter().map(|row| row.iter().filter(|&&cell| cell == 0).count()).sum()
}

fn run_solver(grid: Grid, technique: Rc<dyn Technique>) -> Vec<FixedNumber> {
  let grid_size = grid.len();
  let fixed_numbers = SudokuGrid::new(grid).to_fixed_numbers();
  let mut solver = Solver::new(SudokuConstraints::new(grid_size, fixed_numbers), None)
    .with_single_step_mode()
    .with_techniques(vec![ technique ]);
  let res = solver.logical_solve();
  if res.solution_type == SolutionType::None {
    return vec![]
  }
  res.steps.into_iter()
    .map(|step| FixedNumber {
      position: step.cells[0],
      value: step.values[0],
    })
    .collect()
}

#[test]
fn check_result() {
  let grid = String::from("3000010002100000");
  let technique = String::from("HiddenSingle");
  let user_solution_steps_str = String::from("[{\"type\":\"digit\",\"cells\":[{\"row\":0,\"col\":3}],\"value\":1,\"time\":16},{\"type\":\"digit\",\"cells\":[{\"row\":3,\"col\":0}],\"value\":1,\"time\":18},{\"type\":\"delete\",\"cells\":[{\"row\":3,\"col\":0}],\"value\":null,\"time\":18},{\"type\":\"digit\",\"cells\":[{\"row\":3,\"col\":0}],\"value\":1,\"time\":18},{\"type\":\"digit\",\"cells\":[{\"row\":3,\"col\":1}],\"value\":3,\"time\":18},{\"type\":\"digit\",\"cells\":[{\"row\":2,\"col\":0}],\"value\":4,\"time\":20},{\"type\":\"digit\",\"cells\":[{\"row\":2,\"col\":3}],\"value\":3,\"time\":21},{\"type\":\"digit\",\"cells\":[{\"row\":1,\"col\":2}],\"value\":3,\"time\":23},{\"type\":\"digit\",\"cells\":[{\"row\":1,\"col\":0}],\"value\":2,\"time\":32},{\"type\":\"digit\",\"cells\":[{\"row\":1,\"col\":3}],\"value\":4,\"time\":33},{\"type\":\"digit\",\"cells\":[{\"row\":0,\"col\":2}],\"value\":2,\"time\":34},{\"type\":\"digit\",\"cells\":[{\"row\":0,\"col\":1}],\"value\":4,\"time\":35},{\"type\":\"digit\",\"cells\":[{\"row\":3,\"col\":2}],\"value\":4,\"time\":36},{\"type\":\"digit\",\"cells\":[{\"row\":3,\"col\":3}],\"value\":2,\"time\":38}]");
  let result = run(&grid, &technique, &user_solution_steps_str);
  assert_eq!(result.len(), 12);
}
