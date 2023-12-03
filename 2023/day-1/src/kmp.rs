#[derive(Debug)]
pub struct Occurences {
    indexes: Vec<usize>,
}

impl Occurences {
    pub fn indexes(&self) -> &Vec<usize> {
        &self.indexes
    }
}

pub fn find_occurences(text: &str, pattern: &str, lps: &mut [usize]) -> Option<Occurences> {
    let n = text.len();
    let m = pattern.len();
    let pattern = pattern.chars().collect::<Vec<char>>();
    let text = text.chars().collect::<Vec<char>>();

    init_lps(&pattern, lps);
    pattern_match(&text, &pattern, &lps)
}

fn pattern_match(text: &Vec<char>, pattern: &Vec<char>, lps: &[usize]) -> Option<Occurences> {
    let n = text.len();
    let m = pattern.len();

    let mut j: usize = 0;
    let mut i: usize = 0;
    let mut occurences = 0;
    let mut index = vec![];
    while i < n {
        if text[i] == pattern[j] {
            if j == m - 1 {
                occurences += 1;
                index.push(i - (m - 1)); // 2 - (3 - 1)
                j = 0;
            } else {
                j += 1;
            }
            if i == n - 1 {
                break;
            }
            i += 1;
        } else if j > 0 {
            j = lps[j - 1];
        } else {
            i += 1;
        }
    }
    if occurences > 0 {
        return Some(Occurences { indexes: index });
    } else {
        None
    }
}

/// LPS: longest proper suffix.
fn init_lps(pattern: &Vec<char>, lps: &mut [usize]) {
    let m = pattern.len();
    if m == 1 {
        return;
    }

    let mut j: usize = 0;
    let mut i: usize = 1;

    while i < m {
        if pattern[i] == pattern[j] {
            lps[i] = j + 1;
            i += 1;
            j += 1;
        } else if j > 0 {
            j = lps[j - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }
}
