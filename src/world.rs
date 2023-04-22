use rand::Rng;

pub struct World {
    width: usize,
    height: usize,
    pub cells: Vec<bool>,
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        let cells = vec![false; width * height];
        World {
            width,
            height,
            cells,
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for cell in self.cells.iter_mut() {
            *cell = rng.gen::<bool>();
        }
    }

    pub fn update(&mut self) {
        let mut new_cells = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index(x, y);
                let cell = self.cells[index];
                let live_neighbors = self.live_neighbor_count(x, y);

                new_cells[index] = match (cell, live_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn live_neighbor_count(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in &[self.height - 1, 0, 1] {
            for dx in &[self.width - 1, 0, 1] {
                if *dx == 0 && *dy == 0 {
                    continue;
                }
                let nx = (x + dx) % self.width;
                let ny = (y + dy) % self.height;

                if self.cells[self.get_index(nx, ny)] {
                    count += 1;
                }
            }
        }
        count
    }
}
