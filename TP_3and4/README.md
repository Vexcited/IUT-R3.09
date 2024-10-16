# Rapport : TP 3 et 4

> Rédigé par Mikkel ALMONTE--RINGAUD, G4B.

## Préparation

Dans les 3 langages imposés, j'ai pu trouver des bibliothèques de primitives cryptographiques qui fournissent à minima SHA1, SHA256 et HMAC.

- Java : `bouncycastle` (<https://www.bouncycastle.org/>) et `javax.crypto` qui est fourni dans Java ;
- Rust : L'ensemble de bibliothèques que fourni `RustCrypto` dans `hashes`, <https://github.com/RustCrypto/hashes> et `MACs`, <https://github.com/RustCrypto/MACs> ;
- Python : `hashlib` (<https://docs.python.org/3/library/hashlib.html>) et `hmac` (<https://docs.python.org/3/library/hmac.html>) qui sont fournis dans Python

J'ai décidé de partir sur Rust pour sa simplicité et sa documentation de haute qualité. Les bibliothèques expliquent simplement et complètement comment fonctionne chaque fonctionnalités et donnent même des exemples.

## Exercice 1 : Des mots de passe tout simples

Il nous faut produire une chaîne de 8 caractères qui provient de deux chaînes de caractères passées en paramètre.

Premièrement, nous allons définir les caractères que nous autorisons.

```rust
const ALLOWED_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!?@#$%&*()[]|:;,.";
```

Les caractères que l'on autorise sont :
- lettres majuscules : `A-Z`
- lettres minuscules : `a-z`
- chiffres : `0-9`
- les caractères spéciaux suivants : `!?@#$%&*()[]|:;,.`

Ainsi, nous n'autorisons pas :
- les espaces
- les caractères invisibles

et d'autres caractères spéciaux.

Ensuite, nous allons définir une fonction qui prend deux chaînes de caractères et qui va les concaténer et générer un SHA-256 de cette chaîne.

```rust
fn make256(master: &str, tag: &str) -> Vec<u8> {
  let input = format!("{master}{tag}");
  let input = input.as_bytes();

  let mut hasher = Sha256::new();
  hasher.update(input);
  hasher.finalize().to_vec()
}
```

Pour être sûr que les deux chaînes de caractères sont bien autorisées, nous allons créer une fonction qui va vérifier que chaque caractère est bien dans la liste des caractères autorisés.

```rust
pub fn check_allowed_charset(text: &str) -> bool {
  text.chars().all(|c| ALLOWED_CHARSET.contains(&(c as u8)))
}
```

Nous allons ainsi utiliser cette fonction dans notre fonction `make256` pour vérifier que les deux chaînes de caractères passées en paramètre sont bien autorisées.

```rust
fn make256(master: &str, tag: &str) -> Result<Vec<u8>, String> {
  if !check_allowed_charset(master) || !check_allowed_charset(tag) {
    return Err("Le mot de passe maître ou le tag contient des caractères non autorisés".into());
  }

  let input = format!("{master}{tag}");
  let input = input.as_bytes();

  let mut hasher = Sha256::new();
  hasher.update(input);

  Ok(hasher.finalize().to_vec())
}
```

On a transformé le retour de la fonction en `Result<Vec<u8>, String>` pour pouvoir renvoyer une erreur si les caractères ne sont pas autorisés.

Enfin, nous allons traduire ce SHA-256 que nous obtenons en une chaîne de 8 caractères.

```rust
pub fn generate_password(master: &str, tag: &str) -> Result<String, String> {
  // On utilise notre fonction pour générer un hash SHA-256
  // à partir de `master` et `tag`.
  let hash = make256(master, tag)?;

  // On initialise une chaîne de caractères avec une capacité de 8.
  let mut output = String::with_capacity(8);

  // Tant qu'on a pas 8 caractères dans notre chaîne de sortie...
  while output.len() < 8 {
    let index = output.len();

    // On prend un caractère du hash que l'on convertit en nombre non signé.
    let value = hash[index % hash.len()] as usize;
    
    // On ajoute l'index pour avoir un nombre "unique".
    let value = value + index;

    // On ajoute le caractère correspondant à la valeur modulo la taille du charset.
    output.push(ALLOWED_CHARSET[value % ALLOWED_CHARSET.len()] as char);
  }

  Ok(output)
}
```

### Essais

Le mot de passe maître peut être faible et pourtant, le mot de passe généré sera toujours fort avec assez peu de répétitions.

- `MonMDP!` et `unilim` : `6*hG0|6z`, on a bien 8 caractères et on a des caractères spéciaux.
- `kiwi` et `facebook` : `)EW*UuH?`, on a bien 8 caractères et on a des caractères spéciaux.
- `azerty` et `qwerty` : `NTQu%1fR`, on a bien 8 caractères et on a des caractères spéciaux.
- `MonMDP ‎` et `unilim` : erreur, car espace dans `master`
- `MonMDP` et `unilim ‎` : erreur, car espace dans `tag`

> Par ailleurs, vous pouvez retrouver ces tests dans `src/password.rs` dans `mod tests`.
> Vous pouvez ainsi exécuter les tests en utilisant `cargo test`.

## Exercice 2 : Des mots de passe d'une taille demandée

Dans cet exercice, nous allons générer un mot de passe de taille `n` à partir d'un mot de passe maître et d'un tag.

On va tout simplement reprendre notre `generate_password` et ajouter un paramètre `length` qui va définir la taille du mot de passe.

```rust
pub fn generate_password(master: &str, tag: &str, length: usize) -> Result<String, String> {
  // On utilise notre fonction pour générer un hash SHA-256
  // à partir de `master` et `tag`.
  let hash = make256(master, tag)?;

  // On initialise une chaîne de caractères avec une capacité de `length`.
  let mut output = String::with_capacity(length);

  // Tant qu'on a pas `length` caractères dans notre chaîne de sortie...
  while output.len() < length {
    let index = output.len();

    // On prend un caractère du hash que l'on convertit en nombre non signé.
    let value = hash[index % hash.len()] as usize;
    
    // On ajoute l'index pour avoir un nombre "unique".
    let value = value + index;

    // On ajoute le caractère correspondant à la valeur modulo la taille du charset.
    output.push(ALLOWED_CHARSET[value % ALLOWED_CHARSET_LEN] as char);
  }

  Ok(output)
}
```

On utilise toujours SHA-256 ici car c'est toujours suffisant.

### Essais

#### `MonMDP!` et `unilim` de taille 24

```plaintext
6*hG0|6zdcc5OiInw[lGAiuv`
```

On a bien 24 caractères et des caractères spéciaux.

#### `kiwi` et `facebook` de taille 2056

Celui là est important car il va nous permettre de tester la non répétition des caractères : SHA-256 génère 256 bits, soit 32 caractères, on va donc répéter plusieurs fois la même chaîne de caractères dans notre fonction.

Ici, on va donc tester si on a bien 2056 caractères qui ne se répètent pas malgré la répétition de la chaîne de caractères.

```plaintext
)EW*UuH?|qi8tpr#jQlrp!4:UXG?[u7iYk2W0.nQb|$N,]:S%w*:]PJc03mQZ.M$4&H2Ff)w7bTteacyUBWcavp8FI(w5fsTJVnHl?YBM7z,!68D0h286A]NloXBK?;zp1)n*Q4hsMEePLNjF#HNLgat*[3hqQdE]GY)WwJ#;sk!vrt%lSntr@6,WZI#|w9kam4Y2BpSd;&PA:,U*y),:RLe25oSbBO&6(J4Hh]y9dVvgce0WDYecxr!HK[y7huVLXpJn#aDO91A@8!F2j4!8C:PnqZDM#.1r3]p)S6juOGgRNPlH%JPNicv)|5jsSfG:Ia]YyL%.um@xtv*nUpvt$8AYbK%;y?mco6a4DrUf.(RC,AW)0]A,TNg47qUdDQ(8[L6Jj:0?fXxieg2YFagezt@JM|09jwXNZrLp%cFQ?3C$!@H4l6@!E,RpsbFO%B3t5:r]U8lwQIiTPRnJ*LRPkex];7luUhI,Kc:a0N*Bwo$zvx)pWrxv&!CadM*.0#oeq8c6FtWhB[TEACY]2:CAVPi69sWfFS[!|N8Ll,2#hZzkgi4aHcig1v$LO;2?lyZPbtNr*eHS#5E&@$J6n8$@GATrudHQ*D5v7,t:W!nySKkVRTpL)NTRmgz:.9nwWjKAMe,c2P)Dyq&1xz]rYtzx(@EcfO)B2%qgs!e8HvYjD|VGCEa:4,ECXRk8?uYhHU|@;P!NnA4%jb1mik6cJeki3x&NQ.4#n0bRdvPt)gJU%7G($&L8p!&$ICVtwfJS)F7x9Av,Y@p0UMmXTVrN]PVToi1,B?pyYlMCOgAe4R]F0s(3z1:tav1z[$GehQ]D4*siu@g!JxalF;XIEGc,6AGEZTm!#wajJW;$.R@PpC6*ld3okm8eLgmk5z(PSB6%p2dTfxRv]iLW*9I[&(N!r@(&KEXvyhLU]H9z?CxAa$r2WOoZVXtP:RXVqk3AD#r0anOEQiCg6T:H2u[513,vcx31|&IgjS:F6)ukw$i@LzcnH.ZKGIeA8CIGbVo@%yclLY.&BT$RrE8)nf5qmo!gNiom71[RUD8*r4fVhzTx:kNY)?K|([P@t$[(MGZx0jNW:J?1#EzCc&t4YQqbXZvR,TZXsm5CF%t2cpQGSkEi8V,J4w|735Axez53;(KilU,H8]wmy&k$N1epJBbMIKgC!EKIdXq$*0enNaB(DV&TtG!]ph7soq@iPkqo93|TWF!)t6hXj1Vz,mPa]#M;[|R$v&|[OIbz2lPY,L#3%G1Ee(v6aSsdZbxTAVbZuo7EH*v4erSIUmGk!XAL6y;957Czg175.[MknWAJ!:yo0(m&P3grLDdOKMiE@GMKfZs&)2gpPcD[FX(VvI@:rj9uqs$kRmsq?5;VYH@]v8jZl3X1AoRc:%O.|;T&x(;|QKd14nRaAN%5*I3Gg[x8cUufbdzVCXdbwq9GJ)x6gtUKWoIm@ZCN80.?79E1i397B|OmpYCL@,0q2[o(R5itNFfQMOkG$IOMhbu(]4irReF|HZ[XxK$,tl?wsu&mTous#7.XaJ$:x!lbn5Z3CqTe,*QB;.V(z[.;SMf36pTcCP*7)K5Ii|z!eWwhdf1XEZfdys?IL]z8ivWMYqKo$bEP!2B#9?G3k5?9D;QoraEN$A2s4|q[T7kvPHhSOQmI&KQOjdw[:6ktTgH;Jb|ZzM&Avn#yuw(oVqwu%9BZcL&,z@ndp7b5EsVgA)SD.BX[1|B.UOh58rVeER)9]M7Kk;1@gYyjfh3ZGbhf0u#KN:1!kxYOasMq&dGR@4D%?#I5m7#?F.SqtcGP&C4u6;s|V9mxRJjUQSoK(MSQlfy|,8mvViJ.Ld;b1O(Cxp%0wy[qXsyw*?DbeN(A1$pfr9d7GuXiC]UFBDZ|3;DBWQj7!tXgGT]?:O9Mm.3$ia0lhj5bIdjh2w%MP,3@mzaQcuOs(fIT$6F*#%K7o9%#HBUsveIR(E6w8.u;X?ozTLlWSUqM[OUSnh0;A!oxXkLBNf.d3Q[Ezr*2y0|sZu0y)#FdgP[C3&rht?f9IwZkE:WHDFb;5.FDYSl9@vZiIV:#,Q?OoB5
```

On a bien 2056 caractères et des caractères spéciaux sans répétition.

### `M@tDeP@ss?:)` et `discord` de taille 64

On vérifie le double des caractères générés par rapport à la taille du hash SHA-256.

```plaintext
QcdHxAVBYLOY:6#IElWgV.o7zt,vGYR]w89nCg1h4ru4cLSok*2@1f[ME,eAm4xa
```

On voit que les caractères ne se répètent pas et on a bien 64 caractères.

## Exercice 3 : Mot de passe maître

Pour stocker le mot de passe maître, je place un fichier `mpwd.txt` à l'endroit où est exécuté le programme, vous pourrez retrouver comment je fais cela dans `src/storage.rs`.

Lorsque le programme s'exécute, il va vérifier si le fichier existe, si oui, il va demander à l'utilisateur s'il veut utiliser ce mot de passe ou s'il veut en choisir un nouveau. Au cas où le fichier n'existe pas, le programme demande à l'utilisateur de saisir un mot de passe maître directement.

Lors de la saisie du mot de passe maître, le programme va demander à l'utilisateur de le saisir une deuxième fois pour vérifier qu'il n'y a pas d'erreur de frappe.

Une fois la saisie faite, le programme va stocker le mot de passe maître dans le fichier `mpwd.txt`.

## Exercice 4 : Attaques sur des mots de passe

> Le code source pour l'attaque par force brute est dans `attack/main.rs`.
> Vous pouvez l'exécuter en utilisant `cargo run --example attack`.

On va premièrement prendre un mot de passe maître de 10 caractères : `MonMDP!123`.

Sachant que la liste des caractères autorisés sont les suivants : `ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!?@#$%&*()[]|:;,.`,
on a donc 79 caractères possibles sur 10 caractères, soit `79^10` (= `9468276082626847201`) possibilités.

