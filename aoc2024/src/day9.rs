pub fn solve(input: &str) {
    let part_1_total = solve_part_1(input);
    println!("Part 1 Result: {part_1_total}");

    let part_2_total = solve_part_2(input);
    println!("Part 2 Result: {part_2_total}");
}

//
fn solve_part_1(input: &str) -> usize {
    let mut disk = Disk::from_str(input);
    //println!("Disk created : {:?}", disk.blocks);
    disk.reorganize_blocks();
    //println!("Updated disk: {:?}", disk.blocks);
    disk.calculate_block_checksum()
}

fn solve_part_2(input: &str) -> usize {
    let mut disk = Disk::from_str(input);
    //println!("Disk created : {:?}", disk.blocks);
    disk.reorganize_blocks_filewise();
    //println!("Updated disk: {:?}", disk.blocks);
    disk.calculate_block_checksum()
}

#[derive(Debug, Clone)]
struct Disk {
    map: Vec<u32>,
    blocks: Blocks,
}

impl Disk {
    pub fn from_str(input: &str) -> Self {
        let map = input
            .chars()
            .map(|c| c.to_digit(10))
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect();
        let blocks = Blocks::from_map(&map);

        Disk { map, blocks }
    }
    pub fn reorganize_blocks(&mut self) {
        self.blocks.reorganize();
        assert!(!self.blocks.blocks.contains(&None));
    }
    pub fn reorganize_blocks_filewise(&mut self) {
        self.blocks.reorganize_filewise();
    }
    pub fn calculate_block_checksum(&self) -> usize {
        self.blocks.calculate_checksum()
    }
}

#[derive(Debug, Clone)]
struct Blocks {
    blocks: Vec<Option<u32>>,
}

impl Blocks {
    pub fn from_map(map: &Vec<u32>) -> Self {
        let mut blocks: Vec<Option<u32>> = Vec::new();

        for (i, value) in map.iter().enumerate() {
            if i % 2 == 0 {
                for _ in 0..*value {
                    blocks.push(Some(i as u32 / 2));
                }
            } else {
                for _ in 0..*value {
                    blocks.push(None);
                }
            }
        }

        Self { blocks }
    }
    pub fn reorganize(&mut self) {
        while self.blocks.contains(&None) {
            let index = self.blocks.iter().position(|e| e.is_none()).unwrap();
            self.blocks.swap_remove(index);
        }
    }

    pub fn reorganize_filewise(&mut self) {
        // println!("Current state: {}", self);
        let current_file_id = self.blocks.iter().rev().find(|&f| f.is_some());
        assert!(current_file_id.is_some());
        let mut current_file_id = current_file_id.unwrap().unwrap();

        while current_file_id > 0 {
            let index = self
                .blocks
                .iter()
                .position(|&f| f.is_some() && f.unwrap() == current_file_id);
            assert!(index.is_some());
            let index = index.unwrap();
            let size = self.get_object_size(index);

            let destination = self.find_empty_spot_of_size(size, index);

            if destination.is_some() {
                // println!( "Moving {} of size {} to index {} ", index, size, destination.unwrap());
                self.move_file_to_destination(index, size, destination.unwrap());
            }

            // println!("New state: {}", self);
            current_file_id -= 1;
        }
    }

    pub fn get_object_size(&self, index: usize) -> usize {
        let value = self.blocks[index];

        // Find the size of that block
        let mut size = 0;
        let mut cur_index = index;
        while cur_index < self.blocks.len() && self.blocks[cur_index] == value {
            size += 1;
            cur_index += 1;
        }

        size
    }

    pub fn find_empty_spot_of_size(&self, size: usize, max_index: usize) -> Option<usize> {
        self.blocks
            .iter()
            .enumerate()
            .position(|(i, v)| i < max_index && v.is_none() && self.get_object_size(i) >= size)
    }

    pub fn move_file_to_destination(&mut self, index: usize, size: usize, destination: usize) {
        for i in 0..size {
            self.blocks[destination + i] = self.blocks[index + i];
            self.blocks[index + i] = None;
        }
    }

    pub fn calculate_checksum(&self) -> usize {
        let mut checksum = 0;
        self.blocks.iter().enumerate().for_each(|(i, v)| {
            if v.is_some() {
                checksum += i * v.unwrap() as usize;
            }
        });
        checksum
    }
}

impl std::fmt::Display for Blocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut representation = String::new();
        for v in &self.blocks {
            match v {
                Some(v) => representation.push(char::from_digit(*v, 10).unwrap()),
                None => representation.push('.'),
            }
        }

        write!(f, " {}", representation)?;
        Ok(())
    }
}
