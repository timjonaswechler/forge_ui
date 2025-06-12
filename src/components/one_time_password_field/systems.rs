use bevy::prelude::*;

use super::OtpInputMarker;
use crate::theme::UiTheme;

/// Highlights OTP inputs on hover.
pub fn handle_otp_input_hover(
    mut query: Query<(&Interaction, &mut BackgroundColor), With<OtpInputMarker>>,
    theme: Res<UiTheme>,
) {
    for (interaction, mut color) in query.iter_mut() {
        *color = if *interaction == Interaction::Hovered {
            BackgroundColor(theme.color.gray.step04)
        } else {
            BackgroundColor(theme.color.gray.step03)
        };
    }
}
