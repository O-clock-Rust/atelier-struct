# Atelier : Les Struct et autres plaisirs

Il est temps de mettre en pratique ces premières notions Rust :

- variables
- fonctions
- conditions
- struct / Option / Result

## Instructions

1. Crée un nouveau projet Rust en utilisant Cargo
   
2. Déclare une variable `current_year` qui prendra comme valeur… l'année actuelle
   
3. Définis un struct `Car` selon les instructions suivantes :
    - champs : `brand`, `model` et `year` (année de construction du véhicule)
    - constructeur : méthode associée `new`
    - méthode d'instance `describe` qui formate une chaîne de caractères pour décrire
      l'instance du struct (ex : « In 1998, Fiat created the Multipla »)
    - méthode d'instance `car_age` qui calcule l'âge de la voiture en lui passant
      en argument `current_year` (déclarée à l'étape 2)

4. Écrire un programme permettant de créer une instance du struct `Car`
   et affichant la description de la voiture en console

5. Implémente une méthode `is_classic` qui retourne une `Option` pour vérifier si la voiture
   est ancienne (plus de 10 ans) :

   - retourne l'instance elle-même si la voiture a plus de 10 ans
   - retourne `None` sinon

   > peut-être utiliser `car_age` dans la fonction… ;)  
   > n'oublie pas : `car_age` prend l'année en cours comme paramètre… ;) ;)

6. Complète le programme en incorporant un appel à la fonction `is_classic`

7. Modifie l'année actuelle (`current_year` – normalement utilisée ci-dessus) par `1900`…
   Que se passe-t-il ? Essaie de comprendre ce qu'il se passe avant de continuer.

   <details>
    <summary>Explications ?</summary>

    Le programme **panique** !  
    Rust rencontre une situation qu'il ne peut pas gérer correctement :
    (ici) convertir un nombre négatif (résultat de `1900 - 1998`)
    en type `u32` (_unsigned_).  
    En mode _Debug_, le programme se termine avec un message d'erreur.  
    En mode _Release_, il effectuerait un _wrapping_ de la valeur :
    la valeur serait « enroulait » autour des limites du type ;
    « -98 » deviendrait `4 294 967 198` en `u32` (`2^32 - 98`).

    → <https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html>
   </details>

8. Corrige la méthode `car_age` pour prendre en compte ce comportement.
   Elle doit retourner un `Result` :

   - l'âge de la voiture si celui-ci est positif
   - une erreur (`Current year cannot be less than the car's year of manufacture.`) sinon

9. Transforme la méthode `is_classic` en conséquence :
    elle doit utiliser le `Result` de `car_age` et renvoyer un `Result<bool, String>`
    grâce à l'opérateur `?` (gestion automatique des erreurs)

   <details>
    <summary>Un peu d'aide ?</summary>

    Dans le `Ok` de `is_classic`, tu peux directement renvoyée une condition…  
    Celle-ci sera alors gérée par le `match` grâce à :

    - `Ok(true)`
    - `Ok(false)`
    - `Err(e)`

    Exemple :

    ```rust
    fn main() {
        let answer = "yes"; // Modifie cette valeur pour tester différents cas
        
        match is_rust_cool(answer) {
            Ok(true) => println!("Rust is cool !"),
            Ok(false) => println!("Rust isn't cool… Really?!?"),
            Err(e) => println!("Error: {}", e)
        }
    }

    fn is_rust_cool(answer: &str) -> Result<bool, String> {
        let allowed_answers = ["yes", "no"];
        let is_cool = check_answer(answer, &allowed_answers)?;
        // s'il n'y a pas d'erreur, on retourne le résultat sous forme de booléen
        // → on peut directement passer le résultat d'une condition
        Ok(is_cool == "yes")
    }

    fn check_answer<'a>(answer: &'a str, allowed_responses: &[&'a str]) -> Result<&'a str, String> {
        // Si la réponse est contenue dans le tableau des réponses autorisées
        if allowed_responses.contains(&answer) {
            // on renvoie Ok avec la réponse
            Ok(answer)
        } else {
            // Si la réponse n'est pas valide, on retourne une erreur
            let msg = format!(
                "Unrecognized response, please answer with a valid option: {:?}",
                allowed_responses
            );
            Err(msg)
        }
    }
    ```
   </details>
