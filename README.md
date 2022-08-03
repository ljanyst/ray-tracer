
ray-tracer
==========

My implementation of the [Ray Tracer Challenge][rtc] with the focus on learning
[Rust][rust] and hardware acceleration.

Chapter 4 Demo - Red Shadow
---------------------------

Description:

 * create a unit sphere at origin
 * cast a ray originating from (0, 0, -5) at a 7 by 7 canvas at (x, y, 10)
 * for each pixel, color the pixel red if the ray intersects the sphere;
   otherwise, color the pixel black

![Demo 4 rendering](demo-imgs/demo4.webp)

[code](src/demo/demo4.rs)

Chapter 5 Demo - The Phong Refletcion Model
-------------------------------------------

Description:

 * take the previous demo
 * set the material color of the sphere
 * add a point light source
 * for each pixel, if the ray hits the sphere, compute the lighting at the
   intersection; make the pixel black otherwise

![Demo 5 rendering](demo-imgs/demo5.webp)

[code](src/demo/demo5.rs)

Chapter 6 Demo - Worlds and Cameras
-----------------------------------

Description:

 * build a world with ball shapes in it
 * add the lighting
 * place the camera at a custom location
 * render the scene using the camera properties

![Demo 6 rendering](demo-imgs/demo6.webp)

[code](src/demo/demo6.rs)


[rtc]: http://raytracerchallenge.com/
[rust]: https://www.rust-lang.org/

