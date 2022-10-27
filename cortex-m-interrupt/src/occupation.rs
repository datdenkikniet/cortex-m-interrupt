use crate::{ExceptionRegistration, InterruptRegistration};

/// An interrupt occupation
pub trait InterruptOccupation {
    fn execute();
}

// A marker trait that indicates that a registration is already occupied.
// Calling register on this InterruptRegistration is a no-op
pub trait OccupiedRegistration<T: InterruptOccupation>: InterruptRegistration {}

struct MyPeripheral;

impl InterruptOccupation for MyPeripheral {
    fn execute() {
        // Update some bits.
    }
}

fn tests() {
    let occupied_registration = {
        struct Handle;

        impl InterruptRegistration for Handle {
            fn occupy(self, _f: fn()) {
                // TODO: do nothing, but do panic if `register` is called
                // more than once.
                //
                // TODO: figure out how we can justify discarding `f` entirely...
            }
        }

        impl OccupiedRegistration<MyPeripheral> for Handle {}

        Handle
    };
}
