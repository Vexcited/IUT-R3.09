# Rapport: TP 1 et 2

## Vigenère

### Traitement du texte

J'utilise deux crates, `clap` pour la gestion des arguments et `clio` pour la gestion des fichiers dans les arguments.

Ainsi, pour recevoir la clé et le texte à chiffrer/déchiffrer, vous pouvez utiliser les commandes suivantes :

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

Si vous souhaitez déchiffrer à la place de chiffrer, remplacez la commande `encrypt` par `decrypt`.

### Détails sur l'implémentation

J'ai décidé de rester dans l'implémentation simple proposée par Wikipédia et dCode, où seulement l'alphabet (`A-Za-z`) est géré.

- La clé : les espaces, caractères spéciaux et chiffres sont supprimés et la clé est convertie en minuscules.
- Le texte à chiffrer : les espaces, caractères spéciaux et chiffres **sont ignorés**. Les lettres en majuscules sont chiffrées en majuscules et les lettres en minuscules sont chiffrées en minuscules.

Si la clé est vide, on renvoie le texte inchangé.

### Jeu d'essais

Prenons le texte suivant et essayons de le chiffrer avec la clé `VEXCITED`.

```plaintext
Those scrapers are nothing more than lettuces. One cannot separate bedrooms from presto overcoats. A patio of the clave is assumed to be a foetal rhinoceros. A poison sees a joke as a dormie school. This is not to discredit the idea that a difference can hardly be considered a clingy cornet without also being a fahrenheit. However, the galliard pajama comes from an unglad beef. The zeitgeist contends that the vorant purchase comes from a mimic unit. Far from the truth, a himalayan of the relish is assumed to be an extrorse pear. If this was somewhat unclear, a chime of the toenail is assumed to be a spangly move. The diaphragm of a desert becomes a festive kitten.

In ancient times they were lost without the unfair lyric that composed their surfboard. A salad can hardly be considered a gummous tortellini without also being a mail. A thrill is a tv from the right perspective. A china is an attention's precipitation. What we don't know for sure is whether or not some posit the tenser wall to be less than gearless. A fertilizer sees a governor as a boastful aquarius. Some silty pens are thought of simply as profits. Some softwood kettles are thought of simply as bows. However, authors often misinterpret the knot as an eyeless lizard, when in actuality it feels more like an observed marble. The zeitgeist contends that the british is a bulb.
```

On peut faire ceci avec la commande suivante :

```bash
cargo run encrypt --key="VEXCITED" < nom_du_fichier_dechiffr.txt
```

On obtient le texte chiffré suivant :

```plaintext
Ollum lguvtbta tvh isqjqgk pjvb vptr ozxqwkxw. Rii zcvgsw nimcztxh wiatwhqv avlo xkivos lxmkgrvxp. C xtxlj sc vpx govzb ka twvpqbf bh fh v jlgbtp ucmkqkxvrn. E mqqlsq nibu i csnz ep c lhvpdi pephso. Olfu ql rro xl fqlguzhfv bai lyix vptx d ymchmkiqxi zcv aeuypv dm vsqnmagzxh d xpfpor grmrbv ebxkjyq ctls ezmki i yekmikjmbx. Kjabxmk, xkz kxntbeuy txlife fjqbu nksp vr rpoeeg wibh. Bai czmqimbww xskvmghv olxv bai yjvxpb iyuxlxum vspzw ctwf e pdqfe cgmw. Aeo hzhq wci qtcml, d cmjcttcdi sc vpx vhgmpj ql evnyjgl ms ez ek gfmvrmwb rmtv. La xeka pev nsjgeaew prznmtv, d xlfom hj wci qqmgelg mp calypzh qq jx e vkekitr qrqi. Qjm wmdklocof si v hbumkx ezgloml e izwqkdx oloxbp.

Qg eqxmbpb mmpzw qjmr ahmi iqam alollwb mlh prccqk pbmmz vptx fjqmqaxh wcift anviwsxtl. T wdgea eig ldmhia jx griwffmkig v krouhyv osovmeplim tkbasxo eiuw uilik x oibp. D olokte mv v xs hzhq wci okoax szvprmvxlqi. X epbrd dw xp imxhixfqv'l tuzgfrqmewdsk. Yptx zz hlp'b drrr jlt anvh dw tjmmlhm so pwm wrhi mqabx wci qgvliu rein bh fh gipu baeq bixttxwv. V jbtbbpluio umxw d bssgzgsu vw x dwtwwayi cyneudyp. Uwfi vdpqa xxrv vvb vphyjcx lh abqsgc xu xksidxp. Uwfi vjjqywhh nzxqnml euz xeqczlw jj pkuipb vw yqel. Lrrisgz, tywcsou wyxhi qfuqgxhmtogb mlh frlv il eq zcbnmlw oddxtl, plhi mk ckmydgmqa qm jhzpp owki odob cv hfvzvsgl feuwpb. Vpx dhdxdgqlx fjrqgvww wceq vpx fudxfup bw d wyid.
```

