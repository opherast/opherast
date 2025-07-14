mod generated {
    include!(concat!(env!("OUT_DIR"), "/builtin_commands.rs"));
}

pub use generated::builtin_commands;
