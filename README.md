# Embedded probabilistic programming in Rust

The code of this project is at [**Garbaz/probprog**](https://github.com/Garbaz/probprog)

## The Plan

Main orientation paper: [Lightweight Implementations of Probabilistic Programming Languages Via Transformational Compilation](http://proceedings.mlr.press/v15/wingate11a/wingate11a.pdf)

- [X] Implement the infrastructure for traced probabilistic procedures
- [ ] Implement a macro that allows for writing of probabilistic functions without having to interact with the tracing system directly
- [X] Implement an MCMC algorithm for general conditional inference

## Further references

- [dippl.org](http://dippl.org/)
- [probmods.org](http://probmods.org/)
- [Gen: A General-Purpose Probabilistic Programming System with Programmable Inference](https://dl.acm.org/doi/pdf/10.1145/3314221.3314642)
- [Church: a language for generative models](https://web.stanford.edu/~ngoodman/papers/churchUAI08_rev2.pdf)
