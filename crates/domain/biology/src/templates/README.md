# Templates Module

This module demonstrates **data-driven design**. Instead of hardcoding regional defaults in match statements or conditional logic, templates provide reusable data structures that can be easily extended.

## File Organization

| File | Contains | Purpose |
|------|----------|---------|
| `anatomy.rs` | `AnatomyTemplate` struct | Container for anatomical measurements |
| `regional_defaults.rs` | Template constants | Pre-defined templates for different populations |

## Core Concept: AnatomyTemplate

```rust
pub struct AnatomyTemplate {
    pub arm_measurements: ArmMeasurements,
    pub leg_measurements: LegMeasurements,
    // Extensible: add torso_measurements, head_measurements, etc.
}
```

This struct acts as a **blueprint** for default measurements. Rather than writing:

```rust
// ❌ Anti-pattern: Hardcoded logic
match region {
    Region::NorthAmerica => {
        arms = ArmMeasurements { span: Centimeters(175), ... };
        legs = LegMeasurements { inseam: Centimeters(81), ... };
    }
    Region::EastAsia => {
        arms = ArmMeasurements { span: Centimeters(170), ... };
        legs = LegMeasurements { inseam: Centimeters(78), ... };
    }
}
```

We use **data templates**:

```rust
// ✓ Better: Data-driven approach
Human::builder()
    .with_anatomy_template(&regional_defaults::AVG_NORTH_AMERICAN_MALE)
    .build()
```

## Available Regional Templates

The `regional_defaults.rs` file provides scientifically-sourced population averages:

### North American
- `AVG_NORTH_AMERICAN_MALE` - Based on CDC NHANES 2015-2018
- `AVG_NORTH_AMERICAN_FEMALE`

### East Asian
- `AVG_EAST_ASIAN_MALE` - Chinese/Korean/Japanese populations
- `AVG_EAST_ASIAN_FEMALE`

### Northern European
- `AVG_NORTHERN_EUROPEAN_MALE` - Scandinavian/Dutch/German
- `AVG_NORTHERN_EUROPEAN_FEMALE`

### South Asian
- `AVG_SOUTH_ASIAN_MALE` - Indian subcontinent
- `AVG_SOUTH_ASIAN_FEMALE`

### African
- `AVG_AFRICAN_MALE` - Sub-Saharan African populations
- `AVG_AFRICAN_FEMALE`

## Usage Example

```rust
use biology::templates::regional_defaults;
use biology_animalia::{Human, NameParts};
use biology::Sex;

let person = Human::builder()
    .name(NameParts::from("Jane Doe"))
    .sex(Sex::Female)
    .with_anatomy_template(&regional_defaults::AVG_NORTHERN_EUROPEAN_FEMALE)
    .build();

// The builder applies all measurements from the template
assert_eq!(person.appendages.len(), 4); // 2 arms + 2 legs with template measurements
```

## Adding New Templates

To add a new regional template:

1. Research anthropometric data for the population
2. Add a new constant to `regional_defaults.rs`:

```rust
pub const AVG_CUSTOM_POPULATION: AnatomyTemplate = AnatomyTemplate {
    arm_measurements: ArmMeasurements {
        upper_length: Centimeters(38),
        triceps_length: Centimeters(26),
        span: Centimeters(175),
        flexed_bicep: Centimeters(35),
        unflexed_bicep: Centimeters(33),
    },
    leg_measurements: LegMeasurements {
        inseam: Centimeters(81),
        quad: Centimeters(58),
    },
};
```

3. Document the data sources in comments
4. Use the template in builders

## Extending AnatomyTemplate

To add new measurement types:

1. Create the measurement struct (e.g., `TorsoMeasurements`)
2. Add it to `AnatomyTemplate`:
   ```rust
   pub struct AnatomyTemplate {
       pub arm_measurements: ArmMeasurements,
       pub leg_measurements: LegMeasurements,
       pub torso_measurements: TorsoMeasurements,  // NEW
   }
   ```
3. Update all template constants in `regional_defaults.rs`
4. Update builders to apply the new measurements

## Design Benefits

**Separation of Concerns:**
- Data (templates) is separate from logic (builders)
- Easy to update measurements without touching code
- Templates can be loaded from files or databases later

**Testability:**
- Create custom templates for testing edge cases
- No need to mock complex conditional logic

**Maintainability:**
- Adding a new population only requires data
- No changes to control flow or business logic
- Clear documentation of data sources
