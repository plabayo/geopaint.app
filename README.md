# geopaint.app

A free web editor to make and print Geometric art.

## Remark

This is very early experimental software,
use at your own risk.

## Roadmap

### 1. Editor

Yew Website with a Canvas, Editor menu's are regular HTML elements,
the canvas is a WebGL canvas.

<https://github.com/yewstack/yew/tree/master/examples/webgl> could serve as a starting point.

Reference for WebGL 2.0: <https://registry.khronos.org/webgl/specs/latest/2.0/> (supported in all Major browsers)

As it's been a while that I've written shader myself using DirectX and OpenGL (ES), it might be good
to start also by first playing a bit with the content of <https://webgl2fundamentals.org>
and re-learning it a bit from scratch as to ensure the content is fresh in the head.

The editor should remain as simple as possible, which not only makes it easier to use and learn,
but also enhances creativity by limiting the freedom.

Summary of the canvas concept:

- The canvas has layers;
- Each layer has the following properties:
    - Size: how big are the shapes (can also be named scale);
    - Edges: defines what kind of "regular shape" the layer represents: triangle, square, pentagon, hexagon, ...;
    - Properties that we will not introduce for an MVP version but that could be added later:
           - Rotation: how are they rotated, allowing you to mix up the shapes
           - Opacity: allow to define an opacity scale that would apply to the entire layer
- Layers have no fixed size, it can be seen as infinite
    - this concept is not yet sure, we might want to fix it,
      and as it is meant for printing it would be good to allow it to be easily transformed
      onto the target object, yet it is not uncommon for one artwork to be printed on many
      different targets
    - to be defined more clearly later
    
### 2. App and Payment Flow

Architecture of the App and Payment Flow.

### 3. Account Flow

Introduce accounts, and the flow.
At this point we also introduce the geopaint server which
will be used for storage (API), accounts (API) and payments.

### 4. Payment Integration

Indirect, goes via our server.

### 4. Print API Integration

- preview prints
- order prints (requires a valid payment first)

All indirect, as it does go via our server.
Need to make sure to store intermediate states as well,
e.g. when payment happened but never did the order,
such that it can be used as credit to restart the process,
or even be refunded.
