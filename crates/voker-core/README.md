# Logic core layer

The data-oriented core of the engine, modeled after the ECS (Entity–Component–System)
architecture.

## Concepts

- **Entity** — an entity. At its core it is a 64-bit unsigned integer: a raw handle
  (`EntityId`, an index paired with a generation) that names an entity without
  owning any of its data.
- **Component** — per-entity data. An entity may hold any number of components, but
  at most one component of each type.
- **Scene** — a tree of entities. Scenes are the primary unit of composition and
  serialization.
- **Resource** — data bound to a scene: unique per type and serialized together with
  the scene it belongs to.
- **Service** — scene-independent global data, shared across the whole world.
- **Script** — the behavior interface, conceptually `fn(&mut self, &mut World)`.
  Scripts run non-parallel by default, but can opt into more specific sub-interfaces
  that expose their access pattern and so unlock parallelism:
  - **Entity script** — `fn(&mut self, Entity)`. By guaranteeing properties such as
    "only mutates its own entity" or "reads data only", the scheduler can run many
    instances concurrently.
  - **World script** — `fn(&mut self, &mut World)`. By declaring which components and
    resources it touches, non-conflicting scripts can be scheduled in parallel.
- **World** — the top-level container holding all of the above. A game is typically
  split into a *logic* world and a *render* world (following Bevy's design); the two
  exchange state once per frame.
