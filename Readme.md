# MCMC Visualizer
This project was inspired by Christos Tsanikidis' [blog post](https://tchristos.com/post/mcmc/). Christos built a Monte-Carlo Markov Chain visualizer, specifically focusing on the metropolis-hastings algorithm. His program was impressive, but he didn't go into much detail about how he made it work. So, I decided to take matters into my own hands and give it a shot.

My visualizer is all utilizes two Gaussian distributions that represent the X and Y dimensions. You can tweak the means and standard deviations of both distributions to see how they affect the results.

I hope that by sharing my version of the visualizer, I can help others understand thisalgorithm and encourage them to dive into their own experiments. It's all about learning and building on the work of others, like Christos.

## How to run

1. Simply clone the repo.  

```$: git clone https://github.com/TylerMorton/MCMC-Visualizer.git``` 

2. Go into the MCMC-Visualizer directory  

```$: cd MCMC-Visualizer``` 

3. and run the project.  

```$: cargo run```

