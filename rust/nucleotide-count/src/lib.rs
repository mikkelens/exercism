use std::collections::HashMap;

use itertools::Itertools;

fn non_nucleotide(c: &char) -> bool {
    // Reject characters that are not A, C, G, or T
    match c {
        'A' | 'C' | 'G' | 'T' => false,
        _ => true,
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if non_nucleotide(&nucleotide) {
        // Sanitize search parameter
        Err(nucleotide)
    } else {
        if let Some(wrong) = dna.chars().find(non_nucleotide) {
            // Sanitize input field
            Err(wrong)
        } else {
            Ok(dna.chars().filter(|&c| c == nucleotide).count())
        }
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    if let Some(wrong) = dna.chars().find(non_nucleotide) {
        // Sanitize input field
        Err(wrong)
    } else {
        let mut map = dna.chars().counts();
        for c in ['A', 'C', 'G', 'T'] {
            map.entry(c).or_insert(0); // ensure filled with empty values
        }
        Ok(map)
    }
}
