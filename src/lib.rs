pub mod config {
    pub mod m1 {
        pub const N_MODE: usize = 27;
        pub const N_RAW_MODE: usize = 335;
        pub const MODES: &str = "20230530_1756_m1_bending_modes";
        pub const RAW_MODES: &str = "20230530_1756_m1_raw_bending_modes";
        pub const ACTUATOR_RATE: usize = 10;
    }
    pub mod agws {
        pub mod sh24 {
            pub const RATE: usize = 5;
            pub const INTEGRATOR_GAIN: f64 = 0.2;
        }
        pub mod sh48 {
            pub const RATE: usize = 5000;
        }
    }
}
