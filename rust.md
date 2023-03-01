# ***¿Qué es Rust?***

Rust es un lenguaje de programación de código abierto, enfocado en velocidad, seguridad y 
concurrencia.

Nacio para la programación de sistemas, lo cual es el software que da servicio a otro software, 
y generalmente tiene limitaciones de recursos, i.e Sistemas operativos, browsers, engine de 
videojuegos.

Soporta multiples paradigmas

- Programación imperativa.
- Programación orientada a objetos.
- Programación funcional.
- Programación concurrente.

## ***¿Qué se puede hacer con Rust?***
- Aplicaciones de línea de comando.
- Sistemas operativos.
- Browser engine.
- Servidores.
- Escribir partes de una aplicación que sea a bajo nivel, o tiene ciertos requerimientos de performance.
- Webassembly.
- Sistemas embebidos, microcontroladores, IoT (Internet of Things).

## ***¿Por qué usar Rust?***
- Seguridad de tipos:
    El compilador nos asegura que ninguna operación será aplicada a una variable
    de tipo equivocado

- Seguridad de memoria: 
    Todas las referencias siempre apuntarán a memoria válida

- Sin condiciones de carrera: 
    Sistema ownership de Rust nos garantiza que multiples partes del programa
    no puedan modificar el valor al mismo tiempo.

- Abstracciones a consto cero: 
    Rust nos permite usar conceptos de alto nivel (interfaces, iteraciones, 
    enums, programación funcional...) conun alto costo nulo o mínimo en 
    performance.

- Runtime mínimo: 
    Rust tiene un runtime mínimo y lo más optimizado posible, similar a C o 
    C++.

# ***Cargo***
Cargo es el gestor de paquetes en Rust, com un NPM lo es para JS, o un PIP 
para un Python

```
    // crear nuevo proyecto con cargo

    cargo new name_project

    // crear una nueva librería

    cargo new name_library --lib

    // crear ejecutable

    cargo build

    // el compilador va a agregar optimizaciones sobre el programa

    cargo build --release 

    /*
        * Hace que sea menos pesado y más rápido a la hora de ejecutarse
    */

    // compilar y ejecutar el proyecto de una vez

    cargo run

    // sirve para ver si el proyecto compila correctamente

    cargo check
```

# ***Crates.io***
En este sitio se encuentran los paquetes que se pueden usar en la aplicaciones
de Rust, donde se pueden encontrar bastante paquetes para diferentes tipos de 
desarrollo

[crates.io](https://crates.io/)

## ***¿Cómo agregar un nuevo paquete o dependencia al proyecto?***
Esto se puede hacer de dos forma tanto copiando y pegando el nombre del paquete con su versión en el archivo ***Cargo.toml*** o por medio de la línea de comandos.

- ### ***En el archivo Cargo.toml***
    Hay una sección que dice ***[dependencies]*** y de bajo se pone el nobre y la versión del paquete, y queda listo para usar

    ```
        [dependencies]
        syn = "1.0.109"
        ... otros más
    ```
- ### ***Por línea de comandos***
    ```
        // para agregar una dependencia

        cargo add name_dependency

        // para agregar una feature de una dependencia en concreto

        cargo add name_dependency --features derive
    ```