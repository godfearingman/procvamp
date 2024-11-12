pub mod breakpoint {
    use std::collections::HashMap;
    // Define an enum which describes the current state the breakpoint is in
    //
    #[derive(Debug, Clone, PartialEq)]
    pub enum BreakpointState {
        Enabled,
        Disabled,
    }
    // Define an enum which describes which type of breakpoint we're dealing with
    //
    pub enum BreakpointType {
        Software { orig_byte: u8 },
        Hardware,
    }
    // Create struct for Default Breakpoint system
    //
    pub struct Breakpoint {
        pub bp_addr: u64,
        pub bp_type: BreakpointType,
        pub bp_state: BreakpointState,
    }
    // Create a trait type for our breakpoint system
    //
    pub trait BreakpointSystem {
        // Each struct should implement these functions at least.
        //
        fn add_bp(&mut self, addr: u64);
        fn remove_bp(&mut self, addr: u64);
        fn disable_bp(&mut self, addr: u64);
        fn enable_bp(&mut self, addr: u64);
        // They should also create their own hashmap of stored Breakpoints
        //
        fn get_breakpoints(&self) -> &HashMap<u64, Breakpoint>;
        fn get_breakpoints_mut(&mut self) -> &mut HashMap<u64, Breakpoint>;
        // Specific lookup functions
        //
        fn get_breakpoint(&self, key: u64) -> Option<Breakpoint>;
    }
}
