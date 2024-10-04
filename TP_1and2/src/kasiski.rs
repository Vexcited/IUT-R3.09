use gcd::Gcd;
use std::collections::HashMap;

pub fn kasiski_analysis(ciphertext: &str) -> Result<usize, String> {
    let mut fragment_len = 3;
    let mut last_candidate = 0;
    let bytes = ciphertext.as_bytes();

    while fragment_len < ciphertext.len() {
        // Équivalent du tableau "repet" : en clé on a le fragment, en valeur on a les positions où il se trouve.
        let mut fragment_positions = HashMap::new();
        let mut candidates = Vec::new();

        // On calcule les distances entre les fragments répétés.
        for (i, window) in bytes.windows(fragment_len).enumerate() {
            let positions: &mut Vec<usize> = fragment_positions.entry(window).or_default();

            if !positions.is_empty() {
                candidates.push(i - positions[positions.len() - 1]);
            }

            positions.push(i);
        }

        if !candidates.is_empty() {
            candidates.sort_unstable();
            candidates.dedup();

            let current_candidate = candidates.iter().fold(candidates[0], |acc, &x| acc.gcd(x));

            if current_candidate == last_candidate && current_candidate != 1 {
                return Ok(current_candidate);
            }

            last_candidate = current_candidate;
        }

        fragment_len += 1;
    }

    Err("No key length found.".to_string())
}
