use std::{collections::HashMap, cmp::Reverse};
use crate::{input::normalize, maths::pgcd};

/// Trouver les sous-chaînes répétées d'une longueur minimale.
fn find_repeated_fragments(cipher_text: &str, min_length: usize) -> HashMap<String, Vec<usize>> {
  let mut repetitions: HashMap<String, Vec<usize>> = HashMap::new();

  // On parcourt le texte en cherchant des sous-chaînes répétées.
  for i in 0..cipher_text.len() - min_length + 1 {
    let fragment = &cipher_text[i..(i + min_length)];

    for ii in (i + min_length)..(cipher_text.len() - min_length + 1) {
      if &cipher_text[ii..(ii + min_length)] == fragment {
        repetitions.entry(fragment.to_string())
          .or_default()
          .push(ii - i);
      }
    }
  }

  repetitions
}

/// On filtre les candidats qui peut rester en utilisant le PGCD.
fn filter_candidates_by_pgcd(candidates: &[usize], distance: usize) -> Vec<usize> {
  let mut temp: Vec<usize> = candidates.iter()
      .map(|&candidate| pgcd(candidate, distance))
      // On élimine les PGCD égaux à 1.
      .filter(|&g| g > 1)
      .collect();

  // S'il y a des candidats restants...
  if !temp.is_empty() {
    temp.sort();

    // On supprime les doublons.
    temp.dedup();
  }

  temp
}

/// Algorithme de Kasiski.
pub fn kasiski_analysis(ciphertext: &str, min_fragment_length: usize) -> Vec<usize> {
  let ciphertext = normalize(ciphertext);
  let repetitions = find_repeated_fragments(&ciphertext, min_fragment_length);

  // On retourne '?' si aucune répétition n'est trouvée.
  if repetitions.is_empty() {
    return vec!['?' as usize];
  }

  let mut candidates: Vec<usize> = Vec::new();

  let mut repetitions_vec: Vec<_> = repetitions.values().collect();
  // On trie les répétitions par longueur de fragment.
  repetitions_vec.sort_by_key(|b| Reverse(b.len()));

  for distances in repetitions_vec {
    if let Some(&first_distance) = distances.first() {
      // Initialiser les candidats avec les diviseurs de la première distance
      if candidates.is_empty() {
        candidates = (2..=first_distance).filter(|d| first_distance % d == 0).collect();
      }
      else {
        // Affiner les candidats en utilisant le PGCD avec les autres distances
        for &distance in distances {
          let temp_candidates = filter_candidates_by_pgcd(&candidates, distance);
          if !temp_candidates.is_empty() {
            candidates = temp_candidates;
          }
        }
      }
    }
  }

  if candidates.is_empty() {
    vec!['?' as usize]
  }
  else {
    candidates
  }
}
