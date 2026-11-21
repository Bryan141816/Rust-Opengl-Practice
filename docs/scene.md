# Scene System

## Scene

A `Scene` represents a collection of nodes that make up a portion of the game world. It serves as the root container for organizing objects, their hierarchy, and their relationships.

A scene can contain physical objects, cameras, lights, and organizational nodes. The engine traverses the scene hierarchy to gather and process relevant data, such as meshes, materials, lights, and cameras, before passing the necessary information to the rendering system.

### Structure of a Scene

| Variable Name | Data Type | Description | Default |
|---|---|---|---|
| root | Node | The root node of the scene hierarchy. All other nodes are organized beneath this node. | `Node::new()` |

---

## Node

A `Node` is the fundamental building block of a scene. It contains a transform, components, and child nodes, allowing objects to be organized hierarchically.

Nodes can represent physical objects in the game world or serve purely as organizational containers. Each node has its own local transform, which is combined with its parent's world transform to determine its final position, rotation, and scale in the world.

### Structure of a Node

| Variable Name | Data Type | Description | Default |
|---|---|---|---|
| name | String | The name used to identify the node. | |
| transform | Transform | The local transformation of the node relative to its parent. | Identity |
| components | Vec<Component> | A collection of components attached to the node. | Empty |
| children | Vec<Node> | A collection of child nodes. | Empty |

### Node Hierarchy

Nodes can contain other nodes, forming a hierarchical structure. Child nodes inherit the transformation of their parent, allowing related objects to move, rotate, and scale together.

The engine calculates the world transform of each node by combining its local transform with the world transform of its parent.

---

## Component

A `Component` defines a specific capability or functionality that can be attached to a node. It allows nodes to have different properties and roles without changing the node's fundamental structure.

Components provide functionality such as rendering meshes, defining lights, or acting as a camera. A node can contain multiple components, allowing it to serve multiple purposes.

### Types of Components

| Component | Data Type | Description |
|---|---|---|
| Mesh | MeshInstance | Defines the mesh and material used for rendering. |
| Light | Light | Defines a light source and its lighting properties. |
| Camera | Camera | Defines the projection settings used to view the scene. |

---

## MeshInstance

A `MeshInstance` is a component that allows a node to be rendered using a mesh and a material. It references mesh and material resources rather than directly storing their underlying data.

This allows multiple nodes to share the same mesh or material resources without duplicating their data in memory.

### Structure of a MeshInstance

| Variable Name | Data Type | Description |
|---|---|---|
| mesh | MeshHandle | A reference to a mesh resource containing vertex and index data. |
| material | MaterialHandle | A reference to a material resource defining the surface appearance of the mesh. |

---

## Camera

A `Camera` is a component that defines how the scene is viewed and projected onto the screen. It determines the projection type and the parameters used to generate the camera's projection matrix.

The camera's position and orientation are determined by the transform of the node it is attached to, while the camera component contains the settings that define how the scene is viewed.

### Structure of a Camera

| Variable Name | Data Type | Description |
|---|---|---|
| fov | f32 | The field of view of the camera, typically expressed in degrees. |
| aspect_ratio | f32 | The ratio between the viewport width and height. |
| near_plane | f32 | The nearest distance at which objects are visible to the camera. |
| far_plane | f32 | The farthest distance at which objects are visible to the camera. |

### Camera Projection

The camera uses its projection settings to transform the scene into clip space before it is rendered.

- **Field of View (FOV)** determines how much of the world is visible through the camera.
- **Aspect Ratio** determines the proportions of the camera's view.
- **Near Plane** defines the minimum distance at which objects can be rendered.
- **Far Plane** defines the maximum distance at which objects can be rendered.

---

## Light

A `Light` is a component that defines a light source in the scene. It provides the information required by the rendering system to illuminate objects and calculate their appearance.

The light's position and orientation can be determined by the transform of the node it is attached to, while the light component defines its type and lighting properties.

### Types of Light

| Light Type | Data Type | Description |
|---|---|---|
| Directional | DirectionalLight | Emits light in a uniform direction, simulating a distant light source such as the sun. |
| Point | PointLight | Emits light in all directions from a specific position. |
| Spot | SpotLight | Emits light in a specific direction within a defined cone. |

### DirectionalLight

A `DirectionalLight` represents a light source that emits light in a single direction. Its position is generally irrelevant to lighting calculations, making it suitable for simulating distant light sources.

#### Structure of a DirectionalLight

| Variable Name | Data Type | Description |
|---|---|---|
| direction | Vec3 | The direction in which the light travels. |
| color | Vec3 | The color of the light. |
| intensity | f32 | The strength or brightness of the light. |

### PointLight

A `PointLight` emits light in all directions from a specific position. Its illumination decreases with distance, making it suitable for light sources such as lamps, torches, and bulbs.

#### Structure of a PointLight

| Variable Name | Data Type | Description |
|---|---|---|
| color | Vec3 | The color of the light. |
| intensity | f32 | The strength or brightness of the light. |
| constant | f32 | The constant attenuation factor. |
| linear | f32 | The linear attenuation factor. |
| quadratic | f32 | The quadratic attenuation factor. |

### SpotLight

A `SpotLight` emits light in a specific direction within a cone. It is commonly used for light sources such as flashlights, spotlights, and headlights.

#### Structure of a SpotLight

| Variable Name | Data Type | Description |
|---|---|---|
| direction | Vec3 | The direction in which the spotlight is pointing. |
| color | Vec3 | The color of the light. |
| intensity | f32 | The strength or brightness of the light. |
| cut_off | f32 | The inner cone angle threshold where the spotlight maintains its full intensity. |
| outer_cut_off | f32 | The outer cone angle threshold where the spotlight fades out completely. |

### Light Enum

The `Light` enum represents the different types of light components supported by the engine.

| Variant | Data Type |
|---|---|
| Directional | DirectionalLight |
| Point | PointLight |
| Spot | SpotLight |

---

## Resources and Handles

Resources are data managed independently from the scene hierarchy. They can be shared among multiple nodes without duplicating their underlying data.

Handles are references used to access these resources. Instead of storing mesh or material data directly inside a node, components hold handles to the resources managed by the engine.

| Resource | Handle | Description |
|---|---|---|
| Mesh | MeshHandle | References mesh geometry, such as vertex and index data. |
| Material | MaterialHandle | References material properties, textures, and shader-related settings. |

---

## Scene Traversal

Scene traversal is the process of recursively visiting nodes in the scene hierarchy to gather and process their data.

During traversal, the engine combines each node's local transform with its parent's world transform to calculate the node's world transform. It then examines the node's components to identify relevant data required by different engine systems.

For example, the rendering system can collect mesh instances, the lighting system can collect light components, and the camera system can identify available cameras.

### Traversal Process

1. Start at the scene's root node.
2. Calculate the node's world transform using its local transform and its parent's world transform.
3. Inspect the node's components.
4. Collect or process components required by the engine systems.
5. Traverse the node's children recursively.
6. Continue until all nodes in the scene hierarchy have been processed.

The collected data is then used by the engine's rendering and other systems to process the current scene.

