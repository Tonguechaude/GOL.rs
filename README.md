# Jeu De La Vie en **RUST**

![License](https://img.shields.io/badge/License-GPLv3-blue.svg)

Le **Jeu de la Vie** est une simulation cellulaire automatisée conçue par le mathématicien John Conway. Ce projet est une implémentation en **Rust** du célèbre algorithme, avec une interface graphique utilisant **Bevy** et une version WebAssembly (WASM) pour une exécution dans le navigateur.

---

## À quoi ça sert ?

Bonne question ! En réalité je l'ai fait en Java déjà et j'aime pas le fait qur la JVM consomme 3 GB de RAM donc je la tente en Rust.

---

## Fonctionnalités

- **Simulation du Jeu de la Vie** : Implémentation de l'algorithme classique de Conway.
- **Interface graphique** : Utilisation de **Bevy** pour une interface 2D interactive.
- **Version WebAssembly** : Exécution dans un navigateur web avec une interface simple.

---

## Prérequis

Pour utiliser ce projet, vous aurez besoin des outils suivants :

- Un **cerveau** :brain:
- Une **connexion internet**
- Et une vrai liste de prérequis que voici : 
- **Rust Toolchain** :
  - `rustc` (compilateur Rust)
  - `cargo` (gestionnaire de paquets Rust)
  - `rustup` (gestionnaire de versions Rust)
- **Outils supplémentaires pour WASM** :
  - `wasm32-unknown-unknown` (cible pour la compilation WebAssembly)
  - `wasm-bindgen-cli` (pour générer les bindings JavaScript)
  - `wasm-server-runner` (pour exécuter le projet en local)

---

## Installation

1. Clonez ce dépôt :

```bash
git clone https://github.com/Tonguechaude/GOL.rs.git
cd GOL.rs
```
   
2. Installez les dépendances Rust :

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli wasm-server-runner
```

## Utilisation
    
### Version Rust (fenêtre classique)
    
Pour compiler et exécuter le projet en mode release :

```bash
cargo run --release
```    

### Version WASM (en local)
    
Pour exécuter le projet dans un navigateur web en local :

```bash    
cargo run --target wasm32-unknown-unknown
```
ou
```bash
cargo serveur
```
    
### Version WASM (pour déploiement)
    
Pour compiler le projet en WebAssembly et générer les fichiers JavaScript :

```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --out-dir ./webapp/ --target web ./target/wasm32-unknown-unknown/release/jeu_de_la_vie.wasm
wasm-opt -Oz -o ./webapp/jeu_de_la_vie_bg.wasm ./webapp/jeu_de_la_vie_bg.wasm # Optimize WASM file size
```

## Dépendances
    
Ce projet utilise les dépendances suivantes :
    
**Bevy** : Moteur de jeu pour l'interface graphique.  
**egui** : Interface utilisateur pour la version Rust.  
**rand** : Génération de nombres aléatoires pour l'initialisation de la grille.  
**getrandom** : génération de nombre aléatoir compatible avbec la cible wasm32
**wasm-bindgen** : Pour la compatibilité WebAssembly.  
    
## Docker

Actually I exposed a docker image here : [tonguechaude/rust-wasm-builder](https://hub.docker.com/r/tonguechaude/rust-wasm-builder)

This image exist just because my runner CPU is so bad :(, so I need to optimize compute time in CI.

## Contribuer
    
Les contributions sont les bienvenues ! Si vous souhaitez améliorer ce projet, voici comment procéder :
    
1. Forkez ce dépôt.
2. Créez une branche pour votre fonctionnalité (git checkout -b feature/nouvelle-fonctionnalite).
3. Committez vos changements (git commit -am 'Ajouter une nouvelle fonctionnalité').
4. Poussez vers la branche (git push origin feature/nouvelle-fonctionnalite).
5. Ouvrez une Pull Request.
    
## License
    
On fait que du Logiciel Libre ici !! Blague à part le code est sous licence **GNU GPL v3**

Amusez-vous bien avec le Jeu de la Vie en Rust ! 🚀
