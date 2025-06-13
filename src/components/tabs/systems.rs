use bevy::prelude::ChildOf;
use bevy::prelude::*;

use super::components::{
    TabContent, TabTrigger, TabsContentMarker, TabsMarker, TabsState, TabsTriggerMarker,
};
use super::events::TabsChangedEvent;

/// Toggle the active tab when a trigger is pressed
pub fn handle_tab_trigger_clicks(
    mut q: Query<
        (Entity, &Interaction, &TabTrigger),
        (Changed<Interaction>, With<TabsTriggerMarker>),
    >,
    parents: Query<&ChildOf>,
    mut roots: Query<&mut TabsState, With<TabsMarker>>,
    mut writer: EventWriter<TabsChangedEvent>,
) {
    for (entity, interaction, trigger) in q.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let mut current = entity;
        while let Ok(parent) = parents.get(current) {
            current = parent.parent();
            if let Ok(mut state) = roots.get_mut(current) {
                state.active = Some(trigger.value.clone());
                writer.write(TabsChangedEvent {
                    tabs_entity: current,
                    value: trigger.value.clone(),
                });
                break;
            }
        }
    }
}

/// Update which content node is visible based on [`TabsState`]
pub fn update_tab_content_visibility(
    state_q: Query<(&TabsState, &Children), (Changed<TabsState>, With<TabsMarker>)>,
    mut content_q: Query<(&TabContent, &mut Visibility), With<TabsContentMarker>>,
) {
    for (state, children) in state_q.iter() {
        for child in children.iter() {
            if let Ok((content, mut vis)) = content_q.get_mut(child) {
                *vis = if Some(&content.value) == state.active.as_ref() {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}
