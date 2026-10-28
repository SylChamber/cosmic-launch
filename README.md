# cosmic-launch - Lance ou active une application sous Cosmic

`cosmic-launch` est un outil terminal qui lance une application, ou l'active sous COSMIC si elle est déjà chargée.

Utilise le binaire [cosmicmsg](https://github.com/varbhat/cosmicmsg), qui doit se trouver dans le `PATH`. `cosmicmsg` a été compilé pour AMD64 et est inclus dans les _releases_.

> Code contribué par Mistral. `varbhat/cosmicmsg` est distribué sur licence MIT.

## Compilation et installation

Pour compiler:

```sh
cargo build --release
```

Installer:

```sh
sudo install -m 0755 target/release/cosmic-launch /usr/local/bin
```

## Utilisation

Spécifier le nom de l'exécutable en argument:

```sh
cosmic-launch brave-origin
```

`cosmic-launch` est conçue pour émuler le fonctionnement des raccourcis `Super + 0..9` dans les environnements GNOME, KDE et Windows:

* si l'application n'est pas en mémoire, lance l'application
* si l'application est déjà chargée, active la première fenêtre retournée par `cosmicmsg get-toplevels` (ordre alphabétique de titre de fenêtre)
* si une fenêtre de l'application demandée est déjà sous focus, active la prochaine fenêtre

> Le traitement n'est pas instantané. Ça doit dépendre de `cosmicmsg` et du délai d'interprétation des résultats JSON.

Comme les raccourcis `Super + 0..9` sont déjà assignés, utiliser les raccourcis `Super + [a-z]` libres ou déjà réservés pour les usages pertinents:

* `Super + t`: terminal
* `Super + b`: navigateur web
* `Super + f`: fichiers
* `Super + e` (libre): éditeur, par exemple Gram
* `Super + v` (libre): VSCodium

Pour assigner un raccourci, ouvrir **Paramètres > Périphériques d'entrée > Clavier > Raccourcis clavier > Voir et personnaliser les raccourcis > Personnalisé** et:

* saisir un nom de raccourci sans espace
* inscrire la commande `cosmic-launch <binaire>`, par exemple `cosmic-launch gram`
* assigner le raccourci, par exemple `Super + e`
