// Copyright 2016 Pavel Kiselev

// My implementation of Eller's Algorithm, following instructions at http://www.neocomputer.org/projects/eller.html

#![allow(dead_code)]


use rand::{Rng,thread_rng,ThreadRng};
use std::collections::HashMap;
use std::fmt;


#[derive(Debug,Clone,Copy)]
struct Cell {
    set: usize,
    rw: bool,
    bw: bool,
}

pub struct EllerMaze {
    maze: Vec<Vec<Cell>>,
}

// Somewhat changes look of generated mazes
pub enum MazeOrient {
    Normal,
    Vertical,
    Horizontal,
}

impl EllerMaze {
    pub fn generate(width: usize, height: usize, orient: MazeOrient) -> EllerMaze {

        let mut rng = thread_rng();
        
        //Closure for changing the maze generated (as proposed on neocomputer.org in "Example
        //mazes" section
        let verticality = |orient: &MazeOrient, rng: &mut ThreadRng, is_right: bool| -> bool {
            if is_right {
                match *orient {
                    MazeOrient::Normal => rng.gen(),
                    MazeOrient::Vertical => rng.gen() || rng.gen(),
                    MazeOrient::Horizontal => rng.gen() && rng.gen(),
                }
            } else {
                match *orient {
                    MazeOrient::Normal => rng.gen(),
                    MazeOrient::Vertical => rng.gen() && rng.gen(),
                    MazeOrient::Horizontal => rng.gen() || rng.gen(),
                }
            }
        };

        let mut maze: Vec<Vec<Cell>> = Vec::with_capacity(height);
        
        // 1. Create the first row. No cells will be members of any set
        let mut row = vec![Cell { set: 0, rw: false, bw: false }; width];

        row[width-1].rw = true; // The last one always has a wall.

        let mut sets: HashMap<usize, Vec<usize>> = HashMap::with_capacity(width);

        for count_iterations in 0..height {
            
            sets.drain(); // Empty the db
            
            // Create a database of indexes for all set members
            for (i,cell) in row.iter().enumerate() {
                if sets.contains_key(&cell.set) {
                    let mut vec = sets.get_mut(&cell.set).expect("Cannot get sets to compete the HashMap");
                    (*vec).push(i);
                } else {
                    sets.insert(cell.set, vec![i]);
                }
            }

            // 2. Join any cells not members of a set to their own unique set
            let no_set = sets.remove(&0).unwrap_or(vec![]); //Get indexes of cells without set, or init with empty vec
            for i in no_set {
                let mut j = 1;
                while row[i].set == 0 {
                    if sets.contains_key(&j) {
                        j += 1;
                    } else {
                        row[i].set = j;
                        sets.insert(j, vec![i]);
                    }
                }
            }

            // 3. Create right-walls, moving from left to right:
            for i in 0..row.len()-1 {
                // - If the current cell and the cell to the right are members of the same set, always create a wall between them. (This prevents loops)
                if row[i].set == row[i+1].set {
                    row[i].rw = true;
                    continue;
                }
                // - Randomly decide to add a wall or not:
                if verticality(&orient, &mut rng, true) {
                    row[i].rw = true;
                } else {
                    // - If you decide not to add a wall, union the sets to which the current cell and the cell to the right are members.
                    let mut set2 = sets.remove(&row[i+1].set).expect("Can't get set for union");
                    let mut set1 = sets.get_mut(&row[i].set).expect("Can't get set to union with");
                    for j in set2.iter() {
                        row[*j].set = row[i].set;
                    }
                    set1.append(&mut set2);
                }
            }

            // 4. Create bottom-walls, moving from left to right:
            // 4.1. Randomly decide to add a wall or not. Make sure that each set has at least one cell without a bottom-wall (This prevents isolations)
            // - If a cell is the only member of its set, do not create a bottom-wall
            // - If a cell is the only member of its set without a bottom-wall, do not create a bottom-wall
            let mut wall_sets = sets.clone();
            for (i, cell) in row.iter_mut().enumerate() {
                if verticality(&orient, &mut rng, false) {
                    let mut set = wall_sets.get_mut(&cell.set).expect("Cannot get set to ensure correct bottom walls creation");
                    if set.len() > 1 {
                        cell.bw = true;
                        set.retain(|&x| x != i); // Maybe not optimal. Remove the element
                    }
                }
            }

            // 5. Decide to keep adding rows, or stop and complete the maze
            // 5.1. If you decide to add another row:
            if !(count_iterations == height-1) {
                // 5.1.1. Output the current row
                maze.push(row.clone());
                for cell in row.iter_mut() {
                    // 5.1.2. Remove all right walls
                    if cell.rw {
                        cell.rw = false;
                    }
                    if cell.bw {
                        // 5.1.3. Remove cells with a bottom-wall from their set
                        cell.set = 0;
                        // 5.1.4. Remove all bottom walls
                        cell.bw = false;
                    }
                }
                row[width-1].rw = true; // return the right-most wall

            // 5.1.5. Continue from Step 2
            }
        }

        // 5.2. If you decide to complete the maze
        // 5.2.2. Moving from left to right:
        for i in 0..row.len()-1 {
            // 5.2.1. Add a bottom wall to every cell
            row[i].bw = true;
            // If the current cell and the cell to the right are members of a different set:
            if row[i].set != row[i+1].set {
                // - Remove the right wall
                row[i].rw = false;
                // - Union the sets to which the current cell and cell to the right are members.
                let mut set2 = sets.remove(&row[i+1].set).expect("Can't get set for union");
                let mut set1 = sets.get_mut(&row[i].set).expect("Can't get set to union with");
                for j in set2.iter() {
                    row[*j].set = row[i].set;
                }
                set1.append(&mut set2);
            }
        }
        row[width-1].bw = true;

        // - Output the final row
        maze.push(row);

        EllerMaze {
            maze: maze,
        }
    }
}

// Using the style of http://www.neocomputer.org/projects/eller.html
// Doubled the line count, I think it look better in terminal that way
impl fmt::Display for EllerMaze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut result = String::new();

        for _ in 0..self.maze[0].len() {
            result.push_str(" ___");
        }
        result.push('\n');

        for row in &self.maze {
            let mut string1 = String::new();
            let mut string2 = String::new();
            string1.push('|');
            string2.push('|');
            for cell in row {
                if cell.bw {
                    string1.push_str("   ");
                    string2.push_str("___");
                } else {
                    string1.push_str("   ");
                    string2.push_str("   ");
                }
                if cell.rw {
                    string1.push_str("|");
                    string2.push_str("|");
                } else {
                    string1.push_str(" ");
                    string2.push_str(" ");
                }
            }
            result.push_str(&*string1);
            result.push('\n');
            result.push_str(&*string2);
            result.push('\n');
        }

        write!(f, "{}", result)
    }
}