Ainsi, la probabilité de trouver le mot de passe maître est de `1/9468276082626847201` soit `1.056e-19` (= `0.0000000000000000001056`)

On va maintenant générer des mots de passe d'un caractère :
- Unilim : `)`
- Amazon : `n`
- Netflix : `q`

On souhaite maintenant trouver un mot de passe maître qui donnera le même mot de passe que celui produit.

Après 13 tentatives, on trouve un mot de passe maître qui donne le même résultat pour `Unilim` : `AAAAAAAAAM`.

Théoriquement, pour un mot de passe généré d'un seul caractère (N=1), il y a 79 sorties possibles.
La probabilité de collision pour un essai est donc de 1/79.

Pour estimer le nombre d'essais nécessaires pour trouver une collision, nous pouvons utiliser l'approximation du paradoxe des anniversaires :

- `√(π * n / 2)`, où n est le nombre de sorties possibles (79 dans notre cas).
- `√(π * 79 / 2) ≈ 11.14`

Donc, en théorie, on s'attendrait à trouver une collision après environ 11 essais.
Ce qui est à peu près ce que nous avons trouvé : 13 essais.

---

Maintenant, nous allons trouver un mot de passe maître qui provoque une collision pour les trois tags.

Après 1131248, on trouve un mot de passe maître qui donne le même résultat pour `Unilim`, `Amazon` et `Netflix` : `AAAAAACXUu`.

On remarque que le nombre d'essais est bien plus grand que pour un seul tag, ce qui est normal car la probabilité de collision est plus faible.

---

Recommençons avec N = 2 maintenant.

- 1er temps, `6908` tentatives pour `Unilim` avec le mot de passe maître : `AAAAAAABIi`
- 2ème temps, ???? tentatives, après plus de 50 millions de tentatives, on n'a pas trouvé de mot de passe maître qui donne le même résultat pour `Unilim`, `Amazon` et `Netflix`.

---

Recommençons avec N = 3 maintenant.

- 1er temps, `572904` tentatives pour `Unilim` avec le mot de passe maître : `AAAAAABM!|`
- 2ème temps, ???? tentatives, après plus de 100 millions de tentatives, on n'a pas trouvé de mot de passe maître qui donne le même résultat pour `Unilim`, `Amazon` et `Netflix`.

---

Pour conclure, une utilisation correcte du générateur serait de générer mot de passe assez long pour éviter le plus possible les collisions qui permettrait de retrouver le mot de passe maître.

Il faut aussi choisir un mot de passe maître long pour augmenter le nombre de possibilités lors d'une attaque par force brute.

Il faut aussi que ce mot de passe soit complexe pour éviter les attaques par dictionnaire.
