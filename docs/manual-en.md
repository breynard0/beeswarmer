# How to use Beeswarmer
The following is a little manual on how to generate your very own beeswarm plot using your survey data and Beeswarmer.

## The Intuition
To gain a full understanding of how to best use a beeswarm plot in your work, you must understand the basic idea behind how they are generated. They are extremely helpful, as they can inform how a dependent variable changes in relation to as many independent variables as are present. However, they do have some drawbacks that can lead to inaccurate conclusions if used incorrectly.

Beeswarmer's beeswarm plots graph what's called Shapley values. The formal definition of the Shapley value for a feature is the average marginal contribution of said feature. Now, what does this mean?

Imagine we have Alice and Bob, who enroll together in a cooking competition. By working together, perhaps they are able to win prize money of \$50. Now, let's go back in time and have just Bob do the competition solo. In this hypothetical, he wins \$20. If we assume something called *Efficiency*, this means that we can reason that Alice joining Bob's team would bring \$30 extra of prize money. Bob alone wins \$20, Alice alone wins \$30, and their individual contributions add up to \$50.

Of course, there are flaws here. Putting aside that efficiency is often an idealistic assumption to make, maybe this cooking competition in particular specifically suits Alice's skill set. Perhaps we imagine a new team of Richard and Clyde earns \$40 at some cooking competition. If Alice joins them, then they win \$60 at the competition. In this case, Alice only brought \$20 of value by joining the team, unlike earlier.

Alice's Shapley value is what, on average, she will add to a team by joining it. With this sample size of two, the average value that Alice brings to a team in this case is $25. If we had more data, we could perhaps get a better estimate for what Alice contributes to any given team. If we had yet another team, we could make a decent approximation of how much prize money they would win with Alice joining it simply by adding her Shapley value to the amount the team wins without her.

Neat! How does this apply in any way to survey data?

Well, let's assume the scenario where you're wanting to find out the relationship between some of your survey answers. For this guide, we will use the following mock survey:

**Effects **