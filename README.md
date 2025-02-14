# Jeu De La Vie en **RUST**

## À quoi ça sert ?

Bonne question ! En réalité je l'ai fait en Java déjà et j'aime pas le fait qur la JVM consomme 3 GB de RAM donc je la tente en Rust.

## Prérequis

* Un cerveau :brain:
* Une connexion internet
* Et une vrai liste de prérequis que voici :
  * La Rust tolchain classic (cargo, rustc, rustup)

## Comment on le lance ?

Deuxième bonne question !

### Version Rust dans une fenêtre classique

    cargo run --release

### Version WASM dans un navigateur web (en local)

    rustup target install wasm32-unknown-unknown
    cargo install wasm-server-runner
    cargo run --target wasm32-unknown-unknown
    cargo serve

### Version WASM pour déploiement

Prérequis :

    rustup target add wasm32-unknown-unknown
    cargo install wasm-bindgen-cli

Commande pour build le WASM et générer le JS :

    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --no-typescript --out-dir ./webapp/ --target web ./target/wasm32-unknown-unknown/release/Jeu_de_la_Vie.wasm

### Rappel pour moi

> :no_entry: NE LIS PAS SI TU N'ES PAS MOI !!

déploiement : `scp -r webapp/* tongue@tonguechaude.fr:/var/www/tonguechaude.github.io/gol`

## License

On fait que du Logiciel Libre ici !! Blague à part le code est sous licence GNU GPL -V3
