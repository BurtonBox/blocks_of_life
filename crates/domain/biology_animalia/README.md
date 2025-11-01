# Biology Animalia - API Design & Development Guide

This crate demonstrates **progressive complexity** - a pattern where biological models exist at different levels of detail depending on their use case. Understanding this pattern is critical for contributing to the project.

## The Core Question: Generic vs. Specialized?

**When adding a new organism, should you create a simple generic type or a detailed specialized implementation?**

This README answers that question and explains the architectural pattern guiding this crate.

---

## Current API Overview

### Generic Models (Simple, Baseline)

**Animal** - Simplest baseline for any animal
```rust
let bird = Animal::new("Aves genericus".to_string());
// Provides: nomenclature, basic vitals, simple summary
// NO builder, NO detailed anatomy, NO specialized traits
```

**Mammal** - Class-level generic with mammalian characteristics
```rust
let cat = Mammal::new("Felis catus".to_string());
// Provides: all Animal features + has_fur, mammalian traits
// NO builder, NO detailed anatomy
// CAN: Set fur via .with_fur(false) for hairless mammals
```

### Specialized Models (Detailed, Feature-Rich)

**Human** - Fully detailed species-specific implementation
```rust
let person = Human::builder()
    .name(NameParts::from("Jane Doe"))
    .sex(Sex::Female)
    .with_anatomy_template(&regional_defaults::AVG_NORTH_AMERICAN_FEMALE)
    .with_vitals(VitalStatus::Alive(...))
    .build();
// Provides: Builder pattern, structured naming, detailed appendages,
//           regional templates, comprehensive trait implementations
```

---

## The Progressive Complexity Pattern

```
┌─────────────────────────────────────────────────────────────┐
│ Animal (Baseline)                                           │
│ • Species name                                              │
│ • Basic vitals                                              │
│ • Simple construction: Animal::new(name)                    │
│ USE CASE: Quick placeholder, minimal modeling               │
└─────────────────────────────────────────────────────────────┘
                          ↓ refines
┌─────────────────────────────────────────────────────────────┐
│ Mammal (Class-Level)                                        │
│ • All Animal features                                       │
│ • + Class-specific traits (has_fur)                         │
│ • + Class-specific anatomy descriptions                     │
│ • Simple construction: Mammal::new(name).with_fur(true)     │
│ USE CASE: Generic mammals without needing full detail       │
└─────────────────────────────────────────────────────────────┘
                          ↓ refines
┌─────────────────────────────────────────────────────────────┐
│ Human (Species-Specific)                                    │
│ • All Mammal/Animal features                                │
│ • + Builder pattern for complex construction                │
│ • + Structured naming (NameParts vs simple string)          │
│ • + Detailed anatomy (individual appendages with measurements) │
│ • + Regional templates for realistic defaults               │
│ • Complex construction: Human::builder()...build()          │
│ USE CASE: Detailed study, realistic simulation, complex scenarios │
└─────────────────────────────────────────────────────────────┘
```

---

## Decision Matrix: When to Use Each Level

### Use Generic Models (Animal, Mammal) When:

✓ You need a **placeholder** or background entity
✓ The organism is **not the focus** of study
✓ You need to **quickly instantiate** many entities
✓ **Minimal detail** is sufficient for your use case
✓ You're modeling an **entire class** generically (e.g., "generic bird")

**Example Scenarios:**
- Background animals in an ecosystem simulation
- Generic prey in a predation model
- Placeholder entities in tests
- Modeling broad categories (all reptiles, all fish)

### Use Specialized Models (Human-like) When:

✓ The organism is the **primary subject** of study
✓ You need **realistic, detailed** modeling
✓ You need **regional or demographic** variations
✓ You need **structured data** (e.g., human names vs. just a string)
✓ You're building a **case study** or **detailed scenario**

**Example Scenarios:**
- Medical simulations with human patients
- Anthropological studies
- Detailed individual tracking
- Research requiring anatomical precision

---

## Development Guidelines

### Adding a New Generic Animal Class

**When:** You need a class-level generic (like Mammal for mammals)

**Steps:**
1. Create a struct in the appropriate taxonomic path (e.g., `birds/bird.rs` for Aves)
2. Include basic fields: `species_name`, `vitals`, `id`
3. Add 1-3 class-specific traits (e.g., `can_fly`, `wingspan` for birds)
4. Implement: `Nomenclature`, `Summarizable`, `Anatomy`, `Mobility`
5. Provide a simple `::new(name)` constructor
6. Optional: Provide `.with_*()` methods for class-specific traits

**Example:**
```rust
// Generic Bird (class-level)
pub struct Bird {
    pub id: Identifier,
    pub species_name: String,
    pub vitals: VitalStatus,
    pub can_fly: bool,  // Class-specific
}

impl Bird {
    pub fn new(species_name: String) -> Self {
        Self {
            id: Identifier::new(),
            species_name,
            vitals: VitalStatus::default(),
            can_fly: true,
        }
    }

    pub fn with_flight(mut self, can_fly: bool) -> Self {
        self.can_fly = can_fly;
        self
    }
}
```

