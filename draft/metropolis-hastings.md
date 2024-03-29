# The Metrpolis Hastings Algorithm

While there are many MCMC algorithms, with differing conditions for application and advantages/disadvantages in terms of implementation complexity and performance, we will focus here on a principally rather simple algorithm based upon seminal work by N. Metropolis et al and W. K. Hastings, the "Metropolis-Hastings algorithm" (MH).

Like with all MCMC algorithms, the problem we are trying to solve with MH is the need to obtain samples from some distribution of interest $\pi$, the _target_, and the general method is to explore the space $\mathbb{X}$ of possible samples from $\pi$, its _support_, via a Markov chain that iteratively steps through this space. How these steps are taken is the primary distinction between different MCMC methods.

As with many general MCMC algorithms, to apply MH to sampling from some distribution $\pi$, we need to pick a (usually, relative to $\pi$, very simple) distribution $q$, a _kernel_, with the same support as $\pi$. This kernel defines our exploration of the support $\mathbb{X}$. Specifically, at step $X_t = x_t$ in our Markov chain we use $q$ to choose a possible next step $\hat{X}_{t+1} \sim q[x_t]$. So $q$ can, and usually will, depend on the current position $x_t$ in $\mathbb{X}$.

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

For any $\alpha$ between $0$ and $1$, we will sometimes take the proposed step and sometimes will not, depending on what value is drawn for $U$. So we can still step "back down" to values that are less likely than the current value, but this is increasingly unlikely the smaller the ratio between the likelyhood under $\pi$ of the value after a proposed step and the current value. As a result, we will generally tend towards sampling from regions of high probability under $\pi$, while also in the long run exploring regions of lower (positive) probability.

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

