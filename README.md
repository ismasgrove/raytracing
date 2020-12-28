# Raytracer (Technically a path tracer as of this moment)

This is an exercise on raytracing using the resource 'Raytracing in a Weekend' book series from Peter Shirley, who somewhat of a legend in the field. I decided to do it in Rust so as to not feel as if I'm just copying code ong things without understanding them, which happens when you're doing something from a book or a tutorial. I added the parallel element (using rayon) for the same reason, and will generally try to add as many personal touches as I can.

Book 1 cover reproduced in 44 minutes on my i7-6500U as of last commit.

![Book 1 cover](images/book1_cover.jpg?raw=true)

How to run:

```shell
cargo run image_name.ppm
```

**TODO** (implementation-based, as implementing the rest of the books' features goes without saying):

1. Investigate GPU-acceleration.
2. Use 'image' crate to generate different formats on demand, from the command line.
3. An indication on rendering progress during runtime.
  