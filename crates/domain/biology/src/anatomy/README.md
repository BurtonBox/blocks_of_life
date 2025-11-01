# Anatomy Module

This module demonstrates **composition over inheritance** through struct-variant enums and polymorphic wrappers. It defines the physical structure types used by biological entities.

## File Organization

| File | Contains | Purpose |
|------|----------|---------|
| `appendage.rs` | `Appendage` enum | Heterogeneous limb types (Arm, Leg) with variant-specific data |
| `arm_measurements.rs` | `ArmMeasurements` struct | Measurements for arms (span, flexed bicep, etc.) |
| `leg_measurements.rs` | `LegMeasurements` struct | Measurements for legs (inseam, quad, etc.) |
| `limb_status.rs` | `LimbStatus` enum | Status of a limb (Intact, Injured, Severed, etc.) |
| `body.rs` | `Body<'a>` struct | Polymorphic wrapper for trait objects |

## Key Design Pattern: Struct-Variant Enums

The `Appendage` enum demonstrates Rust's powerful enum composition:

```rust
pub enum Appendage {
    Arm {
        side: Direction,
        measurements: ArmMeasurements,
        status: LimbStatus,
    },
    Leg {
        side: Direction,
        measurements: LegMeasurements,
        status: LimbStatus,
    },
}
```

**Benefits:**
- Type-safe heterogeneous collections (`Vec<Appendage>`)
- Each variant carries its own specific data
- Pattern matching ensures exhaustive handling
- Easy to extend with new appendage types (Tail, Wing, Fin, etc.)

## Polymorphic Wrapper: Body

The `Body<'a>` struct wraps any type implementing `Summarizable`:

```rust
pub struct Body<'a> {
    pub entity: &'a dyn Summarizable,
}
```

This enables collections of heterogeneous entities:

```rust
let human = Human::builder().build();
let animal = Animal::new("Dog".to_string());

let bodies = vec![Body::new(&human), Body::new(&animal)];
```

## Usage Examples

```rust
use biology::anatomy::{Appendage, ArmMeasurements, LimbStatus};
use core_shared::{Direction, Centimeters};

// Create an arm
let left_arm = Appendage::Arm {
    side: Direction::Left,
    measurements: ArmMeasurements {
        span: Centimeters(75),
        flexed_bicep: Centimeters(35),
        // ... other measurements
    },
    status: LimbStatus::Intact,
};

// Pattern match on appendage type
match left_arm {
    Appendage::Arm { side, measurements, status } => {
        println!("Arm on {:?} side", side);
    }
    Appendage::Leg { .. } => {
        println!("This is a leg");
    }
}
```

## Adding New Appendage Types

To add a new appendage variant:

1. Create a measurements struct (e.g., `TailMeasurements`)
2. Add a new variant to `Appendage`:
   ```rust
   Tail {
       position: Direction,
       measurements: TailMeasurements,
       status: LimbStatus,
   }
   ```
3. Update pattern matches to handle the new variant
4. Add a description case in `Appendage::description()`

## Composition Over Inheritance

Rather than using inheritance hierarchies (Animal → Mammal → Human), this module uses **composition**:
- Entities contain `Vec<Appendage>`
- Each appendage is independent and composable
- New entity types can mix and match appendages freely