### Adding a New Specialized Species

**When:** You need detailed, realistic modeling of a specific species

**Steps:**
1. Create a directory for the species (e.g., `primates/chimpanzee/`)
2. Create the main struct file (`chimpanzee.rs`)
3. Create a builder (`builder.rs`)
4. Implement comprehensive traits in separate files:
   - `anatomy.rs` - Detailed anatomical descriptions
   - `mobility.rs` - Species-specific movement
   - `summarizable.rs` - Rich summary output
5. Add species-specific templates if needed
6. Consider structured naming if applicable
7. Export via `mod.rs` with re-exports

**Example:**
```rust
// Specialized Chimpanzee (species-level)
pub struct Chimpanzee {
    pub id: Identifier,
    pub name: String,  // or Moniker if you need structured naming
    pub sex: Sex,
    pub vitals: VitalStatus,
    pub appendages: Vec<Appendage>,  // Detailed anatomy
    pub social_rank: SocialRank,  // Species-specific
}

impl Chimpanzee {
    pub fn builder() -> ChimpanzeeBuilder {
        ChimpanzeeBuilder::new()
    }
}

// ChimpanzeeBuilder with methods like:
// .name(), .sex(), .with_anatomy_template(), .social_rank(), etc.
```

---

## Current API (As of v0.1.0)

### Available Types

| Type | Level | Constructor | Features |
|------|-------|-------------|----------|
| `Animal` | Generic baseline | `::new(name)` | Species name, vitals, basic traits |
| `Mammal` | Class-level | `::new(name)` + `::with_fur(bool)` | All Animal + mammalian traits |
| `Human` | Species-specific | `::builder()` | Full builder, structured naming, detailed anatomy, regional templates |

### Import Paths

```rust
// Recommended: Use re-exports
use biology_animalia::{Animal, Mammal, Human, NameParts};

// Also valid: Full taxonomic paths
use biology_animalia::mammals::primates::Human;
use biology_animalia::mammals::Mammal;
use biology_animalia::animal::Animal;
```

---

## Planned Future Development

### Short-Term (Next Species to Add)

Consider adding one of these as **specialized** implementations:
- **Dog** (Canis familiaris) - Demonstrates domesticated animal with breeds
- **Chimpanzee** (Pan troglodytes) - Demonstrates primate with social hierarchy
- **Horse** (Equus caballus) - Demonstrates large mammal with detailed measurements

### Medium-Term (New Classes)

Add **generic class-level** implementations:
- **Bird** - Add `can_fly`, `wingspan`, `feather_type`
- **Reptile** - Add `is_cold_blooded`, `scale_type`, `venomous`
- **Fish** - Add `water_type` (fresh/salt), `swim_depth_range`

### Long-Term Vision

- [ ] Builder pattern for 2-3 additional specialized species
- [ ] Templates for more species (dog breeds, horse types)
- [ ] Behavior modeling (migration patterns, hibernation)
- [ ] Life cycle stages (juvenile, adult, elderly with different measurements)

---

## Architecture Benefits

**Flexibility:**
- Use simple models when detail isn't needed
- Use complex models when realism matters
- Both can coexist in the same ecosystem

**Performance:**
- Generic types are lightweight
- Specialized types only pay for what they use

**Learning Path:**
- Start with generic models to understand basics
- Progress to specialized models to see advanced patterns
- Code demonstrates iterative refinement

**Maintainability:**
- Generic types are easy to add and maintain
- Specialized types showcase best practices
- Clear separation of concerns

---

## Questions to Ask When Adding New Organisms

1. **Is this organism the focus of study?**
   - Yes → Consider specialized
   - No → Use generic

2. **Do you need demographic/regional variations?**
   - Yes → Definitely specialized (needs templates)
   - No → Generic is fine

3. **Do you need detailed anatomical measurements?**
   - Yes → Specialized with builder
   - No → Generic with simple fields

4. **Will users need structured data access?**
   - Yes → Specialized (e.g., structured names)
   - No → Generic (simple string names)

5. **Are there 5+ configuration options?**
   - Yes → Consider builder pattern
   - No → Simple `::new()` with `.with_*()` methods

---

## Contributing

When adding new organisms:

1. **Start generic** - Begin with a simple implementation
2. **Refine as needed** - Only add complexity when use cases demand it
3. **Follow existing patterns** - Look at Animal → Mammal → Human progression
4. **Document decisions** - Explain why you chose generic vs. specialized
5. **Add tests** - Demonstrate both simple and complex usage

---

## Related Documentation

- `/characteristics/README.md` - Core trait definitions
- `/patterns/README.md` - Reusable trait implementations
- `/anatomy/README.md` - Composition patterns for detailed anatomy
- `/templates/README.md` - Data-driven design for regional defaults
