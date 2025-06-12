use bevy::prelude::*;

use super::*;
use crate::theme::UiTheme;

pub fn handle_tab_interaction(
    theme_opt: Option<Res<UiTheme>>,
    mut trigger_q: Query<(&Interaction, &TabTriggerMarker), Changed<Interaction>>,
    mut tabs_q: Query<(&mut TabsState, &Children), With<TabsMarker>>,
    mut style_q: Query<(&mut BackgroundColor, &TabTriggerMarker)>,
    mut content_q: Query<(&TabContentMarker, &mut Visibility)>,
) {
    let theme = if let Some(t) = theme_opt { t } else { return; };

    for (interaction, trigger) in trigger_q.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Ok((mut state, children)) = tabs_q.get_mut(trigger.tabs_entity) {
                state.selected = trigger.index;
                for child in children.iter() {
                    if let Ok((mut bg, trig_marker)) = style_q.get_mut(child) {
                        *bg = if trig_marker.index == state.selected {
                            theme.accent.step09.into()
                        } else {
                            theme.color.gray.step03.into()
                        };
                    }
                    if let Ok((content_marker, mut vis)) = content_q.get_mut(child) {
                        *vis = if content_marker.index == state.selected {
                            Visibility::Inherited
                        } else {
                            Visibility::Hidden
                        };
                    }
                }
            }
        }
    }
}
