# YARIBOPSS

This is an exercise on raytracing using the resource 'Raytracing in a Weekend' book series from Peter Shirley, who's somewhat of a legend in the field. I decided to do it in Rust so as to not feel as if I'm just copying code without understanding it, which happens when you're doing something from a book or a tutorial. I added the parallel element (using rayon) for the same reason (in addition to my aging laptop making sequential debugging awfully time consuming), and will generally try to add as many personal touches as I can.

![Random scene](images/book1_cover.jpg?raw=true)

![Cornell smoke](images/cornell_smoke.jpg?raw=true)

![Book 2: Final scene](images/final_scene.jpg?raw=true)

![XZPyramid](images/rotate_pyramid.jpg?raw=true)

```shell
cargo run image_name.jpg
cargo run image_name.jpeg
cargo run image_name.tiff
cargo run image_name.ico
cargo run image_name.bmp
cargo run image_name.png
```

**TODO** (implementation-based, as implementing the rest of the books' features goes without saying):

1. Investigate GPU-acceleration.
2. ~~Use 'image' crate to generate different formats on demand, from the command line, without resorting to image conversion.~~ done
3. ~~An indication on rendering progress during runtime.~~ done
4. Figure out the proper way to use imports, mods and crates in Rust as it looks rather unseemly.
5. Include more shapes (Currently eyeing teapots, infinite planes, non-axis aligned planes and boxes)

Resources:

1. RTIOW (obviously)
2. Scratchapixel (amazing website)
3. Frequently surfed Shadertoy for inspiration
4. The Ray Tracer Challenge (namely, the free bonus chapters on bounding boxes)
