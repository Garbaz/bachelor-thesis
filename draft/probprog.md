# Probabilistic Program

A probabilistic program for the purposes of our implementation here is syntactically simply an ordinary function in an (in our case imperative) programming language. This function can contain any code constructs that are part of the host language, including potentially troublesome things like conditionals, loops and recursion. However, a probabilistic program can, as opposed to an ordinary function, contain two additional syntax elements: "sample" expressions and "observe" statements.

Besides the syntactic difference to an ordinary function, the execution of a probabilistic program also differs in a significant way. In addition to running the code as normal, during execution track is kept of what distributions are sampled from with what parameters, what values are drawn and how probable drawing these values was, and most importantly, the total probability of the particular execution happening.

## `sample`

A sample expression is semantically rather simple, it allows us to sample a value from some distribution, be it a primitve distribution provided by our implementation, or a distribution defined by another probabilistic program. The resulting value of a sample expression is in every regard no different than as if it were simply an ordinary function call.

```rs
/// Sampling from a primitve distribution and using recursion
#[prob]
fn example1(p : f64) -> u64 {
    let c = 0;

    while sample!(bernoulli(p)) {
        c += 1;
    }

    c
}

/// Sampling from another probabilistic program and using conditionals & recursion
#[prob]
fn example2(n : u64) -> u64 {
    if sample!(example1(1./(n as f64))) >= n {
        0
    } else {
        1 + sample!(example2(n))
    }
}
```

## `observe`


The other special kind of expression we can use in a probabilistic program is an `observe` statement. It allows us to state that, at this position in the code, and therefore possibly dependent on values computed so far, we "observe" some value from some distribution. We essentially say that "we know that this value is the result of sampling from this distribution", which might or might not be likely.

Neither the value we are observing, nor any parameters to the distribution have to be constant. They can result from any arbitrary combination of ordinary and probabilistic computations. However, we can not observe values from a distribution defined by another probabilistic program, only from primitive distributions.

Observing a value from a distribution does not have any direct effect on the execution of our program. If we were to take a probabilistic program and remove all observe statements, it would still principally run the same way. However, observe statements greatly affect the way samples are drawn from the probabilistic program, specifically the total probability of the current execution. If for example, we were to observe a value of $2$ from a uniform distribution $\mathcal{U}(0,1)$, which of course is not possible, i.e. has a probability of zero, then the total probability of the execution would also be zero.

```rs
/// What parameter `p` for a bernoulli distribution explains our observed results best?
#[prob]
fn example3(obs : [bool]) -> f64 {
    let p = sample!(uniform(0., 1.));

    for o in obs {
        observe!(o, bernoulli(p));
    }

    p
}

/// What might have been the start position of a random walk, given we know the end position and the number of steps?
#[prob]
fn example4(steps : u64, end_pos : f64) -> f64 {
    let start_pos = sample!(uniform(-10.,10.));
    
    let mut pos = start_pos;
    for _ in 0..steps {
        pos += sample!(normal(0.,0.5));
    }
    
    observe!(end_pos, normal(0.,1.));

    start_pos;
}
```
