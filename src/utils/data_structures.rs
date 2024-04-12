#[derive(Debug)]
pub struct Index {
    pub limit: usize,
    pub index: usize,
}

impl Index {
    pub fn new(limit: usize, current: usize) -> Index {
        return Index {
            limit,
            index: Index::normalize(limit, current),
        };
    }

    fn normalize(limit: usize, current: usize) -> usize {
        return current % limit;
    }

    pub fn add(&mut self, change: i8) {
        if change > 0 {
            self.index += change as usize;
        } else {
            let mut to_sub = -change as usize;
            while to_sub >= self.limit {
                to_sub -= self.limit;
            }
            self.index = self
                .index
                .checked_sub(to_sub)
                .unwrap_or(self.limit - to_sub + self.index);
        }
        self.index = Index::normalize(self.limit, self.index);
    }
}

/*
TODO list:
 * create an index object in main
 * Add and subtract to it
 */
