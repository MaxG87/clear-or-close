# clear-or-close

This project allows you to check whether an referendum was close in a
statistical sense. Currently it only supports referendums with exactly two
choices.

The project was born when media coverage claimed that the Brexit referendum was
"close". Since I was preparing a course about statistical testing I took the
chance to present a Monte Carlo test with recent relevance.

## Statistical Model

The performed statistical test is based on MonteCarlo testing. Here, $N$
samples of a suitable model are drawn and checked wheter they are at least as
extreme than the referendum in question. 

Let $N$ be the number of entitled voters, $0 < M <= N$ be the number of actual
voters and blue and red the options to vote for.
the number of voters that choose option 2.

The statistical model is as follows. It assumes that every voter is decided on
exactly one of the options. Who actually attends the referendum is up to
chance. However, there is no correlation between the choice of a voter and
whether she will attend the referendum.

Thus, the outcome of the referendum, according to the statistical model
sketched above, can be modelled by drawing $M$ balls from an urn with $\frac 12
N$ red and $\frac 12 N$ blue balls. This is a special case of the
Hypergeometrice Distribution.

A sampled result is considered to be _at least as extreme_ than the result that
is tested for significance if one option got at least as many votes than the
option with a majority share of votes. Thus, when the actual poll resulted in
$(13, 37)$ and the sampling resulted in $(37, 13)$, the sampled result is
considered to be at least as extreme.


## Sampling from the Statistical Model

The implementation to sample the Hypergeometric Distribution closely mimicks
drawing balls from an urn. It maintains a vector `counts` which tells how many
examples for each option remain and samples options with probability
proportional to these counts. The sampling repeats this process until $M$
elements were drawn.

The runtime is prohibitive for large values of $M$.

## Todo:

* Find clever approximations so sampling gets faster. An $O(k)$ way to sample
  from a k-dimensional Hypergeometric dimension would be awesome.
  HINT: https://wikis.hu-berlin.de/mmstat/Approximation_von_Verteilungen#Approximation_durch_die_Normalverteilung_2

* Write tests!

* Add command line parser so no recompilation is required for different values.

* Add support for general elections. Needs a better $H_0$ and a better measure
  of _extremness_, though.

* Generate confidence intervals to indicate how reliable the results are.


## Result

Usually election results are significant according to the statistical model
described above. Even for 2500 eligible voters the Brexit result would be
significant.
