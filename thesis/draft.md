## Abstract

The paradigm of probabilistic programming allows the expression of computationally arbitrary generative probabilistic models and provides model-independent generic inference techniques over them. We present here an overview over the theory behind a general class of commonly employed inference algorithms, Markov chain Monte Carlo (MCMC) methods, the working details of one particular general MCMC algorithm, the Metropolis Hastings algorithm, and a concrete implementation of probabilistic programming embedded into the imperative programming language Rust via the use of macros.

## Introduction

For various systems across many fields of interest, randomness can be useful in developing tractable models by abstracting over the dynamics of some complex process, such as for example the physics of a coin flying through the air being abstracted over with a simple Bernoulli distribution to model it's behaviour of either landing on heads or tails. Unlike different methods of abstraction, like approximate simulation, if constructed acccurately, a distribution can precisely capture a part of the behaviour of a system without having to fully reproduce it's internal mechanics. Specifically, while a single draw from a distribution will not necessarily match some observed behaviour, the characteristics of a total of draws will approach the characteristics of an equal number of independent observations.

While there are many examples of the behaviour of principally rather complex systems being well approximated by mathematically simple distributions, like for example the distribution of beans in a "bean machine" following a normal distribution, most systems of practical interest do not give rise to such easily describable distributions. Rather, to develop a stochastic abstraction over the mechanics of many systems requires the arbitrarily complex composition of simple distributions, to a point that it can become difficult or even impossible to analytically answer questions of interest about the resulting total distribution. For example, while it might be easy to directly calculate the expected value or variance of some well understood distributions and even of simple combinations of distributions, like a linear combination of real-valued distributions, for many complex distributions this no longer is directly possible.

However, many characteristics about a complex distribution can still be approximated by a general category of methods called "Monte Carlo methods", which rather than analytically working on the structure of a distribution itself to obtain results instead compute numerical approximations by taking random samples from the distribution, just as one would statistically analyze observations from any process.

While it no longer is required for our target distribution to be fully analytically comprehensible to apply a Monte Carlo method of approximation, it is crucially still necessary to have the means to obtain a large number of samples from our distribution, which itself can already be rather difficult to develop for many complex distributions. Though while it might be difficult to generate samples, often times it is much easier to at least compute the probability of some given value having been drawn from a certain distribution.

To still obtain samples under such relaxed conditions, where the only practically computable function is getting the probability of some value being drawn from the distribution, a class of algorithms collectively called "Markov Chain Monte Carlo methods" (MCMC) have been developed. The principle operation of these methods is to iteratively explore the space of values that might be drawn from a distribution by taking repeated randomized steps through it and for each step deciding whether to take it or to revert back to the previous value. If the method of proposing steps to take and deciding whether to accept or reject them is chosen correctly, the resulting sequence of values will converge to a distribution which matches the target distribution. This way we can generate samples from a distribution without having to actually be able to directly draw such samples, allowing us to use Monte Carlo methods to get approximations for characteristics of interest about our distribution.

While MCMC methods are widely used in various fields of application, such as physics, economics and many other endavours of both academia and industry, and routinely applied to potentially very high-dimensional and complex stochastic models, these usually are still drawn from a computationally significantly constrained class, such as generalized linear models. We will here consider a much wider class of models, that of probabilistic programs.

For our purposes, we will consider a probabilistic program to be a function in a Turing-complete imperative programming language which can contain two additional elements besides the language's regular semantics: Instances of sampling from predefined primitive distributions or other probabilistic programs, and statements of observing some value from some primitive distribution.

Introducing randomness into an otherwise deterministic program is itself not much of a significant change to the execution model of a programming language with persistent state, with most everyday programming languages having some readily provided means of getting random, or at the least pseudo-random, values for various practically relevant distributions. So without any change to execution, we can model many computationally complex distributions by simple writing some function which utilizes such primitive distributions. Drawing samples from the composite distribution then corresponding to simply executing this function.

However, in many practical tasks, we might not simply want to obtain some value from a distribution and run with it, but rather wish to express a constraint on such a value. For a very common example, we might have some complex stochastic model, written as a probabilistic program, and some empirical observations of data from some real-life system we wish to understand, and want to know which instances of of our model reproduce most closely this data.

One straightforward possibility would be to simply run our program many times and reject samples which do not fit the observed data. This is called "rejection sampling" and is for relatively simple models with a finite and relatively small space of possible output values a feasible method, if however perhaps computationally somewhat wasteful. But for any more complex model, and especially for distributions over a non-finite domain, this method is infeasible. Rather, an approach which more efficiently explores the possible executions of a probabilistic program is necessary.

To solve this problem with MCMC methods, three considerations have to be made: What is the space over which we define our distribution which we will explore? How will we be able to compute the probability of some sample coming from our distribution? How will we efficiently step through this space while also fulfilling the constraints necessary for our algorithm to efficiently sample from the target distribution?

