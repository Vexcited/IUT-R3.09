# Rapport: TP 1 et 2

## Implémentation Vigenère

Concernant la clé : les espaces, caractères spéciaux et chiffres sont supprimés et la clé est convertie en minuscules.

Concernant le texte à chiffrer : les espaces, caractères spéciaux et chiffres sont ignorés. Les lettres en majuscules sont chiffrées en majuscules et les lettres en minuscules sont chiffrées en minuscules.

Si la clé est vide, on renvoie le texte inchangé.

## Utilisation

On peut directement utiliser le programme avec `cargo run`.

```bash
# Vous pouvez chiffrer un texte directement.
echo "hello" | cargo run encrypt --key "world"

# Vous pouvez aussi utiliser un fichier en entrée (stdin)
cargo run encrypt --key "world" < input.txt

# Vous pouvez aussi utiliser un fichier en entrée (stdin) et en sortie (stdout)
cargo run encrypt --key "world" < input.txt > output.txt
# ou
cargo run encrypt --key "world" --output ./output.txt < input.txt
```

Si vous déchiffrer à la place de chiffrer, remplacez `encrypt` par `decrypt`.

Pour utiliser la méthode Kasiski, remplacez `encrypt` ou `decrypt` par `kasiski`.
La clé et l'output n'est pas nécessaire pour la méthode Kasiski.
La longueur de la clé est affichée à la fin de l'exécution.

```bash
# Vous pouvez utiliser un fichier en entrée (stdin)
cargo run kasiski < input.txt

# Vous pouvez aussi utiliser un texte en entrée
echo "hello" | cargo run kasiski
```

## Tests pour Kasiski

Le fichier `tests/20-output-world.txt` a été chiffré à partir de `tests/20-input.txt` avec la clé `WORLD`.
