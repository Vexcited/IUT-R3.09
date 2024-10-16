use tp_3_and_4::password::{generate_password, ALLOWED_CHARSET};

const MASTER_PASSWORD: &str = "MonMDP!123";
const TARGET_PASSWORD_LENGTH: usize = 3; // N = 3
const MASTER_PASSWORD_LENGTH: usize = 10;

fn main() {
  let mut current_password = vec![ALLOWED_CHARSET[0]; MASTER_PASSWORD_LENGTH];
  let mut attempts = 0;

  loop {
      let master = String::from_utf8_lossy(&current_password).to_string();

      let generated_unilim = generate_password(&master, "Unilim", TARGET_PASSWORD_LENGTH).unwrap();
      let generated_amazon = generate_password(&master, "Amazon", TARGET_PASSWORD_LENGTH).unwrap();
      let generated_netflix = generate_password(&master, "Netflix", TARGET_PASSWORD_LENGTH).unwrap();
      
      let correct_unilim = generate_password(MASTER_PASSWORD, "Unilim", TARGET_PASSWORD_LENGTH).unwrap();
      let correct_amazon = generate_password(MASTER_PASSWORD, "Amazon", TARGET_PASSWORD_LENGTH).unwrap();
      let correct_netflix = generate_password(MASTER_PASSWORD, "Netflix", TARGET_PASSWORD_LENGTH).unwrap();
      
      attempts += 1;

      // Premier temps.
      if generated_unilim == correct_unilim {
          println!("Collision trouvée après {} tentatives, {master} -> {generated_unilim} = {correct_unilim}", attempts);
          return;
      }
      
      // Deuxième temps.
      // if generated_unilim == correct_unilim && generated_amazon == correct_amazon && generated_netflix == correct_netflix {
      //     println!("Collision trouvée après {} tentatives, {master}", attempts);
      //     println!("-> {generated_unilim} = {correct_unilim}");
      //     println!("-> {generated_amazon} = {correct_amazon}");
      //     println!("-> {generated_netflix} = {correct_netflix}");
      //     return;
      // }

      // Incrémenter le mot de passe
      for i in (0..MASTER_PASSWORD_LENGTH).rev() {
          let index = ALLOWED_CHARSET.iter().position(|&c| c == current_password[i]).unwrap();
          if index < ALLOWED_CHARSET.len() - 1 {
              current_password[i] = ALLOWED_CHARSET[index + 1];
              break;
          } else {
              current_password[i] = ALLOWED_CHARSET[0];
              if i == 0 {
                  println!("Toutes les combinaisons ont été essayées sans succès après {} tentatives.", attempts);
                  return;
              }
          }
      }
  }
}