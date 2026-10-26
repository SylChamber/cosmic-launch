# cosmic-launch - Lance ou active une application sous Cosmic

`cosmic-launch` est un outil terminal qui lance une application, ou l'active sous COSMIC si elle est déjà chargée.

Utilise le binaire [cosmicmsg](https://github.com/varbhat/cosmicmsg), qui doit se trouver dans le `PATH`.

## Compilation et installation

Pour compiler:

```sh
cargo build --release
```

Installer:

```sh
sudo install -m 0755 target/release/cosmic-launch /usr/local/bin
```
