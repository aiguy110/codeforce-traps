use std::io::BufRead;

#[derive(Debug, PartialEq)]
struct TrapsPuzzle {
    base_dmgs: Vec<usize>,
    k: usize 
}

impl TrapsPuzzle {
    fn dmg_from_skip_inds(&self, skip_inds: &Vec<usize>) -> usize {
        let mut total_dmg = 0;
        let mut bonus_dmg = 0;
        for i in 0..self.base_dmgs.len() {
            if skip_inds.iter().any(|&si| si == i) {
                bonus_dmg += 1;   
            } else {
                total_dmg += self.base_dmgs[i] + bonus_dmg;
            }
        }

        total_dmg
    }
}

fn parse_traps_puzzle<R>(input: &mut R) -> TrapsPuzzle
    where R: BufRead
{
    let mut buf = String::new();
    input.read_line(&mut buf).unwrap();

    // Find out how many traps and jumps we have
    let line_1_nums: Vec<usize> = buf.split(' ')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    
    let mut puzzle = TrapsPuzzle {
        base_dmgs: Vec::with_capacity( line_1_nums[0] ),
        k: line_1_nums[1],
    };

    // Populate trap damages 
    buf.clear();
    input.read_line(&mut buf).unwrap();
    for s in buf.split(' ') {
        puzzle.base_dmgs.push( s.trim().parse().unwrap() );
    }

    puzzle
}

fn parse_traps_puzzles<R>(input: &mut R) -> Vec<TrapsPuzzle>
    where R: BufRead
{
    let mut buf = String::new();
    input.read_line(&mut buf).unwrap();
    let puzzle_count = buf.trim().parse().unwrap();

    let mut puzzles = Vec::<TrapsPuzzle>::with_capacity(puzzle_count);

    for _ in 0..puzzle_count {
        puzzles.push( parse_traps_puzzle(input) );
    }

    puzzles
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use itertools::Itertools;

    fn brute_force_solve(puzzle: &TrapsPuzzle) -> usize {
        (0..puzzle.base_dmgs.len()).combinations(puzzle.k)
            .map(|skip_inds| puzzle.dmg_from_skip_inds(&skip_inds))
            .min()
            .unwrap()
    }

    #[test]
    fn parse_single_puzzle() {
        let mut input = Cursor::new("3 2\n1 2 3\n");
        let puzzle = parse_traps_puzzle(&mut input);
        assert_eq!(puzzle, TrapsPuzzle {
            base_dmgs: vec![1,2,3],
            k: 2
        });
    }

    #[test]
    fn parse_multiple_puzzles() {
        let mut input = Cursor::new("2\n3 2\n1 2 3\n1 1\n42");
        let puzzles = parse_traps_puzzles(&mut input); 
        assert_eq!(puzzles, vec![
            TrapsPuzzle {base_dmgs: vec![1,2,3], k: 2},
            TrapsPuzzle {base_dmgs: vec![42],    k: 1}
        ])
    }

    #[test]
    fn brute_force_solver_works() {
        let puzzle = TrapsPuzzle {
            base_dmgs: vec![8,2,5,15,11,2,8],
            k: 5
        };

        assert_eq!(brute_force_solve(&puzzle), 9);
    }
}
