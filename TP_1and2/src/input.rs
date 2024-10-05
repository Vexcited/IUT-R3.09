/// Convertir le texte donné pour être utilisable.
pub fn normalize(message: &str) -> String {
    // On supprime les espaces, les caractères spéciaux et les chiffres du message.
    let message: String = message
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .collect();

    // On convertit le texte en minuscule.
    message.to_ascii_lowercase()
}
