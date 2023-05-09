---
marp: true
theme: dracula

math: mathjax

headingDivider: 2

paginate: true
header: Embedded Probabilistic Programming in Rust
footer: Tobias Hoffmann

title: Embedded Probabilistic Programming in Rust
author: Tobias Hoffmann
---

<style>
.outer {
    /* background:blue; */
    display:flex;
    flex-flow: row wrap;
    width:100%;
    height:90%;
}

.inner {
    /* background:green; */
    width:50%;
    display:flex;
    justify-content: center;
    align-items: center;  
}

.inner2 {
    /* background:red; */
    width: 70%;
}
</style>


<style>
section {
    font-size: 30px;
}
</style>

# Embedded Probabilistic Programming <br> in Rust

<style scoped>
  section {
    /* align-items: stretch; */
    display: flex;
    flex-flow: column nowrap;
    justify-content: center;
  }
</style>

## Modelling

- The human mind creates approximate models of the world
  - How? No clue (._.')
- Science creates more precise approximate models of the world
  - How? Math!
- Models specify structure with parameters
- E.g. Newton's law of gravity
  - $F = g \cdot m$
  - Structure: $F \sim m$
  - Parameter: $g$

## Probabilistic Modelling

- Don't try to model system exactly
- Treat it as a random process
- E.g. Coin flip
  - Don't try to predict singular throw (physics is hard)
  - Describe the distribution of many throws
- We trade predictive power for tractability & generality
- → Bayesian statistics


## Probabilistic Programming

- Usually:
  - Take simple well-understood distributions
  - Combine them in some constrained way
  - E.g. Generalized linear models
- Instead:
  - Take simple well-understood distributions
  - Combine them in the most general way: Programming
  - ⇒ Probabilistic programming

## Probabilistic Programming _(cont.)_

- Take Turing-complete programming language
- Introduce probabilistic elements:
  - _sample_: Sample from distribution
  - _observe_: Observe value from distribution
- Develop general inference algorithm _(the hard part)_
- Generate samples from probabilistic program


## Example

- Is the coin we are flipping a fair coin?
- ⇒ What weight explains our observations the best?

```rs
#[prob]
fn coin(observations: Vec<bool>) -> f64 {
    let p = sample!(uniform(0., 1.));
    for o in &observations {
        observe!(bernoulli(p), o);
    }
    p
}
```


## Inference

- We have some probabilistic program
- How do we actually draw samples from it?
- Just run it?
  - Problem: How do we respect _observe_ statements?
- Solution: Markov Chain Monte Carlo


## Markov Chain Monte Carlo

- We want samples from a distribution $\pi$
- But we can only calculate it's density $\pi(x)$
- First idea: Rejection sampling
  - Propose random values, accept them with probability $\pi(x)$
  - Problem: Inefficient
- Better idea: MCMC
  - Propose random values, but not completely random
  - Look for high-probability regions in support of $\pi$

## Metropolis Hastings




## Sources & co

**Slides at:** https://github.com/Garbaz/bachelor-thesis

