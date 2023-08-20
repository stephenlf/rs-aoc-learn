impl super::Forest {
    pub(super) fn calc_visibility(&mut self) {
        let (rows, columns) = (self.0.len(), self.0[0].len());
        let mut max_height: i32;

        // Left to right
        for i in 0..rows {
            max_height = -1;
            for j in 0..columns {
                let tree_height = self.0[i][j].height;
                if tree_height > max_height {
                    max_height = tree_height;
                    self.0[i][j].visibility = true;
                }
            }
        }

        // Top to bottom
        for j in 0..columns {
            max_height = -1;
            for i in 0..rows {
                let tree_height = self.0[i][j].height;
                if tree_height > max_height {
                    max_height = tree_height;
                    self.0[i][j].visibility = true;
                }
            }
        }

        // right to left
        for i in 0..rows {
            max_height = -1;
            for j in (0..columns).rev() {
                let tree_height = self.0[i][j].height;
                if tree_height > max_height {
                    max_height = tree_height;
                    self.0[i][j].visibility = true;
                }
            }
        }

        // bottom to top
        for j in 0..columns {
            max_height = -1;
            for i in (0..rows).rev() {
                let tree_height = self.0[i][j].height;
                if tree_height > max_height {
                    max_height = tree_height;
                    self.0[i][j].visibility = true;
                }
            }
        }
    }

    pub(super) fn sum_visibility(&self) -> u32 {
        self.0.iter().flatten().fold(0, |accum, tree| {
            if tree.visibility {
                accum + 1
            } else {
                accum
            }
        })
    }
}