While there any many answers to the final of these three questions, the first two have relatively straightforward solutions. The space we will explore with MCMC is the space of possible executions of a probabilistic program at hand, more specifically the space of traces of draws from primitive distributions throughout it. And to calculate the probability of some particular trace being a possible execution of the program, we simply run the program and accumulate the probabilities of the individual draws and observations of values from primitive distributions as we encounter them during execution.

## Probability Theory




## Markov Chains

A Markov chain is a random sequence of values from some space. In contrast to any other random sequence, a Markov chain is characterized the particular property that the distribution determining every value in the chain is dependent on exactly the directly preceding value. If considered as a series of steps through some space of values, this means that every step taken is only based on the current location in the space, and entirely agnostic to how we got there. Or perhaps more philosophically, in a Markov chain, the future depends on the present, but not the past.

In mathematical terms, we define a Markov chain as a series $(x_i)_{i \in \mathbb{N}}$ in some space $\mathbb{X}$. 

The advantage of such a forgetful sequence is that it is possible to prove general propositions about it's behaviour, such as whether or not it will converge towards a stable distribution and what this distribution will look like. But by each value depending on the previous, much more interesting behaviour can emerge than with a sequence of entirely independently drawn samples.

While Markov chains are often times conceptualized as a directed graph consisting of nodes representing the possible states the chain can jump between at each step, and weighted edges representing possible transitions between states with their relative probability, for our purposes it is more useful to consider a Markov chain as a series of time-discrete steps in some, potentially high-dimensional and highly structured, abstract space of values.

At each point in time therefore we are at some value and make some decision as to which value to jump to next. The choices of these incremental decisions will accumulate to determine the properties of the resulting sequence of values.

As a tangible example, one might consider the one dimensional array of integer numbers, where at each step we throw a coin to determine whether to take a step in the negative direction or a step in the positive direction. Or for another example perhaps a two dimensional space of real-numbered pairs and a walk throughout it where at each step a value from a two dimensional normal distribution is draw to determine in what direction and how far to jump next. In either case, the result will be a random sequence of values characterized by both the space they came from and the random distribution by which we stepped through this space.


## Metropolis Hastings

While there are many MCMC algorithms, with differing conditions for application and advantages/disadvantages in terms of implementation complexity and performance, we will focus here on a principally rather simple algorithm based upon seminal work by N. Metropolis et al and W. K. Hastings, the "Metropolis-Hastings algorithm" (MH).

As with any MCMC algorithm, the problem we are trying to solve with MH is the need to obtain representative samples from some distribution of interest $\pi$, the _target_, and the general method to do so is to explore the space $\mathbb{X}$ of possible samples from $\pi$, it's _support_, via the iterative development of a Markov chain through this space. How these steps are taken is the primary distinction between different MCMC methods.

To apply MH to sampling from some distribution $\pi$, we need to pick a (usually, relative to $\pi$, very simple) distribution $q$, a _kernel_, with the same support as $\pi$. This kernel defines our exploration of the support $\mathbb{X}$. Specifically, at step $X_t = x_t$ in our Markov chain we use $q$ to choose a possible next step $\hat{X}_{t+1} \sim q[x_t]$. So $q$ can, and usually will, depend on the current position $x_t$ in $\mathbb{X}$.

If our $\pi$ were for example defined over $\mathbb{X} = \mathbb{R}^2$, a possible candidate for $q$ would be a two-dimensional normal distribution centered around $x_t$. So at each step in Markov chain exploration of $\mathbb{X}$, we would draw $\hat{X}_{t+1} \sim \mathcal{N}[\mu = x_t, \sigma^2 = s^2]$ (with $s^2$ being in principle an arbitrary, but in practice a very important parameter to tune).

If we were to simply always take this proposed step $\hat{x}_{t+1}$ drawn from $q$ then the result would be a random walk through $\mathbb{X}$ entirely independent from our target distribution $\pi$. With the goal of course being to obtain a series of samples from $\pi$, this would of course not be of much use.

The second part to every sampling step in the MH algorithm is to decide whether to take the proposed step drawn from $q$, $x_{t+1} = \hat{x}_{t+1}$, or whether to remain at the current position in $\mathbb{X}$, $x_{t+1} = x_{t}$. This decision is once more done randomly and based upon $\pi$, $q$ and the proposal $\hat{x}_{t+1}$:

$$
\begin{align*}
X_{t+1} & = \begin{cases}
    \hat{x}_{t+1} & \text{ if } U \le \alpha \\
            x_{t} & \text{ otherwise }
            \end{cases} \\

\alpha & = \frac{\pi(\hat{x}_{t+1})}{\pi(x_t)} \frac{q(x_t | \hat{x}_{t+1})}{q(\hat{x}_{t+1} | x_t)} \\
U & \sim \mathcal{U}[0,1]
\end{align*}
$$

