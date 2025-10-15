# Characteristics Module

This module contains **core trait definitions** that describe what an organism IS or CAN DO. These are the fundamental capabilities that define biological entities.

## File Organization

Files are named by **domain concept**, not by the specific trait name. This allows related traits to be grouped together as the project grows.

| File | Contains | Purpose |
|------|----------|---------|
| `structural.rs` | `Anatomy` trait | Defines how organisms describe their physical structure |
| `locomotion.rs` | `Mobility` trait | Defines how organisms describe their movement capabilities |
| `presentation.rs` | `Summarizable` trait | Defines how organisms present/display themselves |

## Usage Examples

```rust
// Import from the module re-export (recommended)
use biology::characteristics::{Anatomy, Mobility, Summarizable};

// Or import from specific file
use biology::characteristics::structural::Anatomy;
use biology::characteristics::locomotion::Mobility;
use biology::characteristics::presentation::Summarizable;
```

## Design Principle

**Characteristics are interfaces, not implementations.** They define the contract for what an organism can do, but don't provide the logic. See the `patterns` module for reusable implementations.

## Adding New Characteristics

When adding a new characteristic trait:
1. Determine the domain concept (e.g., sensory, reproductive, metabolic)
2. Create a new file named after the concept (e.g., `sensory.rs`)
3. Define the trait inside (e.g., `pub trait Perception`)
4. Add the module and re-export to `mod.rs`

If a trait is closely related to an existing concept, add it to the existing file rather than creating a new one.
