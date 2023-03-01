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

    cargo new hello_cargo
```
Ese es el comando para crear una aplicación en Rust