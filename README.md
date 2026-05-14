# Learning Procedural Macros in Rust

In order to prevent repetitive boilerplate coding I decided to experiment with
procedural macros to write out a lot of things. Applying proc macros takes the
form of one of the following available options:

1. Derive macros
2. Attribute macros
3. Functional macros

I will be using the following dependencies (i.e.
`syn`,`proc-macro2`,`quote`,`darling` ) to simplify some of the parsing ,codegen
and token transformation when implementing the macros.
