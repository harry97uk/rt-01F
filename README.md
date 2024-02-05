# Ray Tracer Documentation

This documentation provides information on how to customize and render scenes using the ray tracer.
Table of Contents

- Camera Configuration
- Material Definitions
- Object Placement
- Adjusting Brightness
- Moving the Camera

## Camera Configuration

The camera is the viewpoint through which the scene is rendered. You can adjust various parameters in the camera section of the YAML configuration file:

Example:

    camera:
        aspect_ratio: 1.7778 # Ratio of image width over height
        image_width: 400 # Width of the rendered image in pixels
        samples_per_pixel: 100 # Number of random samples for each pixel
        max_depth: 50 # Maximum number of ray bounces into the scene
        vfov: 20.0 # Vertical field of view
        lookfrom: [3.0, 2.0, -9.0] # Camera position
        lookat: [0.0, 0.0, 0.0] # Point the camera is looking at
        vup: [0.0, 1.0, 0.0] # Camera-relative "up" direction
        brightness: 1.0 # Adjust the overall brightness of the scene

## Material Definitions

Materials define the visual properties of objects. You can create different materials in the materials section:

Example:

    materials:
        [
        ground: { type: Lambertian, colour: [0.5, 0.5, 0.5] }, # Lambertian material for the ground
        cylin: { type: Lambertian, colour: [0.0, 1.0, 0.5] }, # Lambertian material for a cylinder
        centre: { type: Metal, colour: [0.7, 0.3, 0.3] }, # Metal material for a central object
        ]

## Object Placement

Objects define the geometry and material of elements in the scene. You can place different objects in the objects section:

Example:

    objects:
        [
        { type: Plane, mat: ground }, # A Lambertian ground plane
        { type: Sphere, centre: [2.0, 0.0, -1.0], radius: 0.5, mat: centre }, # A metal sphere
        { type: Cylinder, mat: cylin }, # A Lambertian cylinder
        ]

## Adjusting Brightness

You can control the overall brightness of the scene by modifying the brightness value in the camera section.

Example:

    camera:
        brightness: 1.0 # Adjust the overall brightness of the scene (default is 1.0)

## Moving the Camera

To change the camera position and viewpoint, modify the lookfrom field in the camera section.

Example:

    camera:
        lookfrom: [x, y, z] # Adjust the camera position (replace x, y, z with desired coordinates)

Feel free to experiment with these parameters to create unique and visually appealing scenes! After making changes, run the ray tracer to generate the rendered image.
