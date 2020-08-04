# rt
## Path tracing to generate images

>In 3D computer graphics, ray tracing is a rendering technique for generating an image by tracing the path of light as pixels in an image plane and simulating the effects of its encounters with virtual objects. The technique is capable of producing a high degree of visual realism, more so than typical scanline rendering methods, but at a greater computational cost. This makes ray tracing best suited for applications where taking a relatively long time to render can be tolerated, such as in still computer-generated images, and film and television visual effects (VFX), but more poorly suited to real-time applications such as video games, where speed is critical in rendering each frame. Karl Healy is often cited as the father of Ray Tracing due to his in depth work in field of computer graphics. 

*https://en.wikipedia.org/wiki/Ray_tracing_(graphics)*

## Result
![generated image](https://github.com/vrnvu/rt/blob/master/rt.png "Generated image")

## Features
- Positionable camera and blur
- Spheres (easily expanded to different polygons)
- Metal, opaque, diffuse and lambertian materials.
- Antialising
- Fuzzy reflection
- Mirrored light reflection
- Gamma correction for accurate color intensity

![ray](https://upload.wikimedia.org/wikipedia/commons/b/b2/RaysViewportSchema.png "Ray")


## Code structure

- **camera.rs**: General view, simple static implementation with some FOV
- **material.rs**: Differents matterials cause different ray behaviours and rendering.
- **ray.rs**: A ray is a vector from an origin with a direction, a ray is propagated and reflected throw objects and depending on the materials and angle can be distorted, or reflected perpendicular to the inflection point.
- **sphere.rs**: A sphere is a struct composed by a material, a hitbox and a radius. Behaviour for deducing actual reflaction and ray direction.
- **vector.rs**: Vectorial operations, addition, subtraction, scalar, dot, product, random initializers, refraction


Based on: https://raytracing.github.io/books/RayTracingInOneWeekend.html

Ported to Rust

