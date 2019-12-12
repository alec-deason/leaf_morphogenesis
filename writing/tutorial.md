# Leaf Morphology

## Intro

I want to make biologically plausible images of the leaves of imaginary tree species. If that sounds exciting to you, then stick tight because that's what we're going to do. Procedural generation of trees is a relatively common thing but it typically involves decorating the trees with leaves that are either designed by a human artist and only placed by the procedural system or are very simple. I want leaves that have interesting diversity and are recongizable as belonging to a particular species which is itself procedurally generated. In order to do that I need a system for procedurally generating non-trivial leaves. I'm going to do it by implementing a simulation of leaf morphogenesis based on [this paper](http://algorithmicbotany.org/papers/leaves.nph.2017.pdf).

### What is morphogenesis
Morphogenesis is the creation of shape. It refers to the processes by which shape emerges from some growth process, not always but usually biological. The way in which an undifferentiated ball of embryonic cells grows into the very particular shape of a kitten is morphogenesis. In this project we will be looking at the morphogenetic process by which a bulb of meristem (roughly, the plant equivelent of stem tissue in animals) grows into a leaf that has a complex shape specific to the plant's species. We'll take that process and turn in into a system for procedurally generating species specific leaves.

### What is procedural generation
In a typical artistic process an artist will go through some process, whether that be preparing and manipulating clay or painting pixels in a digital image, with the goal of producing some final outcome, a work of art. In procedural generation the artist, rather than engaging in a process to design a final artistic object, designs the process by which artistic objects can be created. That movement from the object as the final project to the process as the final product is the key characteristic of procedural generation and gives it both it's strengths and weaknesses. The strength of the approach lies in it's ability to work with levels of detail and at scales that human artists would struggle with. A human couldn't individually sculpt every leaf in a forest in an entire lifetime but a procedural system could do it in seconds. Even an artist with a deep knowledge of biology would struggle to keep in mind the effects of hormone trasport through a plant's tissues as they sketched, but those kinds of details are well within the capabilities of a procedural system. On the flip side it is very difficult for a procedural system to have even a crude awareness of the aesthetic impact of the work it produces where a human artist can judge and react emotionally to every detail of a work as it is produced. The procedural systems must be painstakingly designed to limit their ability to make non-aesthetic choices which often also limits the overall diversity and interest of their output. This system will play to the strength of procedural generation while stearing away from it's weaknesses by simulating in some detail a system which in the real world reliably produces objects which humans find beautiful and interesting, the leaves of plants.

## Expectations/Outline

I will begin by discussing the model the paper proposes and then move to implementing it as a Rust library suitable for use in games or art projects. I'm using Rust both because it's a language well adapted to these kinds of projects and because I intend to use the library myself for a project which is already written in Rust. I'll be focusing as much on the process of writing a casual (ie. artistic rather than scientific) adaptation of a scientific model because I... I don't know why. Just because.

