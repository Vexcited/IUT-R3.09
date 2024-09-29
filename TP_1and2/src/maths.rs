use std::mem::swap;

/// Calculer le PGCD de deux nombres.
pub fn pgcd(mut a: usize, mut b: usize) -> usize {
  while b != 0 {
    if b < a {
      // On échange les valeurs de a et b.
      swap(&mut b, &mut a);
    }

    // On calcule le reste de la division de a par b.
    b %= a;
  }

  a
}
