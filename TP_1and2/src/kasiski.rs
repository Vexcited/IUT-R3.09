use std::{collections::HashMap, cmp::Reverse};
use crate::{input::normalize, maths::pgcd};

const FRAGMENT_LEN: usize = 4;

fn find_repeated_fragments(ciphertext: &str) -> HashMap<String, Vec<usize>> {
  let mut fragments: HashMap<String, Vec<usize>> = HashMap::new();
  let length = ciphertext.len();

  for fragment_length in FRAGMENT_LEN..=length / 2 {
    for i in 0..=length - fragment_length {
      let fragment = &ciphertext[i..i + fragment_length];
      fragments.entry(fragment.to_string())
        .or_default()
        .push(i);
    }
  }

  fragments.retain(|_, positions| positions.len() > 1);
  fragments
}

/// Algorithme de Kasiski.
pub fn kasiski_analysis(ciphertext: &str) -> Vec<usize> {
  // 0. Normaliser le texte chiffré, pour ne garder que les lettres alphabétiques.
  let ciphertext = normalize(ciphertext);
    
  // 1. En examinant le texte chiffré donné en entrée, trouver des fragments de texte qui se répètent.
  let repetitions = find_repeated_fragments(&ciphertext);

  // 2. Calculer, pour chaque fragment, la distance entre les deux (ou plusieurs) occurrences.
  let mut repet: Vec<(String, Vec<usize>)> = Vec::new();
  for (fragment, positions) in &repetitions {
    let mut fragment_distances: Vec<usize> = Vec::new();
    for i in 0..positions.len() {
      for j in i + 1..positions.len() {
        fragment_distances.push(positions[j] - positions[i]);
      }
    }
    repet.push((fragment.clone(), fragment_distances));
  }

  // 3. Trier le tableau repet en ordre inverse de la taille du texte qui se répète.
  repet.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

  // 4. Si le tableau repet est vide, retourner le symbole d'erreur.
  if repet.is_empty() {
      return vec![];
  }

  // 5. Si le tableau repet n’est pas vide, formuler l’hypothèse que la clé doit être un diviseur de la distance entre les occurrences.
  let mut candidates: Vec<usize> = Vec::new();
  if let Some((_, fragment_distances)) = repet.first() {
    for &distance in fragment_distances {
      for i in 1..=distance {
        if distance % i == 0 {
          candidates.push(i);
        }
      }
    }
  }

  // 6. Traiter le tableau ligne par ligne.
  for (_, fragment_distances) in &repet {
    let mut temp: Vec<usize> = Vec::new();
    for &distance in fragment_distances {
      for &candidate in &candidates {
          let gcd = pgcd(candidate, distance);
          if gcd > 1 {
              temp.push(gcd);
          }
      }
    }

    if !temp.is_empty() {
        candidates = temp;
    } else if repet.last().unwrap().1 != *fragment_distances {
        continue;
    } else {
        break;
    }
  }

  // 7. Retourner le tableau des candidats sans doublons.
  candidates.sort_by_key(|&x| Reverse(x));
  candidates.dedup();
  candidates
}