The value $\alpha$ defined here is called the _acceptance ratio_. If $\alpha$ is equal to $0$, which would be the case if $\pi(\hat{x}_{t+1}) = 0$, then we will definitely remain in place, since $P(U \le 0) = 0$. So our Markov chain will never step to values which are impossible to be a sample from $\pi$.

If on the other hand $\alpha$ is greater than or equal to $1$, which would be the case if $\pi(\hat{x}_{t+1}) \ge {\pi(x_t)}$ (assuming a symetric kernel $q$ for which the second right fraction cancels out), then we will definitely take the proposed step, since $P(U \le 1) = 1$. So our Markov chain will always step to values that are more likely under $\pi$ than the current value.

For any $\alpha$ between $0$ and $1$, we will sometimes take the proposed step and sometimes will not, depending on what value is drawn for $U$. So we can still step "back down" to values that are less likely than the current value, but this is increasingly unlikely the smaller the ratio between the likelyhood under $\pi$ of the value after a proposed step and the current value. As a result, we will generally tend towards sampling from regions of high likelihood under $\pi$, while also in the long run exploring regions of lower (positive) likelihood.

And under some assumptions about the target distribution $\pi$ and the kernel $q$ it is possible to rigorously prove that, at least in the limit, the Markov chain of samples generated as such will converge to being a sequence of (dependent) samples from $\pi$.

So in total the complete Metropolis-Hastings algorithm looks as follows:

```
Repeat forever:
    (1) Sample X_proposal from q(X_current)
    (2) Calculate acceptance ratio A based upon X_proposal and X_current
    (3) Sample U between 0 and 1 uniformly
    (4) If U <= A, then X_current := X_proposal
```

One in practice highly relevant property to note about the definition of MH is the fact that we only ever need to compute a ratio $\frac{\pi(x)}{\pi(y)}$ between two results of the probability density function $\pi(x)$. This means that we do not actually have to be able to compute $\pi(x)$ directly, but rather that it is sufficient to be able to compute some proportional function $\tilde{\pi}(x) \propto \pi(x)$, since $\frac{\tilde{\pi}(x)}{\tilde{\pi}(y)} = \frac{\pi(x)}{\pi(y)}$.

As a very common practical example, say we would like to generate samples from some posterior distribution:

$$
\pi(x) = p(x | w) = \frac{p(w | x) p(x)}{p(w)}
$$

While calculating $p(w | x)$ and $p(x)$ might be straightforward, often times directly getting a value for $p(w)$ is rather difficult or impossible. Usually one would have to compute it from the other two quantities as $p(w) = \int p(w | x) p(x) dx$, which can be rather costly.

With MH however, since $p(w)$ does not depend on $x$ and we only need to know $\pi(x)$ up to a proportionality constant, we can simply define $\tilde{\pi}(x) = p(w) \pi(x) = p(w | x) p(x)$ and calculate our acceptance ratio $\alpha$ with $\tilde{\pi}$ rather than $\pi$, sidestepping the need to evaluate any costly integrals.


## Probabilistic Programs

A probabilistic program for the purposes of our implementation here is syntactically simply an ordinary function in an (in our case imperative) programming language. This function can contain any code constructs that are part of the host language, including potentially troublesome things like conditionals, loops and recursion. However, a probabilistic program can, as opposed to an ordinary function, contain two additional syntax elements: "sample" expressions and "observe" statements.

Besides the syntactic difference to an ordinary function, the execution of a probabilistic program also differs in a significant way. In addition to running the code as normal, during execution track is kept of what distributions are sampled from with what parameters, what values are drawn and how probable drawing these values was, and most importantly, the total probability of the particular execution happening.

### Sample Expression

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

### Observe Statement


The other special kind of expression we can use in a probabilistic program is an `observe` statement. It allows us to state that, at this position in the code, and therefore possibly dependent on values computed so far, we "observe" some value from some distribution. We essentially say that "we know that this value is the result of sampling from this distribution", which might or might not be likely.

Neither the value we are observing, nor any parameters to the distribution have to be constant. They can result from any arbitrary combination of ordinary and probabilistic computations. However, we can not observe values from a distribution defined by another probabilistic program, only from primitive distributions.

Observing a value from a distribution does not have any direct effect on the execution of our program. If we were to take a probabilistic program and remove all observe statements, it would still principally run the same way. However, observe statements greatly affect the way samples are drawn from the probabilistic program, specifically the total probability of the current execution. If for example, we were to observe a value of $2$ from a uniform distribution $\mathcal{U}(0,1)$, which of course is not possible, i.e. has a probability of zero, then the total probability of the execution would also be zero. In short, observe statements allow us to, smoothly, constrain what instances of our model are likely or even possible.

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


## Trace Space

## Embedding into Rust
