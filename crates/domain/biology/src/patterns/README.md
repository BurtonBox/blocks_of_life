# Patterns Module

This module contains **reusable trait implementations** that provide common behaviors and helper functionality. These are "mix-in" traits that add specific capabilities to types.

## File Organization

Files are named by **pattern category**, grouping related helper traits together. Each file may contain multiple related traits.

| File | Contains | Purpose |
|------|----------|---------|
| `structural_patterns.rs` | `AnatomyAnalyzer` | Tools for analyzing and describing anatomical structures (limb counting, status descriptions) |
| `locomotion_patterns.rs` | `BipedalMobility`, `QuadrupedalMobility`, `SessileMovement` | Specific movement pattern implementations |
| `presentation_patterns.rs` | `BasicSummary`, `DetailedSummary` | Helper traits for generating formatted summaries |

## Usage Examples

```rust
// Import from the module re-export (recommended)
use biology::patterns::{AnatomyAnalyzer, BipedalMobility};

// Or import from specific file
use biology::patterns::structural_patterns::AnatomyAnalyzer;
use biology::patterns::locomotion_patterns::BipedalMobility;
```

## Design Principle

**Patterns provide default implementations.** Unlike characteristics (which are just interfaces), pattern traits often include default method implementations that types can use directly.

For example:
- `AnatomyAnalyzer` provides `count_limbs()` and `describe_limb_status()` methods with full implementations
- `BipedalMobility` provides `describe_bipedal_movement()` with a default implementation
- Types only need to implement the required abstract methods (if any)

## Patterns vs Characteristics

| Characteristics | Patterns |
|----------------|----------|
| Core capabilities | Helper utilities |
| Usually abstract | Often have defaults |
| Define "what" | Define "how" |
| Required for entity | Optional mix-ins |

## Adding New Patterns

When adding a new pattern trait:
1. Determine if it relates to an existing pattern category
2. If yes, add to the existing file (e.g., add `AquaticMobility` to `locomotion_patterns.rs`)
3. If no, create a new pattern file (e.g., `metabolic_patterns.rs`)
4. Provide sensible default implementations where possible
5. Add the module and re-export to `mod.rs`