On peut ensuite le déchiffrer avec la commande suivante :

```bash
cargo run decrypt --key="VEXCITED" < nom_du_fichier_chiffr.txt
```

On obtient bien le texte que l'on avait au départ.

À noter que l'on utilise la clé `VEXC ITED123!!` par exemple, le chiffrement et déchiffrement fonctionne de la même manière puisque les caractères spéciaux, nombres et espaces sont ignorés dans la clé.

On remarque bien dans le texte chiffré que l'on obtient que les espaces et les caractères spéciaux sont inchangés, de même que pour la casse des lettres.

## Kasiski

### Traitement du texte

Je prend en entrée le texte chiffré et à la sortie on affiche la longueur de la clé - ou un message d'erreur si non trouvable.

```bash
# Vous pouvez utiliser un fichier en entrée (stdin)
cargo run kasiski < input.txt

# Vous pouvez aussi utiliser un texte en entrée
echo "hello" | cargo run kasiski
```

### Détails sur l'implémentation

Pour trouver la longueur de la clé, j'ai...

1. Recherché les fragments répétés dans le texte chiffré en utilisant `windows(fragment_len)` ;
2. Stocké les positions de chaque fragments dans `fragment_positions`, un `HashMap` ;
3. Calculé les distances entre les fragments répétés dans `fragment_positions` ;
4. Stocké ces distances dans le vecteur `candidates` ;
5. Trié et dédoublé les distances dans `candidates` ;
6. Calculé le PGCD des distances dans `candidates`.

Pour être sûr d'avoir la bonne longueur de clé, on va vérifier si deux résultats sont égaux (le premier et le deuxième plus grand PGCD).

Si le PGCD est différent de 1, on affiche la longueur de la clé.
Sinon, on reprend depuis l'étape 1 avec un fragment plus grand de +1.

> Attention ! Mon implémentation n'enlève pas les espaces et ne vérifie pas la casse des lettres.
> Ainsi, si vous avez des espaces ou une casse mixte dans votre texte chiffré, cela peut fausser le résultat.

### Jeu d'essais

> Le répertoire `tests` est inclus dans le `.zip` qui vous a été remis.

Voici deux jeu d'essais de cas assez simple sur deux clés différents :

- Le fichier `tests/20-output.txt` a été chiffré à partir de `tests/20-input.txt` avec la clé `WORLD` (5).
  - `cargo run kasiski < ./tests/20-output.txt` nous donne correctement `5` comme longueur de clé.
- Le fichier `tests/30-output-td.txt` a été chiffré à partir de `tests/30-input-td.txt` avec la clé `PARADIGM` (8).
  - `cargo run kasiski < ./tests/30-output-td.txt` nous donne correctement `8` comme longueur de clé.

Voici un jeu d'essai qui contient des caractères spéciaux :

- Le fichier `tests/50-output.txt` a été chiffré à partir de `tests/50-input.txt` avec la clé `CHARSPECIALS` (12).
  - Le chiffrement a été réalisé avec l'outil <https://cryptii.com/pipes/vigenere-cipher>, avec l'alphabet `achrspeil!@%&#(){}|\`, avec la stratégie `Ignore case` et `Ignore foreign chars`.
  - `cargo run kasiski < ./tests/50-output.txt` nous donne correctement `12` comme longueur de clé.
