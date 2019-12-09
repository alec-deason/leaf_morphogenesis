# Intro

Plants are awesome. Simulation is awesome. Morphogenesis is awesome.

The tree model, for some game project. Needs leaves.

The paper: http://algorithmicbotany.org/papers/leaves2017.html

## Tutorial expectations

Probably several seperate essays. Include some of the mistakes involved, not just the finished product. Commits or releases in the repo associated with different sections of the tutorial.

## Technical expectations

Rust. Probably Piet. Nalgebra or maybe vek. Non-interactive. Developed as open source. Input: species description as a set of parameters. Output: Texture image

# Skim the paper

What are they modeling exactly? What are the basic structures/objects?

# Sketch the software architecture

What will we store? What does the loop look like? What are the parameters?

# Write a basic flow

Load parameters (serde). Run an empty simulation loop. Output a blank image from non-existent intermediate data.

# First pass at the simulation

Implement enough of the simulation loop that we have a pretty good idea of what the intermediate data is going to look like.

# first pass at the renderer

Given that intermediate representation write a version of the renderer that can dump at least a silhouette of the leaf along with major structures like veins. Now we can visually debug.

# Second pass at sim

Polish things?

# Second pass at the renderer

Make it reasonably pretty

# Tune parameters

Spend some time fiddling with the parameters to understand the space.

# Refactor

Get things shaped such that the system makes sense as a library.

# Testing

Hit reasonable coverage if it isn't there already

# Package

Turn it into a crate and get it on the package index.

# Conclusion

Likely extensions. Use cases.
