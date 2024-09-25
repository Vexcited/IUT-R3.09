use inquire::Text;

// implémentation du chiffrement de Vigenère.
// prend en entrée un texte clair et une clé.
// renvoie le texte chiffre avec la clé.
pub fn vigenere_cipher(plain_text: &str, key: &str) -> String {
    // on supprime tous les caractères unicode et non-ASCII de la clé
    let key: String = key.chars().filter(|&c| {
        // on garde que les caractères alphabétiques
        // donc allant de A à Z et a à z
        c.is_ascii_alphabetic()
    }).collect();

    // on transforme la clé en minuscule
    let key = key.to_ascii_lowercase();

    let key_length = key.len();
    if key_length == 0 {
        return String::from(plain_text);
    }

    let mut index = 0;

    // on chiffre chaque caractère du texte
    let cipher_text: String = plain_text.chars()
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
        .collect();

    cipher_text
}

fn main() {
    let plain_text = Text::new("Entrez le texte que vous voulez chiffrer: ").prompt().unwrap();
    let key = Text::new("Entrez la clé de chiffrement: ").prompt().unwrap();

    let cipher_text = vigenere_cipher(&plain_text, &key);
    println!("Texte chiffré: {}", cipher_text);
}

