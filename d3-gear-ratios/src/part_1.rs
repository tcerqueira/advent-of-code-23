#![allow(dead_code)]


fn process(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    lines.iter()
        .enumerate()
        .map(|(idx_line, line)| {
            let mut sum = 0;
            for (idx_char, char) in line.chars().enumerate() {
                if char.is_alphanumeric() || char == '.' {
                    continue;
                }
                
                sum += process_symbol(&lines, (idx_line, idx_char));
            }
            sum
        })
        .sum()
        
}

fn process_symbol(lines: &Vec<&str>, (idx_line, idx_char): (usize, usize)) -> u32 {
    let mut lines_arr: [Option<&str>; 3] = [None, Some(lines[idx_line]), None];
    if idx_line != 0 {
        lines_arr[0] = Some(lines[idx_line-1]);
    }
    if idx_line != lines.len()-1 {
        lines_arr[2] = Some(lines[idx_line+1])
    }

    lines_arr.into_iter().
        flatten()
        .map(|line| {
            let mut numbers: Vec<&str> = vec![];
            let mut last_num_idx = 0;
            let mut is_number = false;
            for (idx, char) in line.chars().enumerate() {
                if !char.is_alphanumeric() {
                    last_num_idx = idx;
                    if is_number {
                        numbers.push(&line[last_num_idx+1..idx]);
                    }
                    is_number = false;
                }
                
                is_number = true;
            }
            if is_number {
                numbers.push(&line[last_num_idx+1..]);
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn example_test() {
        assert_eq!(process(EXAMPLE_INPUT), 4361)
    }
    
}
