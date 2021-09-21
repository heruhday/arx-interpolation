use crate::entry::CommandEntry;
use crate::commands::set_scale;

pub const COMMAND_TABLE: &'static [CommandEntry] =
    &[
        CommandEntry{ name: "set_scale", func : set_scale },
    ];
