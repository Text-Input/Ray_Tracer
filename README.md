

**Ray Tracing Engine**

Based off of the book series "Ray Tracing: In One Weekend". 

All chapters of the first book in the series have been completed, and support for triangle-based objects has been added. 
Further, fogs of constant densities have also been implemented. 
BVH building with a surface area heuristic has also been added, as a performance speedup.

Multithreaded rendering is accomplished using Rayon. 


Python bindings have been added using PyO3. This, however, seems to have broken building
the binary on linux (or at least WSL), though the raytracing lib seems to still build properly.

The python bindings are used to create an addon for the 3D modeling software "Blender".
This addon is currently in a *very* rudimentary state(for example, all polygons are colored red), 
but its source can be found in the 'python' directory.
It, along with the .so library file (or platform equivalent) can be placed in the correct addon 
directory of Blender, and it will show up as an addon that can be selected, and then some
basic scenes can be rendered. 

For the time being, the entire project is confirmed to build correctly on Windows 10. 
