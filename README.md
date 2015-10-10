# Множесто Мандельброта при помощи OpenGL и Rust

Для сборки нужен компилятор Rust, его можно скачать тут:
[Rust](https://www.rust-lang.org/), или же, для Linux и Mac:

`curl -sf -L https://static.rust-lang.org/rustup.sh | sh`

## Запуск

`cargo run` из директории `mal`.

## Управление

* `w, s, a, d` --- движение,
* `j, k` --- изменение масштаба,
* `r` --- перезагрузка шейдеров и конфигурации.

В файле `config.json` находится конфигурация в формате `JSON`.


Для использования OpenGL из Rust была использованна библиотека glium.
[Туториал](http://tomaka.github.io/glium/book/).