The paper does include [code](http://algorithmicbotany.org/papers/Leaves2017/index.html) for the author's own implementation. That's super awesome and more scientists should do it. I'm going to ignore the code for now though. I want to implement it myself because I think it will be fun. Also the code from the paper is in C++ that's specific to a [modeling tool](http://www.algorithmicbotany.org/virtual_laboratory/) which is neat but not something I can embed into my project so I need to be independent of it. There may be things that I can't figure out from the description in the paper along in which case I'll be super glad to have that code to look at.

## Overview of the paper

What are leaves? If you look at a leaf it seems to have two main parts. There's a branching structure of veins which start at the main stem (the "petiole" if you want to get fancy) and spread out to the edges of the leaf. Then there's the tissue in between the veins (the lamina) which forms a flat sheet (I know it's not always flat but the paper pretends it is and I'm going to as well (though 3d buckling or the shapes of succulent leaves would be a super neat enhancement to figure out in the future). There's a lot else going on but that's enough to start with, a tree of veins embeded in a sheet of tissue. Leaves start out as little bulbs of tissue on a branch, I'm actually not sure what early growth is like at all but eventually you get the begining of that vein structure and after that the veins grow longer and occasionally branch and the lamina grows along with it such that there is always some lamina out to the very tips of the veins. There doesn't seem to be concensous about exactly how or why the parts grow the way they do but that's not super imortant for this high level model, we just want to get the crude process about right.

The paper's model breaks the leaf down into three distinct parts. The first is a triangle mesh which represents the lamina. The other two are the veins and the outermost edge of the leaf which turns out to be a key component of their model. Both are embeded in the lamina mesh.

***
**What is a triangle mesh**
Triangle meshes are a common structure in grahics software or other places where there's a need to represent 2d or 3d shapes but if you haven't worked in those domains they might be unfamiliar so I'll introduce them here. We're likely to be using them a lot.

_illustration of a triangle mesh_

A triangle mesh consists of two things: points in space (two dimensional space in our case) and connections between those points such that if you drew a line between each connected point you'd end up with a bunch of triangles, each sharing edges with three neighboors unless it's on the edge of the mesh.

_example of a triangle mesh structure in Rust_

***

The leaf margin is the outermost edge of the lamina mesh and it's vertices are annotated with morphogens (discussed below). The veins run along edges of the lamina mesh and are all internal except the base of thepietole and the tip of each vein which end at one of the margin vertices. The vertices where a joins the margin are important to the model and are called convergence points in the paper. The rest of the mesh is unlabeled and mostly passive in the model but will be important when we render the leaf models into images.

My impulse is to store all that in a single mesh structure though it may end up being convenient to have the veins as a seperate component. We'll see what operations we really need to do on the structures, that will guide how we represent them.

The morphogens that exist in the leaf margin represent the present of growth hormones or other processes (it's vague) in the plant that control some aspects of growth. Right now all we need to know that there are some number of these morphogens which can exists at a vertex. I think they're boolean, just presence or absence, but we might need a number to store an amount.

So the model's data lives in a labeled mesh, what are the processes that will use that structure. There are three main process: vein elongation, vein division and lamina growth.

Vein growth is fairly simple. The veins consist of segments end to end. They can grow both by adding more segments at the tip or by elongating existing segments. Various aspects of the model effect which kind of growth happens at any given time, more below. When a segment elongates it pushes segments beyond it outward. It also pulls the margin near it's tip outward with it.

Vein division is controlled by the length of the margin between the tips of two existing veins. Once that length exceeds some threshold a new vein tip is added at the midpoint of that margin section and connected to some point on an existing vein. The paper proposes several mechanisms for choosing the attachment point and I honestly can't remember which one they actually use without going back and looking. Basically you connect it to the closest point on an existing vein.

Lamina growth is conceptually simple but harder to implement and I don't actually understand how to do it yet, that'll be a thing to figure out and may end up being the hardest part of this project. The basic idea is that the lamina grows continually and uniformly in all directions which creates stresses within the leaf that cause it to distort. If we were doing 3d this is where buckling would come in. In the 2d model I think it only effects how deep the curves between vein tips end up being but it may change vein orientation as well, I'm not sure.. So, a lot more on this later.

There are at least two important parameters which effect these growth processes. One is the presence of morphogens. These represent growth regulation systems in the leaf, maybe growth hormonse but maybe other things. I don't believe the paper suggests that their morphogen model reproduces an actual mechanism in real plants, but it approximates the effects of those systems on leaf shape. In the model morphogens are boolean flags attached to the vertices at the leaf margin. They seem to be able to produce arbitrary effects on the leaf. In the paper I've so far seen examples of morphogens that control the margin length threshold for vein branching, either making it shorter or longer, possibly supressing branching entirly in the region of the leaf they effect. There also seems to be one that supresses lamina growth. There may be others or I may not fully understand how those examples actually work. Morphogens are distributed over the leaf margin when the model is initialized and then basically just stick to vertices as they move due to growth. When more vertices are added to the margin they seem to inherit the morphogens from their neighboors. Something more complex happens when a new vein tip is added, I'm not totally sure how that works yet but it seems to split contiguous regions of morphogens in half. The other important parameter controls vein segment elongation. In some species elongation only happens near the base of the leaf and that's important in forming certain leaf shapes. I believe that even in those cases new segments still grow at the tip of the veins.

Ok. I think that covers the broad strokes of the model. There are a lot of details to fill in but we've got enough to start sketching some code.

## Basic Structures


