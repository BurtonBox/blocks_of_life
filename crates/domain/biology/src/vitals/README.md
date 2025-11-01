# Vitals Module

This module demonstrates **sum types** (enums with data) and **type-safe wrappers** for measurements. It models the vital status of biological entities using Rust's type system to prevent invalid states.

## File Organization

| File | Contains | Purpose |
|------|----------|---------|
| `vital_status.rs` | `VitalStatus` enum | Sum type representing Alive or Deceased states |
| `live_vitals.rs` | `LiveVitals` struct | Vital signs for living organisms |
| `post_mortem_report.rs` | `PostMortemReport` struct | Data for deceased organisms |
| `blood_pressure.rs` | `BloodPressure` struct | Systolic/diastolic tuple wrapper |
| `vital_types.rs` | `Celsius`, `BPM` structs | Type-safe measurement wrappers |

## Core Design: Sum Types

The `VitalStatus` enum uses Rust's algebraic data types to model mutually exclusive states:

```rust
pub enum VitalStatus {
    Alive(LiveVitals),           // Contains live vital signs
    Deceased(PostMortemReport),  // Contains post-mortem data
}
```

**Benefits of this approach:**
- **Impossible states are unrepresentable**: An entity cannot be both alive and deceased
- **Exhaustive matching**: Compiler forces you to handle both cases
- **Each state carries appropriate data**: Alive entities have pulse, deceased have cause of death

## Type-Safe Wrappers

Rather than using raw integers for measurements, this module wraps them in newtype structs:

```rust
pub struct Celsius(pub u32);     // Temperature in Celsius
pub struct BPM(pub u32);         // Beats per minute (heart rate)
pub struct BloodPressure(pub u32, pub u32);  // (systolic, diastolic)
```

**Why this matters:**

```rust
// ❌ Easy to mix up raw integers
fn set_vitals(temp: u32, pulse: u32) { ... }
set_vitals(72, 37);  // Oops! Swapped temperature and pulse

// ✓ Type system catches errors
fn set_vitals(temp: Celsius, pulse: BPM) { ... }
set_vitals(BPM(72), Celsius(37));  // Compile error! Wrong types
```

## LiveVitals

Contains measurements for living organisms:

```rust
pub struct LiveVitals {
    pub blood_pressure: BloodPressure,  // (systolic, diastolic)
    pub temperature: Celsius,            // Body temperature
    pub pulse: BPM,                      // Heart rate
    pub respiration_rate: u32,           // Breaths per minute
}
```

**Usage:**

```rust
use biology::vitals::live_vitals::LiveVitals;
use biology::vitals::blood_pressure::BloodPressure;
use biology::vitals::vital_types::{Celsius, BPM};

let vitals = LiveVitals::new(
    BloodPressure(120, 80),  // Normal blood pressure
    Celsius(37),             // Normal temperature
    BPM(72),                 // Normal resting heart rate
    16,                      // Normal respiration rate
);
```

## PostMortemReport

Contains information for deceased organisms:

```rust
pub struct PostMortemReport {
    pub cause_of_death: String,
    pub date_of_death: String,
    pub findings: Vec<String>,  // Autopsy findings
}
```

## Pattern Matching on VitalStatus

The sum type enables safe, exhaustive handling:

```rust
match entity.vitals {
    VitalStatus::Alive(vitals) => {
        println!("Pulse: {:?}, Temp: {:?}", vitals.pulse, vitals.temperature);
        // `vitals` contains LiveVitals data
    }
    VitalStatus::Deceased(report) => {
        println!("Cause: {}", report.cause_of_death);
        // `report` contains PostMortemReport data
    }
}
```

The compiler **forces** you to handle both cases. You can't forget to check if someone is deceased before reading their pulse!

## Usage Example

```rust
use biology::vitals::vital_status::VitalStatus;
use biology::vitals::live_vitals::LiveVitals;
use biology::vitals::post_mortem_report::PostMortemReport;
use biology::vitals::blood_pressure::BloodPressure;
use biology::vitals::vital_types::{Celsius, BPM};

// Living entity
let alive_status = VitalStatus::Alive(LiveVitals::new(
    BloodPressure(120, 80),
    Celsius(37),
    BPM(72),
    16,
));

// Deceased entity
let deceased_status = VitalStatus::Deceased(PostMortemReport::new(
    "Natural causes".to_string(),
    "2024-01-01".to_string(),
    vec!["No trauma observed".to_string()],
));

// Pattern matching handles both cases
fn check_status(status: &VitalStatus) {
    match status {
        VitalStatus::Alive(vitals) => {
            println!("Living - Pulse: {} BPM", vitals.pulse.0);
        }
        VitalStatus::Deceased(report) => {
            println!("Deceased - Cause: {}", report.cause_of_death);
        }
    }
}
```

## Adding New Vital Signs

To add a new measurement type:

1. Create a type-safe wrapper in `vital_types.rs`:
   ```rust
   pub struct OxygenSaturation(pub u32);  // Percentage
   ```

2. Add it to `LiveVitals`:
   ```rust
   pub struct LiveVitals {
       // ... existing fields
       pub oxygen_saturation: OxygenSaturation,
   }
   ```

3. Update the constructor to accept the new parameter

## Design Benefits

**Type Safety:**
- Can't mix up measurement units
- Compiler prevents invalid state combinations

**Clarity:**
- Sum types make the domain model explicit
- No ambiguous "null" or "zero" values for deceased entities

**Maintainability:**
- Adding new vital signs is straightforward
- Pattern matching ensures all cases are handled
