use std::cell::RefCell;

// Define the CommandHandler trait
pub trait CommandHandler {
    fn handle_command(&self);
}

// Define the CommandRegistry struct
pub struct CommandRegistry<'a> {
    capacity: usize,
    size: usize,
    keys: [u32; 10],
    values: [Option<&'a RefCell<dyn CommandHandler>>; 10],
}

impl<'a> CommandRegistry<'a> {
    // Method to create a new CommandRegistry
    pub fn new() -> Self {
        CommandRegistry {
            capacity: 10,
            size: 0,
            keys: [0; 10],
            values: [None; 10],
        }
    }

    // Method to register a command handler for a specific command ID
    pub fn register_handler(
        &mut self,
        command_id: u32,
        handler: &'a RefCell<dyn CommandHandler>,
    ) -> Result<(), &'static str> {
        if self.size >= self.capacity {
            return Err("Command registry is full. Cannot register more handlers.");
        }

        let index = self.size;
        self.keys[index] = command_id;
        self.values[index] = Some(handler);
        self.size += 1;

        Ok(())
    }

    // Method to handle a command based on its ID
    pub fn handle_command(&self, command_id: u32) -> Result<(), &'static str> {
        for i in 0..self.size {
            if let Some(handler) = self.values[i] {
                if self.keys[i] == command_id {
                    handler.borrow().handle_command();
                    return Ok(());
                }
            }
        }

        Err("No handler found for command ID")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std:cell::Cell;

    // Define a struct that implements the CommandHandler trait
    struct CommandHandler1 {
        count: Cell<u32>,
    }

    impl CommandHandler for CommandHandler1 {
        fn handle_command(&self) {
            self.count.set(self.count.get() + 1);
        }
    }

    #[derive(Default)]
    struct CommandHandler2 {
        count: Cell<u32>,
    }

    impl CommandHandler for CommandHandler2 {
        fn handle_command(&self) {
            self.count.set(self.count.get() + 1);
        }
    }

    #[test]
    fn test_command_registry() {
        // Create a CommandRegistry instance
        let mut registry = CommandRegistry::new();

        // Create instances of command handlers
        let handler1 = RefCell::new(CommandHandler1 {
            count: Cell::new(0),
        });
        let handler2 = RefCell::new(CommandHandler2::default());

        // Register command handlers
        assert!(registry.register_handler(1, &handler1).is_ok());
        assert!(registry.register_handler(2, &handler2).is_ok());

        // Handle commands
        assert!(registry.handle_command(1).is_ok()); // Output: Handler for Command ID 1 called
        assert!(registry.handle_command(2).is_ok()); // Output: Handler for Command ID 2 called
        assert!(registry.handle_command(3).is_err()); // Output: No handler found for command ID: 3

        assert_eq!(handler1.borrow().count.get(), 1);
        assert_eq!(handler2.borrow().count.get(), 1);
    }
}
