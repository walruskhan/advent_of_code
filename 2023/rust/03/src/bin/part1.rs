use std::fs::FileType;
use std::str::Chars;

fn main() {
    // iterate each value
    // if it's a number, check for number to it's right,
        // if number found, append to current
        // check neighbours for symbol
            // if symbol found, set flag to save number on complete
}

#[derive(Debug)]
struct Input<'a> {
    input: Vec<&'a str>,
    col: usize,
    row: usize,
    width: usize,
    height: usize,
    part_nums: Vec<i64>,
    total: i64,
}

impl Input<'_> {
    pub fn new(input: &str) -> Input {
        let rows: Vec<_> = input.split("\n").collect();
        let width = if let Some(r) = rows.first() { r.len() } else { 0 };
        let height = rows.len();
        
        Input {
            input: rows,
            col: 0,
            row: 0,
            width,
            height,
            part_nums: Vec::new(),
            total: 0,
        }
    }
    
    pub fn is_partnum(&self, row: usize, col: usize) -> bool {
        for r in 0..=2 {
            for c in 0..=2 {
                // Skip current tile (middle)
                if r == 1 && c == 1 {
                    continue;
                }
                
                if r == 0 && row == 0 {
                    continue;
                }
                
                if (c == 0 && col == 0) {
                    continue;
                }
                
                let some_row = self.input.get(row +r -1);
                if some_row.is_none() {
                    return false;
                }
                
                let some_c = some_row.unwrap()
                    .chars().nth(col +c -1);
                if some_c.is_none() {
                    return false;
                }
                
                let char = some_c.unwrap();
                if char != '.' && !char.is_digit(10) {
                    return true;
                }
                    
            }
        }
        
        false
    }
    
    pub fn process(&mut self) {
        let mut is_valid_part = false;
        let mut part_total: i64 = 0;
        
        loop {
            let c = self.input.get(self.row).unwrap()
                .chars().nth(self.col)
                .unwrap();
            
            if c.is_digit(10) {
                is_valid_part = is_valid_part || self.is_partnum(self.row, self.col);
                part_total = part_total * 10 + (c.to_digit(10).unwrap() as i64);
            } else { 
                if is_valid_part {
                    self.part_nums.push(part_total);
                    self.total += part_total;
                }
                
                part_total = 0;
                is_valid_part = false;
            }
            
            self.col += 1;
            if self.col >= self.width {
                self.col = 0;
                self.row += 1;
            }
            
            if self.row >= self.height {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use log::debug;
    use super::*;

    #[test]
    #[no_mangle]
    fn example() {
        let input = concat!(
            "467..114..\n",
            "...*......\n",
            "..35..633.\n",
            "......#...\n",
            "617*......\n",
            ".....+.58.\n",
            "..592.....\n",
            "......755.\n",
            "...$.*....\n",
            ".664.598..");
        
        let mut puzzle = Input::new(input);
        puzzle.process();
        
        println!("{:?}", puzzle);

        itertools::assert_equal(puzzle.part_nums.iter(), [467, 35, 633, 617, 592, 755, 664, 598].iter());
        assert_eq!(puzzle.total, 4361);
    }
}
