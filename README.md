# state_machine

amethyst.rs state machine generator macro.
Made exactly for abstracting over `update/handle_events` method parameters.

## Usage:
`state_machine!(StateMachine; State; _world: &mut World, _something: &mut f64);`

Will generate StateMachine struct and trait for using with State trait implementations all methods of which should take `&mut World` and `&mut f64` as parameters.
