# Galactic Explorer

A cinematic space exploration app built with Rust + Bevy.

## Current Highlights

- Third-person shuttle flight with smooth follow camera
- Selectable planetary targets (Earth and Mars)
- Planet rotation and Mars orbital movement
- Target warp and scanner progression gameplay loop
- Discovery mission tracking and HUD event log
- Twinkling 3D starfield for depth and atmosphere
- Target highlighting to make Earth/Mars selection obvious
- Recovery controls for rapid testing and stable play

## Controls

- W / A / S / D: Strafe and forward/back movement
- Q / E: Move down/up
- Shift: Boost speed
- Arrow Left / Arrow Right: Shuttle yaw rotation
- 1: Target Earth
- 2 or M: Target Mars
- G: Warp toward selected target
- R: Reset shuttle position and orientation
- + / -: Camera zoom in/out

## Mission Flow

1. Select a target using 1 or 2/M.
2. Approach until scanner becomes ACTIVE.
3. Hold position while scan progress reaches 100%.
4. Discovery count increases and mission log updates.

## Run

From workspace root:

cargo run

