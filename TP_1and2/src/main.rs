use inquire::Text;

mod vigenere;
use vigenere::Vigenere;

fn main() {
  let key = Text::new("Entrez la clé de chiffrement: ").prompt().unwrap();
  let vigenere = Vigenere::new(&key);

  let plain_text = Text::new("Entrez le texte que vous voulez chiffrer: ").prompt().unwrap();
  let cipher_text = vigenere.encipher(&plain_text);
  println!("Texte chiffré: {}", cipher_text);

  let cipher_text = Text::new("Entrez le texte que vous voulez déchiffrer: ").prompt().unwrap();
  let plain_text = vigenere.decipher(&cipher_text);
  println!("Texte déchiffré: {}", plain_text);
}

