# Practice Exercises for Rust Module System Skills

Below are practice exercises designed to help you develop and evaluate the skills needed to master Rust's module system. Each exercise targets specific objectives from our list.

## Exercise 1: Basic Module Organization

**Skills practiced**: Code organization, deciding public vs. private elements

**Task**: Create a library crate for a simple task management system with the following structure:
- A `task` module containing:
  - A `Task` struct with fields: `id`, `title`, `description`, and `completed`
  - Functions to create, update, and delete tasks
- A `list` module containing:
  - A `TaskList` struct that holds a collection of tasks
  - Functions to add tasks, remove tasks, and list all tasks

Decide which elements should be public and which should be private. Write code to demonstrate how a user of your library would interact with your API.

**Evaluation criteria**: Are the modules logically structured? Is the privacy correctly set up to expose only what users need?

## Exercise 2: Path Navigation and File Management

**Skills practiced**: Path navigation, file management

**Task**: Split the code from Exercise 1 into multiple files:
1. Create appropriate files for each module
2. Ensure all paths are correctly defined for items to be accessible
3. Use both absolute and relative paths in different parts of the code
4. Include at least one example of using `super`

**Evaluation criteria**: Does the code compile without errors? Are files organized according to Rust conventions? Are paths used appropriately?

## Exercise 3: Import Management

**Skills practiced**: Using `use` efficiently, creating readable import sections

**Task**: Create a file that uses functionality from the modules you created in Exercises 1-2:
1. Use the `use` keyword to bring relevant items into scope
2. Apply idiomatic import patterns (functions vs. structs)
3. Use nested paths to organize related imports
4. Use the `as` keyword to rename at least one import
5. Write code that demonstrates each imported item

**Evaluation criteria**: Are imports organized following Rust idioms? Is code duplication minimized? Are imports easy to read and understand?

## Exercise 4: API Design with Re-exports

**Skills practiced**: API design, re-exporting

**Task**: Improve the API of your task management system:
1. Create a front-facing API that re-exports the most important functionality
2. Use `pub use` to make the interface more convenient for users
3. Hide implementation details that users shouldn't access directly
4. Write example code showing how a user would interact with your improved API

**Evaluation criteria**: Does the API present a logical interface? Are implementation details appropriately hidden? Is the API convenient to use?

## Exercise 5: External Dependency Integration

**Skills practiced**: Dependency management, import management

**Task**: Enhance your task management system by adding date/time functionality:
1. Add `chrono` as a dependency to your project
2. Add due dates to tasks using `chrono`'s date types
3. Implement functions to:
   - Find overdue tasks
   - Sort tasks by due date
   - Calculate days remaining until due date
4. Properly import and use the external functionality

**Evaluation criteria**: Is the external crate correctly integrated? Are imports managed cleanly? Does the enhanced functionality work correctly?

## Exercise 6: Complete Project Structure

**Skills practiced**: All skills combined - code organization, file management, path navigation, import management, dependency management

**Task**: Create a complete command-line task management application:
1. Set up a binary crate that depends on your library crate
2. Implement a command-line interface for all task operations
3. Organize code into appropriate modules
4. Set up proper visibility between modules
5. Use external crates for:
   - Command-line argument parsing
   - Colorful terminal output
   - Date handling

**Evaluation criteria**: Is the project well-structured? Are modules organized logically? Is privacy correctly configured? Are dependencies effectively integrated? Does the application work as expected?

## How to Use These Exercises

Each exercise builds on the previous ones, progressively applying more skills from the module system. Start with Exercise 1 and work your way through. After each exercise, evaluate your work against the criteria provided.

Would you like to start with any particular exercise, or would you like me to provide a skeleton or example solution for one of them?
