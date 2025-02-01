use crate::input;

type Regions = Vec<Vec<(usize, usize)>>;

pub fn solve() -> (Option<usize>, Option<usize>) {
    let input = input::DAY_12_INPUT;
    let grid = Grid::from_str(input);
    let regions = grid.get_regions();
    let part_1_total = solve_part_1(&grid, &regions);
    let part_2_total = solve_part_2(&grid, &regions);
    (part_1_total, part_2_total)
}

fn solve_part_1(grid: &Grid, regions: &Regions) -> Option<usize> {
    let mut price = 0;
    for region in regions {
        // println!("Regions {}-{}", region[0].0, region[0].1);
        price += grid.get_region_price(region.as_slice());
    }

    Some(price)
}

fn solve_part_2(grid: &Grid, regions: &Regions) -> Option<usize> {
    let mut price = 0;
    for region in regions {
        // println!("Regions {}-{}", region[0].0, region[0].1);
        price += grid.get_new_region_price(region.as_slice());
    }

    Some(price)
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let data: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        Self { data }
    }

    fn at(&self, line: isize, pos: isize) -> Option<char> {
        if line < 0 || pos < 0 {
            return None;
        }
        let line = line as usize;
        let pos = pos as usize;
        self.data.get(line)?.get(pos).copied()
    }

    fn get_regions(&self) -> Regions {
        // Keep track of the tiles we used
        let mut used: Vec<Vec<bool>> = self
            .data
            .iter()
            .map(|l| l.iter().map(|_| false).collect())
            .collect();

        let mut regions: Regions = Vec::new();

        for (l, line) in self.data.iter().enumerate() {
            for (p, ch) in line.iter().enumerate() {
                if used[l][p] {
                    continue;
                }
                used[l][p] = true;

                let mut region = vec![(l, p)];
                self.find_region(*ch, &mut used, &mut region, (l as isize, p as isize));

                regions.push(region);
            }
        }
        regions
    }

    fn find_region(
        &self,
        c: char,
        used: &mut Vec<Vec<bool>>,
        region: &mut Vec<(usize, usize)>,
        current: (isize, isize),
    ) {
        let (l, p) = current;
        for (ld, pd) in [(l - 1, p), (l + 1, p), (l, p - 1), (l, p + 1)] {
            let neighbor = self.at(ld, pd);
            if neighbor.is_none() || neighbor.unwrap() != c {
                continue;
            }
            if used[ld as usize][pd as usize] {
                continue;
            }

            used[ld as usize][pd as usize] = true;
            region.push((ld as usize, pd as usize));
            self.find_region(c, used, region, (ld, pd));
        }
    }

    fn get_region_area(region: &[(usize, usize)]) -> usize {
        region.len()
    }

    fn get_region_perimeter(region: &[(usize, usize)]) -> usize {
        let mut perimeter = 0;

        for (l, p) in region {
            let mut individual_border = 4;
            if region.contains(&(*l + 1, *p)) {
                individual_border -= 1;
            }
            if region.contains(&(*l, *p + 1)) {
                individual_border -= 1;
            }
            if *l > 0 && region.contains(&(*l - 1, *p)) {
                individual_border -= 1;
            }
            if *p > 0 && region.contains(&(*l, *p - 1)) {
                individual_border -= 1;
            }

            perimeter += individual_border;
        }

        perimeter
    }

    fn get_region_sides(&self, region: &[(usize, usize)]) -> usize {
        // the number of sides is the same as the number of corners

        // Storing here coordinates of sides, together with a bool showing vertical/horizontal
        // (line, position, is_horizontal, from_before)
        let mut side_count: usize = 0;
        for (l, p) in region {
            // top left
            if !region.contains(&(*l, p.wrapping_sub(1)))
                && !region.contains(&(l.wrapping_sub(1), *p))
            {
                side_count += 1;
            } else if region.contains(&(*l, p.wrapping_sub(1)))
                && region.contains(&(l.wrapping_sub(1), *p))
                && !region.contains(&(l.wrapping_sub(1), p.wrapping_sub(1)))
            {
                side_count += 1;
            }
            // top right
            if !region.contains(&(*l, *p + 1)) && !region.contains(&(l.wrapping_sub(1), *p)) {
                side_count += 1;
            } else if region.contains(&(*l, *p + 1))
                && region.contains(&(l.wrapping_sub(1), *p))
                && !region.contains(&(l.wrapping_sub(1), *p + 1))
            {
                side_count += 1;
            }

            // bottom right
            if !region.contains(&(*l, p + 1)) && !region.contains(&(*l + 1, *p)) {
                side_count += 1;
            } else if region.contains(&(*l, p + 1))
                && region.contains(&(*l + 1, *p))
                && !region.contains(&(*l + 1, p + 1))
            {
                side_count += 1;
            }
            // bottom left
            if !region.contains(&(*l, p.wrapping_sub(1))) && !region.contains(&(*l + 1, *p)) {
                side_count += 1;
            } else if region.contains(&(*l, p.wrapping_sub(1)))
                && region.contains(&(*l + 1, *p))
                && !region.contains(&(*l + 1, p.wrapping_sub(1)))
            {
                side_count += 1;
            }
        }
        // dbg!(side_count);
        side_count
    }

    fn get_region_price(&self, region: &[(usize, usize)]) -> usize {
        Grid::get_region_perimeter(region) * Grid::get_region_area(region)
    }

    fn get_new_region_price(&self, region: &[(usize, usize)]) -> usize {
        self.get_region_sides(region) * Grid::get_region_area(region)
    }
}
