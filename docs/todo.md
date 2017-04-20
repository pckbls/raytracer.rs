# Major stuff

* Do we need to operate in camera space?
  It seems that our algorithm to only render the mesh closest to the camera does not work.
  Also: Back-face culling should only work in camera space...

# Minor stuff

## Models

* Get rid of the `new()` constructor.
* Use BSP trees to allow effective polygon traversal

## Lighting

* Light intensity should depend on distance between object and light source.
* We shoot rays from the camera into the scene.
  Upon hitting an object we should shoot another ray towards the light sources to properly handle lighting and shadows.
* Create `PointLight` data structure and add function to calculate lighting for a scene point

## Debugging

* We should add another rendering mode called `Debug`?
  * Instead of lighting the mesh should be colored after the face normals' directions.
  * Or all models should have a static color
  * Light sources could be rendered as small spheres.

## Raytracing

* Generate rays with an interator for code re-use in normal/debug modes
  * http://rustbyexample.com/trait/iter.html

## Testing

* Create some rendering/integration tests in a separate directory
* Write a benchmark test for some of the performance critical functions
  * Möller-Trumbore intersection algorithm
