use crate::input::normalize;
const ALPHABET_LEN: u8 = 26;

/// Chiffrer un message en utilisant le chiffre de Vigenère.
pub fn vigenere_encrypt (message: &str, key: &str) -> String {
  let key = normalize(key);

  let key_length = key.len();
  if key_length == 0 {
    return message.into();
  }

  let mut index = 0;

  // On chiffre chaque caractère du texte.
  message.chars()
    .map(|char| {
      if char.is_ascii_alphabetic() {
        // Si la lettre est minuscule, on commence par 'a' sinon 'A'.
        let alphabet_start = if char.is_ascii_lowercase() { b'a' } else { b'A' };
        let key_shift = key.as_bytes()[index % key_length] - b'a';
        let char_pos = char as u8;

        // On incrémente l'index dans la clé.
        index += 1;

        (alphabet_start + (char_pos + key_shift - alphabet_start) % ALPHABET_LEN) as char
      }
      // Si le caractère n'est pas alphabétique, on le renvoie tel quel.
      else {
        char
      }
    })
    .collect()
}

/// Déchiffrer un message en utilisant le chiffre de Vigenère.
pub fn vigenere_decrypt (ciphertext: &str, key: &str) -> String {
  let key = normalize(key);

  let key_length = key.len();
  if key_length == 0 {
    return ciphertext.into();
  }

  let mut index = 0;

  // On déchiffre chaque caractère du texte.
  ciphertext.chars()
    .map(|char| {
      // Si le caractère est alphabétique.
      if char.is_ascii_alphabetic() {
        // Si la lettre est minuscule, on commence par 'a' sinon 'A'.
        let alphabet_start = if char.is_ascii_lowercase() { b'a' } else { b'A' };
        let key_shift = key.as_bytes()[index % key_length] - b'a';
        let char_pos = char as u8;

        // On incrémente l'index dans la clé.
        index += 1;

        (alphabet_start + (char_pos + ALPHABET_LEN - key_shift - alphabet_start) % ALPHABET_LEN) as char
      }
      // Si le caractère n'est pas alphabétique, on le renvoie tel quel.
      else {
        char
      }
    })
    .collect()
}
