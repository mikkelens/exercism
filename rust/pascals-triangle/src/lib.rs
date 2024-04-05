pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = vec![];
        if row_count == 0 {
            return Self(rows);
        }
        for row_size in 1..=row_count {
            rows.push({
                rows.get(row_size.saturating_sub(2) as usize)
                    .map(|above_row| {
                        (0..row_size)
                            .map(|x| {
                                x.checked_sub(1)
                                    .map(|first| {
                                        above_row.get(first as usize).unwrap()
                                            + above_row.get(x as usize).unwrap_or(&0)
                                    })
                                    .unwrap_or(*above_row.get(x as usize).unwrap_or(&1))
                            })
                            .collect()
                    })
                    .unwrap_or(vec![1])
            });
        }
        Self(rows)
    }

    pub fn rows(self) -> Vec<Vec<u32>> {
        self.0
    }
}
