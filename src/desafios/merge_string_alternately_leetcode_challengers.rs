pub fn merge_alternately(word1: &str, word2: &str) -> Result<String, &'static str> {
    let n = word1.len();
    let m = word2.len();
    let mut result = String::new();

    for i in 0..std::cmp::min(n, m) {
        result.push_str(&word1[i..=i]);
        result.push_str(&word2[i..=i]);
    }

    if n > m {
        result.push_str(&word1[m..]);
    } else if n < m {
        result.push_str(&word2[n..]);
    }
    return Ok(result);
}
