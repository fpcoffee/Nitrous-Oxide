# Nitro Examples

## OCaml Value Comparison in `ocaml_compare.nox`

[The OCaml runtime system is written in C](https://github.com/ocaml/ocaml/blob/trunk/byterun/).
For an earlier version of OCaml, the `ocaml_compare` function is a C function
that compares two OCaml values and returns `ocaml_true` or `ocaml_false`.

The OCaml runtime system must be able to read and write OCaml values.
Type and value definitions for doing so are spread out across the C runtime system.
In contrast, `ocaml_compare.nox` encodes the OCaml runtime values in one concise type declaration.

The `ocaml_compare` function is implemented by the Nitro function `compare_val`. 
It is fast like the C implementation, but it also offers advanced type-safety.
