# dla-rust

<p>
 <img src=./pics/cardioid.png width="200" height="200">
 <img src=./pics/cardioid_3.png width="200" height="200">
 <img src=./pics/polar_perlin.gif width="200" height="200">
</p>
 <img src=./pics/pi_collide_4.gif width="500" height="200">

# Graphics Archive https://disdeal.github.io/


### Compiled files are located in "html" folder and can viewed on [site.](https://disdeal.github.io/)

#### You can compile source files from "src" folder.

* Golang files

```bash
gopherjs build -m main.go -o main.js
```

Dependencies: [PGoJS](https://github.com/bregydoc/PGoJs)

* Rust files

```bash
cargo build --release
```

```bash
cargo web build --release
```
If you want to test your application locally, use `cargo web start` and open browser to the port it provides.

```bash
cargo web deploy
```
Dependencies: [quicksilver](https://github.com/ryanisaacg/quicksilver), [cargo-web
](https://github.com/koute/cargo-web), 


