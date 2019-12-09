#Other models/background

Growth at the leaf magin is important. Growth in different sections causes stress, buckling and stretching that help create over all leaf shape.

Veination. Webbing growth is oriented along veins. Veins form by self reenforcing auxin flow? Some theories suggest that vein growth provides the force for expansion which is resisted by the webbing.

#This Model

##Three data structures:
 - the leaf margin is a series of "sample points" which together form a polygon that defines the outer shape of the leaf. Each point stores both it's location and the presence/amount of several morphogens which effect it's growth and whether it is a "convergence point" which means that it is attached to the tip of a vein.
 - the vein structure. A simple tree structure with (probably) binary branching and straight segments.
 - a triangle mesh representing he webbing. They list this as a fundamental structure but it seems to me like it could be derived from the others.

##Growth

Growth is driven by elongation of veins, expansion of leaf margin and formation of new veins to support sections of margin. Vein elongation only occures at the base of the leaf. I'm a bit confused about how that works in practice, especially since it's so different from SAM driven growth which comes from the tips of branches. Growing veins stretch the margin (possibly also impacted by the physical constraint of the webbing. Margin growth is impacted by the presence of morphogens. When a section of margin becomes too long a new vein is grown back along the margin's normal towards the nearest existing vein. This may only happen in the presence of a particular morphogen. Sample points on the margin move as the vein near them moves. They are projected onto the nearest vein and move along the same vector that that point on that vein moves.

Oh, it says that vein growth involves both elogation of segments (this seems to be the part that only happens at the base) and addition of new fixed size segments at the tip (which presumable happens in the distal parts of the leaves as well).

They also say that uniform growth of the webbing is part of the model. I'm not sure yet if that is modeled explicitly or if it's subsumed into the behavior of the sample points on the margin.


Ok, later they clarify that the thing with vein growth is that it is a function (potentially a complex function) of distance along the tree to the base. So it can be uniform (distance makes no differenc) or basipital (growth rate decreases with distance).


Margin growth is driven by three processes: Movement tied to the associated vein. "Propegation" perpendicular to the surface normal. I'm not sure what "propogation" means here, maybe the addition of new points? Finally minimization of stretching, some sort of optimization process which they refer to as "geometric fairing". There are details on this in the supplement: algorithmicbotany.org/papers/leaves.nph.2017.SI.pdf


Development of new convergence points can be controlled by several parameters. The basic thing is distance to the nearest prexisting convergence point. Formation can be suppressed (possibly also enhanced) by the presence or absence of a morphogen, distance from the base of the leaf or over all leaf age. New veins connect a new convergence point to an existing vein in such a way to minimize resistance to flow to the base of the leaf (in practice just minimize distance). Also they constrain the angle at which the vein will meet the margin to some value for the sake of looking reasonable.


