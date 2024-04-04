pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = vec![];
        if row_count == 0 {
            return Self(rows);
        }
        for row_size in 1..=row_count {
            eprintln!("row size is {}", row_size);
            rows.push({
                let new_row = row_size
                    .checked_sub(1)
                    .map(|above| {
                        (0..row_size)
                            .map(|x| {
                                eprintln!("x is {}", x);
                                rows.get(above as usize)
                                    .map(|above_row| {
                                        x.checked_sub(1)
                                            .map(|first| {
                                                eprintln!("adding two together.");
                                                above_row.get(first as usize).unwrap_or(&0)
                                                    + above_row.get(x as usize).unwrap_or(&0)
                                            })
                                            .unwrap_or(0)
                                    })
                                    .unwrap_or(1)
                            })
                            .collect()
                    })
                    .unwrap_or(vec![1]);
                eprintln!("new row: {:?}", new_row);
                new_row
            });
        }
        Self(rows)
    }

    pub fn rows(self) -> Vec<Vec<u32>> {
        self.0
    }
}
