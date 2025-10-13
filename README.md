# **Blocks of Life: An Exploration of Software Architecture in Rust**

This project is an exercise in iterative software design, using the challenge of modeling biological life as a practical sandbox. It's not a biology simulation, but rather a journey that demonstrates how to build a flexible, scalable, and maintainable application in Rust by progressively applying advanced architectural patterns.

The name blocks\_of\_life reflects the core design principle of the project: **composition**. We start with the smallest "blocks" (like LimbStatus enums or ArmMeasurements structs) and compose them into complex, stateful entities.

## **The Design Journey**

The repository's commit history tells a story. We began with a simple, flawed model and evolved it step-by-step to solve new, more complex requirements. Each stage of the journey introduces a new architectural concept:

1. **Polymorphism with Trait Objects:** The foundation of the model, allowing different entities (Human, Animal, Plant) to be treated uniformly.  
2. **Trait Composition:** Refactoring a single, large trait into smaller, more focused traits (Anatomy, Mobility) for greater accuracy and flexibility.  
3. **Scalable Module System:** Organizing the growing codebase into a clean, modern Rust module structure for maintainability.  
4. **Composition over Inheritance:** Building entities from smaller, independent "block" components (like Appendages) instead of relying on a rigid inheritance hierarchy.  
5. **The Default Trait:** Hiding the complexity of creating a "standard" entity to provide a clean and simple API.  
6. **The Builder Pattern:** Creating a flexible and readable API for constructing complex objects with many optional or customizable fields.  
7. **Data-Driven Design:** Moving from hardcoded logic (match Region) to a flexible, data-driven approach using AnatomyTemplate structs, effectively creating an in-memory database of defaults.

## **Key Architectural Patterns Demonstrated**

This project serves as a practical, hands-on example of several key software design patterns:

* **Polymorphism**  
* **Composition**  
* **The Builder Pattern**  
* **The Factory Pattern** (using for\_region before evolving to Templates)  
* **The Template Pattern** (for data-driven defaults)  
* **Separation of Concerns** (separating data, logic, and presentation)

## **Getting Started**

To see the current model in action, simply clone the repository and run the main application:

git clone \[https://github.com/your-username/blocks\_of\_life.git\](https://github.com/your-username/blocks\_of\_life.git)  
cd blocks\_of\_life  
cargo run

This will print a summary of the entities currently defined in main.rs, showcasing the dynamic anatomy descriptions based on their constructed state.