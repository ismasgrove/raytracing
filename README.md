# YARIBOPSS

This is an exercise on raytracing using the resource 'Raytracing in a Weekend' book series from Peter Shirley, who somewhat of a legend in the field. I decided to do it in Rust so as to not feel as if I'm just copying code ong things without understanding them, which happens when you're doing something from a book or a tutorial. I added the parallel element (using rayon) for the same reason (in addition to my aging laptop making sequential debugging awfully time consuming), and will generally try to add as many personal touches as I can.

![Random scene](images/book1_cover.jpg?raw=true)

![Cornell smoke](images/cornell_smoke.jpg?raw=true)

[!Book 2: Final scene](images/final_scene.jpg?raw=true)

How to run:

```shell
cargo run output_image_name.ppm
```

**TODO** (implementation-based, as implementing the rest of the books' features goes without saying):

1. Investigate GPU-acceleration.
2. Use 'image' crate to generate different formats on demand, from the command line, without resorting to image conversion.
3. ~~An indication on rendering progress during runtime.~~ done
4. Figure out the proper way to I imports, mods and crates in Rust as it looks rather unseemly.
5. Include more shapes (Currently eyeing teapots, infinite planes, non-axis aligned planes and boxes)
