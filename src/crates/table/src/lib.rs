
use color_eyre::eyre;
use ratatui::text::{Line, Span};
use ratatui::style::{Color, Style};

#[derive(Debug, Default)]
pub struct Table {
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
    pub fn new(size_x: usize, size_y: usize) -> Self {
        Self { cells: vec![vec![0; size_y]; size_x] }
    }

    pub fn draw_table(&self) {
        println!("{}", self);
    }

    pub fn set_points(&mut self, ptrx: usize, ptry: usize) -> eyre::Result<&mut Self> {
        if ptrx > self.cells.len() || ptry > self.cells[0].len() {
            return Err(eyre::eyre!("Index out of range!"));
        }

        self.cells[ptrx-1][ptry-1] = 1;
        Ok(self)
    }

    pub fn into_lines(&self) -> Vec<Line> {
        self.cells
            .iter()
            .map(|row| {
                let spans = row
                    .iter()
                    .map(|&cell| {
                        // Преобразование числа в символ
                        let text = if cell == 1 { "1" } else { "0" };
                        // Создание стиля с цветом
                        Span::styled(text, Style::default().fg(Color::Yellow))
                    })
                    .collect::<Vec<Span>>();

                // Создание строки Line из Span
                Line::from(spans)
            })
            .collect()
    }


    pub fn find_next_gen(&mut self) -> &mut Self {
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
