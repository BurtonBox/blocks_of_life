# Nomenclature Module

This module defines the **naming system** for biological entities. It provides two complementary traits that separate display concerns from structured data access.

## File Organization

| File | Contains | Purpose |
|------|----------|---------|
| `nomenclature.rs` | `Nomenclature` and `NomenclatureComponents` traits | Naming interfaces for entities |

## Two Trait System

### 1. Nomenclature (Display Interface)

```rust
pub trait Nomenclature {
    fn display_name(&self) -> String;
}
```

**Purpose:** Provides a single, formatted string representation of an entity's name suitable for display to users.

**Examples:**
- Human with structured name → `"Dr. Jane Marie Smith Jr."`
- Human with designation → `"Case #12345"`
- Animal → `"Canis lupus"` (scientific name)

### 2. NomenclatureComponents (Structured Access)

```rust
pub trait NomenclatureComponents {
    fn prefix_name(&self) -> Option<String>;   // "Dr.", "Mr.", "Ms."
    fn first_name(&self) -> Option<String>;    // "Jane"
    fn middle_name(&self) -> Option<String>;   // "Marie"
    fn last_name(&self) -> Option<String>;     // "Smith"
    fn suffix_name(&self) -> Option<String>;   // "Jr.", "III", "PhD"
}
```

**Purpose:** Provides structured access to individual name components. Returns `Option<String>` because not all entities have all components.

**Examples:**
- Human with structured name → All fields may be populated
- Human with designation → All fields return `None` (designations like "Case #12345" don't have structured parts)
- Animal → All fields return `None` (animals don't have human-style names)

## Why Two Traits?

**Separation of Concerns:**
- `Nomenclature` is for **display** (UI, logging, reports)
- `NomenclatureComponents` is for **data access** (forms, filters, sorting)

**Flexibility:**
- Some entities have structured names (Humans with `NameParts`)
- Some entities have simple identifiers (Humans with designations, Animals)
- Not all entities need both traits

## Implementation Examples

### Human with Structured Name (Implements Both Traits)

```rust
// Via Moniker enum
impl Nomenclature for Moniker {
    fn display_name(&self) -> String {
        match self {
            Moniker::Name(name) => {
                // Concatenates: prefix + first + middle + last + suffix
                "Dr. Jane Marie Smith Jr."
            }
            Moniker::Designation(title) => {
                title.clone()  // "Case #12345"
            }
        }
    }
}

impl NomenclatureComponents for Moniker {
    fn first_name(&self) -> Option<String> {
        match self {
            Moniker::Name(name) => name.first.clone(),  // Some("Jane")
            Moniker::Designation(_) => None,            // Designations have no structure
        }
    }
    // ... other component methods
}
```

### Animal (Implements Only Nomenclature)

```rust
impl Nomenclature for Animal {
    fn display_name(&self) -> String {
        self.species_name.clone()  // Simple string
    }
}

// Does NOT implement NomenclatureComponents
// Animals don't have "first name" or "last name"
```

## Usage Examples

### Display Name (Always Available)

```rust
use biology::Nomenclature;

fn print_entity_name(entity: &impl Nomenclature) {
    println!("Name: {}", entity.display_name());
}

// Works for any type implementing Nomenclature
print_entity_name(&human);   // "Jane Smith"
print_entity_name(&animal);  // "Canis lupus"
```

### Accessing Structured Components (Optional)

```rust
use biology::nomenclature::nomenclature::NomenclatureComponents;

fn print_formal_greeting(entity: &impl NomenclatureComponents) {
    if let Some(prefix) = entity.prefix_name() {
        print!("{} ", prefix);  // "Dr. "
    }
    if let Some(last) = entity.last_name() {
        println!("{}", last);   // "Smith"
    }
}

// Only works for types with structured names
print_formal_greeting(&human_with_name);        // ✓ Works
// print_formal_greeting(&human_with_designation); // ✓ Works but returns all None
// print_formal_greeting(&animal);                 // ✗ Compile error - doesn't implement trait
```

## Real-World Application

This design pattern is common in domain modeling:

**Display vs. Structure:**
- **Postal addresses**: `display()` → "123 Main St, Springfield, IL 62701" vs. structured fields (street, city, state, zip)
- **Phone numbers**: `display()` → "(555) 123-4567" vs. structured fields (country_code, area_code, number)
- **Dates**: `display()` → "January 15, 2024" vs. structured fields (year, month, day)

## When to Implement Each Trait

| Entity Type | Nomenclature | NomenclatureComponents |
|------------|--------------|----------------------|
| Has simple name | ✓ Required | ✗ Not needed |
| Has structured name | ✓ Required | ✓ Recommended |
| Has designation/ID only | ✓ Required | Optional (returns all None) |

## Design Benefits

**Type Safety:**
- Compiler ensures all entities can be displayed
- Optional structured access where it makes sense

**Flexibility:**
- New entity types choose which traits to implement
- Doesn't force inappropriate structure on simple names

**Extensibility:**
- Easy to add new name component methods
- Easy to support new naming conventions
