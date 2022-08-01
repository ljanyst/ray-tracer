
ray-tracer
==========

My implementation of the [Ray Tracer Challenge][rtc] with the focus on learning
[Rust][rust] and hardware acceleration.

Chapter 4 Demo
--------------

Description:

 * create a unit sphere at origin
 * cast a ray originating from (0, 0, -5) at a 7 by 7 canvas at (x, y, 10)
 * for each pixel, color the pixel red if the ray intersects the sphere;
   otherwise, color the pixel black

![Demo 4 rendering](demo-imgs/demo4.webp)

[code](src/demo/demo4.rs)

[rtc]: http://raytracerchallenge.com/
[rust]: https://www.rust-lang.org/

