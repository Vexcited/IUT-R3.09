pub struct Vigenere {
  key: String
}

impl Vigenere {
  pub fn new (key: &str) -> Vigenere {
    Vigenere { key: key.into() }
  }

  pub fn encipher (&self, plain_text: &str) -> String {
    // on supprime tous les caractères unicode et non-ASCII de la clé
    let key: String = self.key.chars().filter(|&c| {
      // on garde que les caractères alphabétiques
      // donc allant de A à Z et a à z
      c.is_ascii_alphabetic()
    }).collect();

    // on transforme la clé en minuscule
    let key = key.to_ascii_lowercase();

    let key_length = key.len();
    if key_length == 0 {
      return plain_text.into();
    }

    let mut index = 0;

    // on chiffre chaque caractère du texte
    plain_text.chars()
      .map(|c| {
          if c.is_ascii_alphabetic() {
            // si la lettre est minuscule, on commence par 'a'
            // sinon 'A' (pour les opérations en ascii)
            let first_char = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let shift = key.as_bytes()[index % key_length] - b'a';
            index += 1;

            // modulo la distance pour garder le caractère dans la plage (de l'alphabet)
            (first_char + (c as u8 + shift - first_char) % 26) as char
          }
          // si c'est pas alphabétique, on renvoie le caractère tel quel
          else {
            c
          }
      })
      .collect()
  }

  pub fn decipher (&self, cipher_text: &str) -> String {
    // on supprime tous les caractères unicode et non-ASCII de la clé
    let key: String = self.key.chars().filter(|&c| {
      // on garde que les caractères alphabétiques
      // donc allant de A à Z et a à z
      c.is_ascii_alphabetic()
    }).collect();

    // on transforme la clé en minuscule
    let key = key.to_ascii_lowercase();

    let key_length = key.len();
    if key_length == 0 {
      return cipher_text.into();
    }

    let mut index = 0;

    // on déchiffre chaque caractère du texte
    cipher_text.chars()
      .map(|c| {
          if c.is_ascii_alphabetic() {
            // si la lettre est minuscule, on commence par 'a'
            // sinon 'A' (pour les opérations en ascii)
            let first_char = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let shift = key.as_bytes()[index % key_length] - b'a';
            index += 1;

            // modulo la distance pour garder le caractère dans la plage (de l'alphabet)
            (first_char + (c as u8 + 26 - shift - first_char) % 26) as char
          }
          // si c'est pas alphabétique, on renvoie le caractère tel quel
          else {
            c
          }
      })
      .collect()
  }
}
