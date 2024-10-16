use sha2::{Digest, Sha256};

/// Fonction qui génère un hash SHA-256 à partir
/// d'un mot de passe maître et d'un tag qui
/// sont concaténés.
/// 
/// ```rust
/// // Obtenir le SHA-256 de "monmdpunilim" en vecteur d'octets.
/// make256("monmdp", "unilim");
/// ```
fn make256(master: &str, tag: &str) -> Result<Vec<u8>, String> {
  if !check_allowed_charset(master) || !check_allowed_charset(tag) {
    return Err("Le mot de passe maître ou le tag contient des caractères non autorisés".into());
  }

  let input = format!("{master}{tag}");
  let input = input.as_bytes();

  let mut hasher = Sha256::new();
  hasher.update(input);

  Ok(hasher.finalize().to_vec())
}

/// Vérifie si le texte ne contient que des caractères autorisés.
pub fn check_allowed_charset(text: &str) -> bool {
  text.chars().all(|c| ALLOWED_CHARSET.contains(&(c as u8)))
}

/// Les caractères que l'on autorise sont :
/// - lettres majuscules : A-Z
/// - lettres minuscules : a-z
/// - chiffres : 0-9
/// - les caractères spéciaux suivants : !?@#$%&*()[]|:;,.
/// 
/// Ainsi, nous n'autorisons pas :
/// - les espaces
/// - les caractères invisibles
/// 
/// et d'autres caractères spéciaux.
pub const ALLOWED_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!?@#$%&*()[]|:;,.";
const ALLOWED_CHARSET_LEN: usize = ALLOWED_CHARSET.len();

pub fn generate_password(master: &str, tag: &str, length: usize) -> Result<String, String> {
  // On utilise notre fonction pour générer un hash SHA-256
  // à partir de `master` et `tag`.
  let hash = make256(master, tag)?;

  // On initialise une chaîne de caractères avec une capacité de `length`.
  let mut output = String::with_capacity(length);

  // Tant qu'on a pas `length` caractères dans notre chaîne de sortie...
  while output.len() < length {
    let index = output.len();

    // On prend un caractère du hash que l'on convertit en nombre non signé.
    let value = hash[index % hash.len()] as usize;
    
    // On ajoute l'index pour avoir un nombre "unique".
    let value = value + index;

    // On ajoute le caractère correspondant à la valeur modulo la taille du charset.
    output.push(ALLOWED_CHARSET[value % ALLOWED_CHARSET_LEN] as char);
  }

  Ok(output)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_generates_8_characters() {
    let result = generate_password("MonMDP!", "unilim", 8);
    assert_eq!(result.unwrap().len(), 8);
  }

  #[test]
  fn it_generates_1024_characters() {
    let result = generate_password("MonMDP!", "unilim", 1024);
    assert_eq!(result.unwrap().len(), 1024);
  }

  #[test]
  fn it_should_not_generate_password_with_invalid_master() {
    let result = generate_password("MonMDP ", "unilim", 8);
    assert!(result.is_err());
  }

  #[test]
  fn it_should_not_generate_password_with_invalid_tag() {
    let result = generate_password("MonMDP!", "unilim ", 8);
    assert!(result.is_err());
  }
}