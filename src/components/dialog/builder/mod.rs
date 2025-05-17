use crate::theme::UiTheme;
use bevy::prelude::*;

mod body;
mod builder;
mod close;
mod content;
mod footer;
mod header;
mod trigger;

pub use body::DialogBodyBuilder;
pub use builder::DialogBuilder;
pub use close::DialogCloseButtonBuilder;
pub use content::DialogContentBuilder;
pub use footer::DialogFooterBuilder;
pub use header::DialogHeaderBuilder;
pub use trigger::DialogTriggerBuilder;

type SectionElementBuilderFn =
    Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static>;
