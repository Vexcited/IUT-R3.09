use inquire::{error::InquireResult, prompt_confirmation, prompt_secret, prompt_text};

mod password;
use password::{check_allowed_charset, generate_password};

mod storage;

fn prompt_master_password() -> String {
  let master = prompt_secret("Mot de passe maître <caché> :")
  .expect("Le mot de passe maître est requis");

  if !check_allowed_charset(&master) {
    eprintln!("Le mot de passe maître contient des caractères non autorisés");
    std::process::exit(1);
  }

  storage::store_master_password(&master);
  master
}

fn main() {
  let mut master = storage::read_master_password();

  if master.is_empty() {
    master = prompt_master_password();
  }
  else {
    match prompt_confirmation("Utiliser le mot de passe maître stocké ?") {
      InquireResult::Ok(true) => (),
      InquireResult::Ok(false) => {
        master = prompt_master_password();
      },
      InquireResult::Err(err) => println!("Une erreur s'est produite: {}", err),
    }
  }

  let tag = prompt_text("Tag :")
    .expect("Le tag est requis");

  if !check_allowed_charset(&tag) {
    eprintln!("Le tag contient des caractères non autorisés");
    std::process::exit(1);
  }
  
  let length = prompt_text("Longueur du mot de passe :")
    .expect("La longueur est requise");
  let length: usize = length.parse()
    .expect("La longueur doit être un nombre");

  let password = generate_password(&master, &tag, length)
    .expect("Le mot de passe maître ou/et le tag contient des caractères non autorisés");

  println!("{password}");
}
