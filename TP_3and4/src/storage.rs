use crate::password::check_allowed_charset;
use std::fs;

const FILE_NAME: &str = "mpwd.txt";

pub fn read_master_password () -> String {
  // On vérifie que le fichier existe bien.
  if fs::metadata(FILE_NAME).is_ok() {
    let master = fs::read_to_string(FILE_NAME)
      .expect("Impossible de lire le fichier de stockage du mot de passe maître");

    // Si le mot de passe est autorisé, on le retourne.
    if check_allowed_charset(&master) {
      return master;
    }
  }

  // Sinon, on retourne une chaîne vide.
  "".into()
}

pub fn store_master_password (master: &str) {
  fs::write(FILE_NAME, master)
    .expect("Impossible d'écrire le fichier de stockage du mot de passe maître")
}
