pub fn part01(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();

    let nav_lines = file.map(|x| x.chars());

    let mut syntax_error_score = 0;
    for nav_line in nav_lines {
        let mut unclosed_chunks: Vec<char> = Vec::new();
        for chunk_char in nav_line {
            if is_chunk_open_char(chunk_char) {
                unclosed_chunks.push(chunk_char);
            } else {
                let chunk_open_char = unclosed_chunks.pop().unwrap();
                if !is_chunk_close_char(chunk_char, chunk_open_char) {
                    syntax_error_score += lookup_points_syntax(chunk_char);
                    break;
                }
            }
        }
    }

    syntax_error_score
}

pub fn part02(file_path: &str) -> u64 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let file = file.lines();

    let nav_lines = file.map(|x| x.chars());

    let mut scores: Vec<u64> = Vec::new();
    'nav: for nav_line in nav_lines {
        let mut unclosed_chunks: Vec<char> = Vec::new();
        for chunk_char in nav_line {
            if is_chunk_open_char(chunk_char) {
                unclosed_chunks.push(chunk_char);
            } else {
                let chunk_open_char = unclosed_chunks.pop().unwrap();
                if !is_chunk_close_char(chunk_char, chunk_open_char) {
                    continue 'nav;
                }
            }
        }

        let mut score: u64 = 0;
        unclosed_chunks.iter().rev().for_each(|c| {
            let chunk_close_char = get_chunk_close_char(*c);
            score *= 5;
            score += lookup_points_autocomplet(chunk_close_char);
        });
        scores.push(score);
    }

    scores.sort_unstable();
    let middle = scores.len() / 2;

    scores[middle]
}

fn is_chunk_open_char(chunk_character: char) -> bool {
    match chunk_character {
        '(' | '[' | '{' | '<' => return true,
        _ => return false,
    }
}

fn is_chunk_close_char(chunk_character: char, chunk_open_char: char) -> bool {
    match (chunk_open_char, chunk_character) {
        ('(', ')') => return true,
        ('[', ']') => return true,
        ('{', '}') => return true,
        ('<', '>') => return true,
        _ => return false,
    }
}

fn get_chunk_close_char(chunk_open_char: char) -> char {
    match chunk_open_char {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => ' ',
    }
}

fn lookup_points_syntax(illegal_character: char) -> u32 {
    match illegal_character {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn lookup_points_autocomplet(illegal_character: char) -> u64 {
    match illegal_character {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}
