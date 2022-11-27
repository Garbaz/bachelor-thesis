# An implementation of probabilistic programming in Rust

Implement a probabilistic programming framework in Rust.

- Attribute macro implementation for convenient writing of probabilistic functions
- MCMC ?




# Agda proofs about STLC with isomorphism

(1) Preservation of typing under reduction in STLCi.

(2) Translation from STLCi to STLC and reduction commute, e.g.: 
```
(\x:A, y:B, z:C {y}) (C0) (B0) (A0)
~> 
(\x:A, y:B {y}) (B0) (A0)
~>
...
~>
B0
```
and
```
(\x:A, y:B, z:C {y}) (C0) (B0) (A0)
~>
(\x:A; y:B; z:C {y}) (A0) (B0) (C0)
~>
...
~>
B0
```

(3) ?

## Probprog Calculus

...
