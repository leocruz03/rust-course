fn main() {
    
    // Lint

    /*
        es una regla que nuestro código tiene que seguir, pero el linter es la herramienta que chequea 
        esos lints o reglas en el código, en rust el mismo compilador del lenguaje tiene un lint.

        en lint hay varios niveles, que se pueden decir que son niveles de permisividad, si es que esta 
        regla se tiene que cumplir o no

        --> allow:
            son líneas que por defecto no hacen nada, pero son permitidas en el lenguaje

        --> warn:
            producira una advertencia si se llegase a violar una regla, un ejemplo claro es cuando se 
            crea una variable, pero no se usa

        --> force-warn:
            es un nivel especial de lint. Es lo mismo que advertir en ese lint en este nivel producira 
            una advertencia, pero a diferencia del nivel del warn, el nivel de force-warn no se puede 
            anular

        --> deny:
            produce un error si se llegase a violar la regla

        --> forbid:
            es un nivel de lint especial que cumple el mismo papel para deny que force-warn para warn. 
            Es lo mismo que negar pero la gran diferencia es que este nivel no se puede anular
    */

}
