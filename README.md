# Множество Мандельброта при помощи OpenGL и Rust

Для сборки нужен компилятор Rust, его можно скачать тут:
[Rust](https://www.rust-lang.org/), или же, для Linux и Mac:

`curl -sf -L https://static.rust-lang.org/rustup.sh | sh`.

Для сборки под Windows вам также понадобится [MinGW-64](http://msys2.github.io/)


## Запуск

`cargo run` из директории `mandelbrot`.


## Управление

* `w, s, a, d` --- движение,
* `j, k` --- изменение масштаба.


## Tech

Для использования OpenGL из Rust была использованна библиотека glium.
[Туториал](http://tomaka.github.io/glium/book/).

Для виджитов была использована библиотека [imgui-rs](https://github.com/Gekkio/imgui-rs).
