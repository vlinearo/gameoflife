use color_eyre::eyre;
use std::thread;
use std::time::Duration;


#[derive(Debug, Default)]
struct Table {
    cells: Vec<Vec<u8>>
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.cells {
            for el in line {
                write!(f, "{} ", el)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Table {
    fn new(size_x: usize, size_y: usize) -> Self {
        Self { cells: vec![vec![0; size_y]; size_x] }
    }

    fn draw_table(&self) {
        println!("{}", self);
    }

    fn set_points(&mut self, ptrx: usize, ptry: usize) -> eyre::Result<&mut Self> {
        if ptrx > self.cells.len() || ptry > self.cells[0].len() {
            return Err(eyre::eyre!("Index out of range!"));
        }

        self.cells[ptrx-1][ptry-1] = 1;
        Ok(self)
    }

    fn find_next_gen(&mut self) -> &mut Self {
        let positions: [[i8; 2]; 8] = [
            [0, 1], [1, 0], [0, -1], [-1, 0],
            [1, -1], [-1, 1], [1, 1], [-1, -1],
        ];

        let mut next_gen = vec![vec![0; self.cells[0].len()]; self.cells.len()];

        for (i, row) in self.cells.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let mut alive: u8 = 0;

                for pos in positions {
                    let (nx, ny) = (i as i8 + pos[0], j as i8 + pos[1]);
                    if nx >= 0 && nx < self.cells.len() as i8 && ny >= 0 && ny < self.cells[0].len() as i8 {
                        if self.cells[nx as usize][ny as usize] == 1 {
                            alive += 1;
                        }
                    }
                }

                next_gen[i][j] = match (cell, alive) {
                    (1, 2) | (1, 3) => 1,
                    (0, 3) => 1,
                    _ => 0,
                };
            }
        }

        self.cells = next_gen;

        self
    }
}

fn main() {
    let mut new_table = Table::new(10, 10);
    new_table.set_points(5, 5).unwrap()
        .set_points(5, 4).unwrap()
        .set_points(4, 6).unwrap()
        .set_points(6, 6).unwrap();

    new_table.draw_table();
    loop {
        new_table.find_next_gen().draw_table();
        thread::sleep(Duration::from_millis(1_000));
    }
}
