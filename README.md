```rust
state_machine!(
    player;
    GetHit, Jump;
    mv: &mut MovingObject,
    bb: &mut HasAABB,
    anim: &mut HasAnimationSequence,
    rend: &mut Renderable,
    dir: &Directional,
    pi: &PlayerInput,
    delta: &DeltaTime
);
```
