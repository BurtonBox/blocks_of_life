# Ecosystem Module

This module demonstrates **higher-level composition** by combining individual biological entities into populations within environmental contexts. It models ecological relationships and environmental factors.

## File Organization

| File | Contains | Purpose |
|------|----------|---------|
| `environment.rs` | `Environment`, `Climate`, `Terrain` | Environmental conditions and settings |
| `population.rs` | `Population<'a>` struct | Collection of entities in a shared environment |
| `interactions.rs` | `Interaction` enum | Types of ecological relationships |

## Core Concept: Composition Layers

The ecosystem module builds on the entity layer:

```
Individual Entity (Human, Animal, Mammal)
    ↓ wrapped in
Body<'a> (polymorphic wrapper)
    ↓ collected into
Population<'a> (group of Bodies)
    ↓ contextualized by
Environment (climate, terrain, etc.)
    ↓ relationships via
Interaction (predation, competition, etc.)
```

## Environment

Models the physical and climatic conditions:

```rust
pub struct Environment {
    pub climate: Climate,        // Tropical, Temperate, Arctic, Desert, Custom
    pub terrain: Terrain,         // Forest, Plains, Mountains, Ocean, Urban, Wetlands
    pub altitude: Centimeters,    // Elevation above sea level
    pub humidity: f32,            // Relative humidity (0.0-1.0)
}
```

### Climate Variants

```rust
pub enum Climate {
    Tropical,
    Temperate,
    Arctic,
    Desert,
    Custom {
        avg_temp: Celsius,
        rainfall_mm: u32,
    },
}
```

**Custom variant** allows modeling of specific conditions not covered by presets.

### Terrain Types

```rust
pub enum Terrain {
    Forest,
    Plains,
    Mountains,
    Ocean,
    Urban,
    Wetlands,
}
```

## Population

A collection of heterogeneous entities sharing an environment:

```rust
pub struct Population<'a> {
    pub members: Vec<Body<'a>>,       // Polymorphic collection of entities
    pub environment: Environment,      // Shared environmental context
    pub interactions: Vec<Interaction>, // Ecological relationships
}
```

**Key Features:**
- **Heterogeneous**: Can contain any type implementing `Summarizable` (Humans, Animals, etc.)
- **Lifetimes**: `'a` ensures entities outlive the population
- **Shared context**: All members exist in the same environment

## Interaction

Models ecological relationships between population members:

```rust
pub enum Interaction {
    Predation {
        predator_id: usize,
        prey_id: usize,
    },
    Competition {
        competitor1_id: usize,
        competitor2_id: usize,
        resource: String,
    },
    Mutualism {
        participant1_id: usize,
        participant2_id: usize,
        benefit: String,
    },
    Parasitism {
        parasite_id: usize,
        host_id: usize,
    },
}
```

Each variant captures the specific data for that relationship type.

## Usage Example

```rust
use biology::anatomy::body::Body;
use biology::ecosystem::environment::{Climate, Environment, Terrain};
use biology::ecosystem::population::Population;
use biology::ecosystem::interactions::Interaction;
use biology_animalia::{Human, Animal};
use core_shared::Centimeters;

// Create an environment
let savanna = Environment {
    climate: Climate::Tropical,
    terrain: Terrain::Plains,
    altitude: Centimeters(120000),  // 1200 meters
    humidity: 0.35,
};

// Create a population in that environment
let mut ecosystem = Population::new(savanna);

// Add heterogeneous members
let lion = Animal::new("Panthera leo".to_string());
let researcher = Human::builder()
    .designation("Field Researcher #1".to_string())
    .build();

ecosystem.add_member(Body::new(&lion));
ecosystem.add_member(Body::new(&researcher));

// Model interactions
ecosystem.add_interaction(Interaction::Predation {
    predator_id: 0,  // Lion
    prey_id: 2,      // Some other animal
});

ecosystem.add_interaction(Interaction::Competition {
    competitor1_id: 0,
    competitor2_id: 1,
    resource: "Water sources".to_string(),
});

// Query the ecosystem
println!("Population size: {}", ecosystem.size());
println!("Climate: {:?}", ecosystem.environment.climate);
println!("Interactions: {}", ecosystem.interactions.len());

// Display all members
for body in &ecosystem.members {
    body.display_summary();
}
```

## Design Patterns

### 1. Polymorphic Collections via Body<'a>

```rust
// Different entity types in the same collection
let mut members = vec![
    Body::new(&human),
    Body::new(&animal),
    Body::new(&mammal),
];
```

All types must implement `Summarizable`, enabling uniform operations.

### 2. Context Object (Environment)

Rather than each entity storing its own environment, the `Population` provides shared context:

```rust
// ❌ Anti-pattern: Each entity stores environment
struct Human {
    environment: Environment,  // Duplicated across all humans
    // ...
}

// ✓ Better: Population provides shared context
struct Population<'a> {
    members: Vec<Body<'a>>,
    environment: Environment,  // Single source of truth
}
```

### 3. Relationship Modeling via IDs

Interactions use indices (`usize`) to reference population members:

```rust
Interaction::Predation {
    predator_id: 0,  // Index into population.members
    prey_id: 1,
}
```

**Trade-off:** Simple but requires maintaining valid indices. Could be improved with stronger typing (e.g., `EntityId` newtype).

## Adding New Features

### New Climate Type

```rust
pub enum Climate {
    // ... existing variants
    Subarctic {
        permafrost_depth: Centimeters,
    },
}
```

### New Terrain Type

```rust
pub enum Terrain {
    // ... existing variants
    Tundra,
    Savanna,
}
```

### New Interaction Type

```rust
pub enum Interaction {
    // ... existing variants
    Commensalism {
        beneficiary_id: usize,
        host_id: usize,
        description: String,
    },
}
```

## Real-World Applications

This pattern is useful for:
- **Simulation systems**: Game entities in a world
- **Multi-tenancy**: Users in an organization context
- **Event sourcing**: Events in an aggregate context
- **Scientific modeling**: Specimens in an experiment

## Design Benefits

**Composition:**
- Entities compose into populations
- Populations exist within environments
- Interactions model relationships

**Polymorphism:**
- Any `Summarizable` type can join a population
- Uniform operations across heterogeneous entities

**Separation of Concerns:**
- Entities don't know about their environment
- Environment doesn't know about specific entity types
- Population bridges the gap
