# Goals

## Version 0.1.0

* Mesh input using OFF file format
* Ray trace a simple scene
  * Reverse ray tracing without recursion (rays from camera into scene)
  * Calculate mesh intersections in 3D
  * Assign fixed colors to intersection points
  * No light sources
* Implement writable pixmap as that can be saved as PPM
* Do not use external crates

## Version 0.1.1

* Fix render output being upside down
* Implement ppm loader
* Implement raytrace test
* Clean up the mess
  * Do not simply use `.clone()` everywhere to satisfy the Rust compiler, instead try to use references whenever possible
  * ~~Split math stuff up into smaller files~~
  * ~~Eliminate all warnings~~
  * ~~Resolve all `TODO` comments~~

## Version 0.2.0

* Only render the mesh intersection that is closest to the camera
* Print progress while rendering
* Allow multiple objects to be rendered
* Enable lighting

