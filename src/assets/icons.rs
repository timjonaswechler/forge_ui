use bevy::asset::Asset;
use bevy::{prelude::*, reflect::TypePath};
use bevy_asset_loader::prelude::*;

use crate::assets::icon_loader::Icon;

#[derive(AssetCollection, Resource, Asset, TypePath)]
pub struct IconAssets {
    #[asset(path = "icons/accessibility.svg")]
    pub accessibility: Handle<Icon>,

    #[asset(path = "icons/activity-log.svg")]
    pub activity_log: Handle<Icon>,

    #[asset(path = "icons/align-baseline.svg")]
    pub align_baseline: Handle<Icon>,

    #[asset(path = "icons/align-bottom.svg")]
    pub align_bottom: Handle<Icon>,

    #[asset(path = "icons/align-center-horizontally.svg")]
    pub align_center_horizontally: Handle<Icon>,

    #[asset(path = "icons/align-center-vertically.svg")]
    pub align_center_vertically: Handle<Icon>,

    #[asset(path = "icons/align-left.svg")]
    pub align_left: Handle<Icon>,

    #[asset(path = "icons/align-right.svg")]
    pub align_right: Handle<Icon>,

    #[asset(path = "icons/align-top.svg")]
    pub align_top: Handle<Icon>,

    #[asset(path = "icons/all-sides.svg")]
    pub all_sides: Handle<Icon>,

    #[asset(path = "icons/angle.svg")]
    pub angle: Handle<Icon>,

    #[asset(path = "icons/archive.svg")]
    pub archive: Handle<Icon>,

    #[asset(path = "icons/arrow-bottom-left.svg")]
    pub arrow_bottom_left: Handle<Icon>,

    #[asset(path = "icons/arrow-bottom-right.svg")]
    pub arrow_bottom_right: Handle<Icon>,

    #[asset(path = "icons/arrow-down.svg")]
    pub arrow_down: Handle<Icon>,

    #[asset(path = "icons/arrow-left.svg")]
    pub arrow_left: Handle<Icon>,

    #[asset(path = "icons/arrow-right.svg")]
    pub arrow_right: Handle<Icon>,

    #[asset(path = "icons/arrow-top-left.svg")]
    pub arrow_top_left: Handle<Icon>,

    #[asset(path = "icons/arrow-top-right.svg")]
    pub arrow_top_right: Handle<Icon>,

    #[asset(path = "icons/arrow-up.svg")]
    pub arrow_up: Handle<Icon>,

    #[asset(path = "icons/aspect-ratio.svg")]
    pub aspect_ratio: Handle<Icon>,

    #[asset(path = "icons/avatar.svg")]
    pub avatar: Handle<Icon>,

    #[asset(path = "icons/backpack.svg")]
    pub backpack: Handle<Icon>,

    #[asset(path = "icons/badge.svg")]
    pub badge: Handle<Icon>,

    #[asset(path = "icons/bar-chart.svg")]
    pub bar_chart: Handle<Icon>,

    #[asset(path = "icons/bell.svg")]
    pub bell: Handle<Icon>,

    #[asset(path = "icons/blending-mode.svg")]
    pub blending_mode: Handle<Icon>,

    #[asset(path = "icons/bookmark-filled.svg")]
    pub bookmark_filled: Handle<Icon>,

    #[asset(path = "icons/bookmark.svg")]
    pub bookmark: Handle<Icon>,

    #[asset(path = "icons/border-all.svg")]
    pub border_all: Handle<Icon>,

    #[asset(path = "icons/border-bottom.svg")]
    pub border_bottom: Handle<Icon>,

    #[asset(path = "icons/border-dashed.svg")]
    pub border_dashed: Handle<Icon>,

    #[asset(path = "icons/border-dotted.svg")]
    pub border_dotted: Handle<Icon>,

    #[asset(path = "icons/border-left.svg")]
    pub border_left: Handle<Icon>,

    #[asset(path = "icons/border-none.svg")]
    pub border_none: Handle<Icon>,

    #[asset(path = "icons/border-right.svg")]
    pub border_right: Handle<Icon>,

    #[asset(path = "icons/border-solid.svg")]
    pub border_solid: Handle<Icon>,

    #[asset(path = "icons/border-split.svg")]
    pub border_split: Handle<Icon>,

    #[asset(path = "icons/border-style.svg")]
    pub border_style: Handle<Icon>,

    #[asset(path = "icons/border-top.svg")]
    pub border_top: Handle<Icon>,

    #[asset(path = "icons/border-width.svg")]
    pub border_width: Handle<Icon>,

    #[asset(path = "icons/box-model.svg")]
    pub box_model: Handle<Icon>,

    #[asset(path = "icons/button.svg")]
    pub button: Handle<Icon>,

    #[asset(path = "icons/calendar.svg")]
    pub calendar: Handle<Icon>,

    #[asset(path = "icons/camera.svg")]
    pub camera: Handle<Icon>,

    #[asset(path = "icons/card-stack-minus.svg")]
    pub card_stack_minus: Handle<Icon>,

    #[asset(path = "icons/card-stack-plus.svg")]
    pub card_stack_plus: Handle<Icon>,

    #[asset(path = "icons/card-stack.svg")]
    pub card_stack: Handle<Icon>,

    #[asset(path = "icons/caret-down.svg")]
    pub caret_down: Handle<Icon>,

    #[asset(path = "icons/caret-left.svg")]
    pub caret_left: Handle<Icon>,

    #[asset(path = "icons/caret-right.svg")]
    pub caret_right: Handle<Icon>,

    #[asset(path = "icons/caret-sort.svg")]
    pub caret_sort: Handle<Icon>,

    #[asset(path = "icons/caret-up.svg")]
    pub caret_up: Handle<Icon>,

    #[asset(path = "icons/chat-bubble.svg")]
    pub chat_bubble: Handle<Icon>,

    #[asset(path = "icons/check-circled.svg")]
    pub check_circled: Handle<Icon>,

    #[asset(path = "icons/check.svg")]
    pub check: Handle<Icon>,

    #[asset(path = "icons/checkbox.svg")]
    pub checkbox: Handle<Icon>,

    #[asset(path = "icons/chevron-down.svg")]
    pub chevron_down: Handle<Icon>,

    #[asset(path = "icons/chevron-left.svg")]
    pub chevron_left: Handle<Icon>,

    #[asset(path = "icons/chevron-right.svg")]
    pub chevron_right: Handle<Icon>,

    #[asset(path = "icons/chevron-up.svg")]
    pub chevron_up: Handle<Icon>,

    #[asset(path = "icons/circle-backslash.svg")]
    pub circle_backslash: Handle<Icon>,

    #[asset(path = "icons/circle.svg")]
    pub circle: Handle<Icon>,

    #[asset(path = "icons/clipboard-copy.svg")]
    pub clipboard_copy: Handle<Icon>,

    #[asset(path = "icons/clipboard.svg")]
    pub clipboard: Handle<Icon>,

    #[asset(path = "icons/clock.svg")]
    pub clock: Handle<Icon>,

    #[asset(path = "icons/code.svg")]
    pub code: Handle<Icon>,

    #[asset(path = "icons/codesandbox-logo.svg")]
    pub codesandbox_logo: Handle<Icon>,

    #[asset(path = "icons/color-wheel.svg")]
    pub color_wheel: Handle<Icon>,

    #[asset(path = "icons/column-spacing.svg")]
    pub column_spacing: Handle<Icon>,

    #[asset(path = "icons/columns.svg")]
    pub columns: Handle<Icon>,

    #[asset(path = "icons/commit.svg")]
    pub commit: Handle<Icon>,

    #[asset(path = "icons/component-1.svg")]
    pub component_1: Handle<Icon>,

    #[asset(path = "icons/component-2.svg")]
    pub component_2: Handle<Icon>,

    #[asset(path = "icons/component-boolean.svg")]
    pub component_boolean: Handle<Icon>,

    #[asset(path = "icons/component-instance.svg")]
    pub component_instance: Handle<Icon>,

    #[asset(path = "icons/component-none.svg")]
    pub component_none: Handle<Icon>,

    #[asset(path = "icons/component-placeholder.svg")]
    pub component_placeholder: Handle<Icon>,

    #[asset(path = "icons/container.svg")]
    pub container: Handle<Icon>,

    #[asset(path = "icons/cookie.svg")]
    pub cookie: Handle<Icon>,

    #[asset(path = "icons/copy.svg")]
    pub copy: Handle<Icon>,

    #[asset(path = "icons/corner-bottom-left.svg")]
    pub corner_bottom_left: Handle<Icon>,

    #[asset(path = "icons/corner-bottom-right.svg")]
    pub corner_bottom_right: Handle<Icon>,

    #[asset(path = "icons/corner-top-left.svg")]
    pub corner_top_left: Handle<Icon>,

    #[asset(path = "icons/corner-top-right.svg")]
    pub corner_top_right: Handle<Icon>,

    #[asset(path = "icons/corners.svg")]
    pub corners: Handle<Icon>,

    #[asset(path = "icons/countdown-timer.svg")]
    pub countdown_timer: Handle<Icon>,

    #[asset(path = "icons/counter-clockwise-clock.svg")]
    pub counter_clockwise_clock: Handle<Icon>,

    #[asset(path = "icons/crop.svg")]
    pub crop: Handle<Icon>,

    #[asset(path = "icons/cross-1.svg")]
    pub cross_1: Handle<Icon>,

    #[asset(path = "icons/cross-2.svg")]
    pub cross_2: Handle<Icon>,

    #[asset(path = "icons/cross-circled.svg")]
    pub cross_circled: Handle<Icon>,

    #[asset(path = "icons/crosshair-1.svg")]
    pub crosshair_1: Handle<Icon>,

    #[asset(path = "icons/crosshair-2.svg")]
    pub crosshair_2: Handle<Icon>,

    #[asset(path = "icons/crumpled-paper.svg")]
    pub crumpled_paper: Handle<Icon>,

    #[asset(path = "icons/cube.svg")]
    pub cube: Handle<Icon>,

    #[asset(path = "icons/cursor-arrow.svg")]
    pub cursor_arrow: Handle<Icon>,

    #[asset(path = "icons/cursor-text.svg")]
    pub cursor_text: Handle<Icon>,

    #[asset(path = "icons/dash.svg")]
    pub dash: Handle<Icon>,

    #[asset(path = "icons/dashboard.svg")]
    pub dashboard: Handle<Icon>,

    #[asset(path = "icons/desktop.svg")]
    pub desktop: Handle<Icon>,

    #[asset(path = "icons/dimensions.svg")]
    pub dimensions: Handle<Icon>,

    #[asset(path = "icons/disc.svg")]
    pub disc: Handle<Icon>,

    #[asset(path = "icons/discord-logo.svg")]
    pub discord_logo: Handle<Icon>,

    #[asset(path = "icons/divider-horizontal.svg")]
    pub divider_horizontal: Handle<Icon>,

    #[asset(path = "icons/divider-vertical.svg")]
    pub divider_vertical: Handle<Icon>,

    #[asset(path = "icons/dot-filled.svg")]
    pub dot_filled: Handle<Icon>,

    #[asset(path = "icons/dot.svg")]
    pub dot: Handle<Icon>,

    #[asset(path = "icons/dots-horizontal.svg")]
    pub dots_horizontal: Handle<Icon>,

    #[asset(path = "icons/dots-vertical.svg")]
    pub dots_vertical: Handle<Icon>,

    #[asset(path = "icons/double-arrow-down.svg")]
    pub double_arrow_down: Handle<Icon>,

    #[asset(path = "icons/double-arrow-left.svg")]
    pub double_arrow_left: Handle<Icon>,

    #[asset(path = "icons/double-arrow-right.svg")]
    pub double_arrow_right: Handle<Icon>,

    #[asset(path = "icons/double-arrow-up.svg")]
    pub double_arrow_up: Handle<Icon>,

    #[asset(path = "icons/download.svg")]
    pub download: Handle<Icon>,

    #[asset(path = "icons/drag.svg")]
    pub drag: Handle<Icon>,

    #[asset(path = "icons/drag-handle-dots-1.svg")]
    pub drag_handle_dots_1: Handle<Icon>,

    #[asset(path = "icons/drag-handle-dots-2.svg")]
    pub drag_handle_dots_2: Handle<Icon>,

    #[asset(path = "icons/drag-handle-horizontal.svg")]
    pub drag_handle_horizontal: Handle<Icon>,

    #[asset(path = "icons/drag-handle-vertical.svg")]
    pub drag_handle_vertical: Handle<Icon>,

    #[asset(path = "icons/drawing-pin-filled.svg")]
    pub drawing_pin_filled: Handle<Icon>,

    #[asset(path = "icons/drawing-pin.svg")]
    pub drawing_pin: Handle<Icon>,

    #[asset(path = "icons/dropdown-menu.svg")]
    pub dropdown_menu: Handle<Icon>,

    #[asset(path = "icons/enter-full-screen.svg")]
    pub enter_full_screen: Handle<Icon>,

    #[asset(path = "icons/enter.svg")]
    pub enter: Handle<Icon>,

    #[asset(path = "icons/envelope-closed.svg")]
    pub envelope_closed: Handle<Icon>,

    #[asset(path = "icons/envelope-open.svg")]
    pub envelope_open: Handle<Icon>,

    #[asset(path = "icons/eraser.svg")]
    pub eraser: Handle<Icon>,

    #[asset(path = "icons/exclamation-triangle.svg")]
    pub exclamation_triangle: Handle<Icon>,

    #[asset(path = "icons/exit-full-screen.svg")]
    pub exit_full_screen: Handle<Icon>,

    #[asset(path = "icons/exit.svg")]
    pub exit: Handle<Icon>,

    #[asset(path = "icons/external-link.svg")]
    pub external_link: Handle<Icon>,

    #[asset(path = "icons/eye-closed.svg")]
    pub eye_closed: Handle<Icon>,

    #[asset(path = "icons/eye-none.svg")]
    pub eye_none: Handle<Icon>,

    #[asset(path = "icons/eye-open.svg")]
    pub eye_open: Handle<Icon>,

    #[asset(path = "icons/face.svg")]
    pub face: Handle<Icon>,

    #[asset(path = "icons/figma-logo.svg")]
    pub figma_logo: Handle<Icon>,

    #[asset(path = "icons/file-minus.svg")]
    pub file_minus: Handle<Icon>,

    #[asset(path = "icons/file-plus.svg")]
    pub file_plus: Handle<Icon>,

    #[asset(path = "icons/file-text.svg")]
    pub file_text: Handle<Icon>,

    #[asset(path = "icons/file.svg")]
    pub file: Handle<Icon>,

    #[asset(path = "icons/font-bold.svg")]
    pub font_bold: Handle<Icon>,

    #[asset(path = "icons/font-family.svg")]
    pub font_family: Handle<Icon>,

    #[asset(path = "icons/font-italic.svg")]
    pub font_italic: Handle<Icon>,

    #[asset(path = "icons/font-roman.svg")]
    pub font_roman: Handle<Icon>,

    #[asset(path = "icons/font-size.svg")]
    pub font_size: Handle<Icon>,

    #[asset(path = "icons/font-style.svg")]
    pub font_style: Handle<Icon>,

    #[asset(path = "icons/frame.svg")]
    pub frame: Handle<Icon>,

    #[asset(path = "icons/framer-logo.svg")]
    pub framer_logo: Handle<Icon>,

    #[asset(path = "icons/gear.svg")]
    pub gear: Handle<Icon>,

    #[asset(path = "icons/github-logo.svg")]
    pub github_logo: Handle<Icon>,

    #[asset(path = "icons/globe.svg")]
    pub globe: Handle<Icon>,

    #[asset(path = "icons/grid.svg")]
    pub grid: Handle<Icon>,

    #[asset(path = "icons/group.svg")]
    pub group: Handle<Icon>,

    #[asset(path = "icons/half-1.svg")]
    pub half_1: Handle<Icon>,

    #[asset(path = "icons/half-2.svg")]
    pub half_2: Handle<Icon>,

    #[asset(path = "icons/hamburger-menu.svg")]
    pub hamburger_menu: Handle<Icon>,

    #[asset(path = "icons/hand.svg")]
    pub hand: Handle<Icon>,

    #[asset(path = "icons/heading.svg")]
    pub heading: Handle<Icon>,

    #[asset(path = "icons/heart-filled.svg")]
    pub heart_filled: Handle<Icon>,

    #[asset(path = "icons/heart.svg")]
    pub heart: Handle<Icon>,

    #[asset(path = "icons/height.svg")]
    pub height: Handle<Icon>,

    #[asset(path = "icons/hobby-knife.svg")]
    pub hobby_knife: Handle<Icon>,

    #[asset(path = "icons/home.svg")]
    pub home: Handle<Icon>,

    #[asset(path = "icons/iconjar-logo.svg")]
    pub iconjar_logo: Handle<Icon>,

    #[asset(path = "icons/id-card.svg")]
    pub id_card: Handle<Icon>,

    #[asset(path = "icons/image.svg")]
    pub image: Handle<Icon>,

    #[asset(path = "icons/info-circled.svg")]
    pub info_circled: Handle<Icon>,

    #[asset(path = "icons/input.svg")]
    pub input: Handle<Icon>,

    #[asset(path = "icons/instagram-logo.svg")]
    pub instagram_logo: Handle<Icon>,

    #[asset(path = "icons/keyboard.svg")]
    pub keyboard: Handle<Icon>,

    #[asset(path = "icons/lap-timer.svg")]
    pub lap_timer: Handle<Icon>,

    #[asset(path = "icons/laptop.svg")]
    pub laptop: Handle<Icon>,

    #[asset(path = "icons/layers.svg")]
    pub layers: Handle<Icon>,

    #[asset(path = "icons/layout.svg")]
    pub layout: Handle<Icon>,

    #[asset(path = "icons/letter-case-capitalize.svg")]
    pub letter_case_capitalize: Handle<Icon>,

    #[asset(path = "icons/letter-case-lowercase.svg")]
    pub letter_case_lowercase: Handle<Icon>,

    #[asset(path = "icons/letter-case-toggle.svg")]
    pub letter_case_toggle: Handle<Icon>,

    #[asset(path = "icons/letter-case-uppercase.svg")]
    pub letter_case_uppercase: Handle<Icon>,

    #[asset(path = "icons/letter-spacing.svg")]
    pub letter_spacing: Handle<Icon>,

    #[asset(path = "icons/lightning-bolt.svg")]
    pub lightning_bolt: Handle<Icon>,

    #[asset(path = "icons/line-height.svg")]
    pub line_height: Handle<Icon>,

    #[asset(path = "icons/link-1.svg")]
    pub link_1: Handle<Icon>,

    #[asset(path = "icons/link-2.svg")]
    pub link_2: Handle<Icon>,

    #[asset(path = "icons/link-break-1.svg")]
    pub link_break_1: Handle<Icon>,

    #[asset(path = "icons/link-break-2.svg")]
    pub link_break_2: Handle<Icon>,

    #[asset(path = "icons/link-none-1.svg")]
    pub link_none_1: Handle<Icon>,

    #[asset(path = "icons/link-none-2.svg")]
    pub link_none_2: Handle<Icon>,

    #[asset(path = "icons/linkedin-logo.svg")]
    pub linkedin_logo: Handle<Icon>,

    #[asset(path = "icons/list-bullet.svg")]
    pub list_bullet: Handle<Icon>,

    #[asset(path = "icons/lock-closed.svg")]
    pub lock_closed: Handle<Icon>,

    #[asset(path = "icons/lock-open-1.svg")]
    pub lock_open_1: Handle<Icon>,

    #[asset(path = "icons/lock-open-2.svg")]
    pub lock_open_2: Handle<Icon>,

    #[asset(path = "icons/loop.svg")]
    pub loop_icon: Handle<Icon>,

    #[asset(path = "icons/magic-wand.svg")]
    pub magic_wand: Handle<Icon>,

    #[asset(path = "icons/magnifying-glass.svg")]
    pub magnifying_glass: Handle<Icon>,

    #[asset(path = "icons/margin.svg")]
    pub margin: Handle<Icon>,

    #[asset(path = "icons/mask-off.svg")]
    pub mask_off: Handle<Icon>,

    #[asset(path = "icons/mask-on.svg")]
    pub mask_on: Handle<Icon>,

    #[asset(path = "icons/minus-circled.svg")]
    pub minus_circled: Handle<Icon>,

    #[asset(path = "icons/minus.svg")]
    pub minus: Handle<Icon>,

    #[asset(path = "icons/mix.svg")]
    pub mix: Handle<Icon>,

    #[asset(path = "icons/mixer-horizontal.svg")]
    pub mixer_horizontal: Handle<Icon>,

    #[asset(path = "icons/mixer-vertical.svg")]
    pub mixer_vertical: Handle<Icon>,

    #[asset(path = "icons/mobile.svg")]
    pub mobile: Handle<Icon>,

    #[asset(path = "icons/modulz-logo.svg")]
    pub modulz_logo: Handle<Icon>,

    #[asset(path = "icons/moon.svg")]
    pub moon: Handle<Icon>,

    #[asset(path = "icons/notion-logo.svg")]
    pub notion_logo: Handle<Icon>,

    #[asset(path = "icons/opacity.svg")]
    pub opacity: Handle<Icon>,

    #[asset(path = "icons/open-in-new-window.svg")]
    pub open_in_new_window: Handle<Icon>,

    #[asset(path = "icons/overline.svg")]
    pub overline: Handle<Icon>,

    #[asset(path = "icons/padding.svg")]
    pub padding: Handle<Icon>,

    #[asset(path = "icons/paper-plane.svg")]
    pub paper_plane: Handle<Icon>,

    #[asset(path = "icons/pause.svg")]
    pub pause: Handle<Icon>,

    #[asset(path = "icons/pencil-1.svg")]
    pub pencil_1: Handle<Icon>,

    #[asset(path = "icons/pencil-2.svg")]
    pub pencil_2: Handle<Icon>,

    #[asset(path = "icons/person.svg")]
    pub person: Handle<Icon>,

    #[asset(path = "icons/pie-chart.svg")]
    pub pie_chart: Handle<Icon>,

    #[asset(path = "icons/pilcrow.svg")]
    pub pilcrow: Handle<Icon>,

    #[asset(path = "icons/pin-bottom.svg")]
    pub pin_bottom: Handle<Icon>,

    #[asset(path = "icons/pin-left.svg")]
    pub pin_left: Handle<Icon>,

    #[asset(path = "icons/pin-right.svg")]
    pub pin_right: Handle<Icon>,

    #[asset(path = "icons/pin-top.svg")]
    pub pin_top: Handle<Icon>,

    #[asset(path = "icons/play.svg")]
    pub play: Handle<Icon>,

    #[asset(path = "icons/plus-circled.svg")]
    pub plus_circled: Handle<Icon>,

    #[asset(path = "icons/plus.svg")]
    pub plus: Handle<Icon>,

    #[asset(path = "icons/question-mark-circled.svg")]
    pub question_mark_circled: Handle<Icon>,

    #[asset(path = "icons/question-mark.svg")]
    pub question_mark: Handle<Icon>,

    #[asset(path = "icons/quote.svg")]
    pub quote: Handle<Icon>,

    #[asset(path = "icons/radiobutton.svg")]
    pub radiobutton: Handle<Icon>,

    #[asset(path = "icons/reader.svg")]
    pub reader: Handle<Icon>,

    #[asset(path = "icons/rectangle.svg")]
    pub rectangle: Handle<Icon>,

    #[asset(path = "icons/reload.svg")]
    pub reload: Handle<Icon>,

    #[asset(path = "icons/reset.svg")]
    pub reset: Handle<Icon>,

    #[asset(path = "icons/resume.svg")]
    pub resume: Handle<Icon>,

    #[asset(path = "icons/rocket.svg")]
    pub rocket: Handle<Icon>,

    #[asset(path = "icons/rotate-counter-clockwise.svg")]
    pub rotate_counter_clockwise: Handle<Icon>,

    #[asset(path = "icons/row-spacing.svg")]
    pub row_spacing: Handle<Icon>,

    #[asset(path = "icons/rows.svg")]
    pub rows: Handle<Icon>,

    #[asset(path = "icons/ruler-horizontal.svg")]
    pub ruler_horizontal: Handle<Icon>,

    #[asset(path = "icons/ruler-square.svg")]
    pub ruler_square: Handle<Icon>,

    #[asset(path = "icons/scissors.svg")]
    pub scissors: Handle<Icon>,

    #[asset(path = "icons/section.svg")]
    pub section: Handle<Icon>,

    #[asset(path = "icons/sewing-pin-filled.svg")]
    pub sewing_pin_filled: Handle<Icon>,

    #[asset(path = "icons/sewing-pin.svg")]
    pub sewing_pin: Handle<Icon>,

    #[asset(path = "icons/shadow-inner.svg")]
    pub shadow_inner: Handle<Icon>,

    #[asset(path = "icons/shadow-none.svg")]
    pub shadow_none: Handle<Icon>,

    #[asset(path = "icons/shadow-outer.svg")]
    pub shadow_outer: Handle<Icon>,

    #[asset(path = "icons/shadow.svg")]
    pub shadow: Handle<Icon>,

    #[asset(path = "icons/share-1.svg")]
    pub share_1: Handle<Icon>,

    #[asset(path = "icons/share-2.svg")]
    pub share_2: Handle<Icon>,

    #[asset(path = "icons/shuffle.svg")]
    pub shuffle: Handle<Icon>,

    #[asset(path = "icons/size.svg")]
    pub size: Handle<Icon>,

    #[asset(path = "icons/sketch-logo.svg")]
    pub sketch_logo: Handle<Icon>,

    #[asset(path = "icons/slash.svg")]
    pub slash: Handle<Icon>,

    #[asset(path = "icons/slider.svg")]
    pub slider: Handle<Icon>,

    #[asset(path = "icons/space-between-horizontally.svg")]
    pub space_between_horizontally: Handle<Icon>,

    #[asset(path = "icons/space-between-vertically.svg")]
    pub space_between_vertically: Handle<Icon>,

    #[asset(path = "icons/space-evenly-horizontally.svg")]
    pub space_evenly_horizontally: Handle<Icon>,

    #[asset(path = "icons/space-evenly-vertically.svg")]
    pub space_evenly_vertically: Handle<Icon>,

    #[asset(path = "icons/speaker-loud.svg")]
    pub speaker_loud: Handle<Icon>,

    #[asset(path = "icons/speaker-moderate.svg")]
    pub speaker_moderate: Handle<Icon>,

    #[asset(path = "icons/speaker-off.svg")]
    pub speaker_off: Handle<Icon>,

    #[asset(path = "icons/speaker-quiet.svg")]
    pub speaker_quiet: Handle<Icon>,

    #[asset(path = "icons/square.svg")]
    pub square: Handle<Icon>,

    #[asset(path = "icons/stack.svg")]
    pub stack: Handle<Icon>,

    #[asset(path = "icons/star-filled.svg")]
    pub star_filled: Handle<Icon>,

    #[asset(path = "icons/star.svg")]
    pub star: Handle<Icon>,

    #[asset(path = "icons/stitches-logo.svg")]
    pub stitches_logo: Handle<Icon>,

    #[asset(path = "icons/stop.svg")]
    pub stop: Handle<Icon>,

    #[asset(path = "icons/stopwatch.svg")]
    pub stopwatch: Handle<Icon>,

    #[asset(path = "icons/stretch-horizontally.svg")]
    pub stretch_horizontally: Handle<Icon>,

    #[asset(path = "icons/stretch-vertically.svg")]
    pub stretch_vertically: Handle<Icon>,

    #[asset(path = "icons/strikethrough.svg")]
    pub strikethrough: Handle<Icon>,

    #[asset(path = "icons/sun.svg")]
    pub sun: Handle<Icon>,

    #[asset(path = "icons/switch.svg")]
    pub switch_icon: Handle<Icon>,

    #[asset(path = "icons/symbol.svg")]
    pub symbol: Handle<Icon>,

    #[asset(path = "icons/table.svg")]
    pub table: Handle<Icon>,

    #[asset(path = "icons/target.svg")]
    pub target: Handle<Icon>,

    #[asset(path = "icons/text-align-bottom.svg")]
    pub text_align_bottom: Handle<Icon>,

    #[asset(path = "icons/text-align-center.svg")]
    pub text_align_center: Handle<Icon>,

    #[asset(path = "icons/text-align-justify.svg")]
    pub text_align_justify: Handle<Icon>,

    #[asset(path = "icons/text-align-left.svg")]
    pub text_align_left: Handle<Icon>,

    #[asset(path = "icons/text-align-middle.svg")]
    pub text_align_middle: Handle<Icon>,

    #[asset(path = "icons/text-align-right.svg")]
    pub text_align_right: Handle<Icon>,

    #[asset(path = "icons/text-align-top.svg")]
    pub text_align_top: Handle<Icon>,

    #[asset(path = "icons/text-none.svg")]
    pub text_none: Handle<Icon>,

    #[asset(path = "icons/text.svg")]
    pub text: Handle<Icon>,

    #[asset(path = "icons/thick-arrow-down.svg")]
    pub thick_arrow_down: Handle<Icon>,

    #[asset(path = "icons/thick-arrow-left.svg")]
    pub thick_arrow_left: Handle<Icon>,

    #[asset(path = "icons/thick-arrow-right.svg")]
    pub thick_arrow_right: Handle<Icon>,

    #[asset(path = "icons/thick-arrow-up.svg")]
    pub thick_arrow_up: Handle<Icon>,

    #[asset(path = "icons/timer.svg")]
    pub timer: Handle<Icon>,

    #[asset(path = "icons/tokens.svg")]
    pub tokens: Handle<Icon>,

    #[asset(path = "icons/track-next.svg")]
    pub track_next: Handle<Icon>,

    #[asset(path = "icons/track-previous.svg")]
    pub track_previous: Handle<Icon>,

    #[asset(path = "icons/transform.svg")]
    pub transform: Handle<Icon>,

    #[asset(path = "icons/transparency-grid.svg")]
    pub transparency_grid: Handle<Icon>,

    #[asset(path = "icons/trash.svg")]
    pub trash: Handle<Icon>,

    #[asset(path = "icons/triangle-down.svg")]
    pub triangle_down: Handle<Icon>,

    #[asset(path = "icons/triangle-left.svg")]
    pub triangle_left: Handle<Icon>,

    #[asset(path = "icons/triangle-right.svg")]
    pub triangle_right: Handle<Icon>,

    #[asset(path = "icons/triangle-up.svg")]
    pub triangle_up: Handle<Icon>,

    #[asset(path = "icons/twitter-logo.svg")]
    pub twitter_logo: Handle<Icon>,

    #[asset(path = "icons/underline.svg")]
    pub underline: Handle<Icon>,

    #[asset(path = "icons/update.svg")]
    pub update: Handle<Icon>,

    #[asset(path = "icons/upload.svg")]
    pub upload: Handle<Icon>,

    #[asset(path = "icons/value-none.svg")]
    pub value_none: Handle<Icon>,

    #[asset(path = "icons/value.svg")]
    pub value: Handle<Icon>,

    #[asset(path = "icons/vercel-logo.svg")]
    pub vercel_logo: Handle<Icon>,

    #[asset(path = "icons/video.svg")]
    pub video: Handle<Icon>,

    #[asset(path = "icons/view-grid.svg")]
    pub view_grid: Handle<Icon>,

    #[asset(path = "icons/view-horizontal.svg")]
    pub view_horizontal: Handle<Icon>,

    #[asset(path = "icons/view-none.svg")]
    pub view_none: Handle<Icon>,

    #[asset(path = "icons/view-vertical.svg")]
    pub view_vertical: Handle<Icon>,

    #[asset(path = "icons/width.svg")]
    pub width: Handle<Icon>,

    #[asset(path = "icons/zoom-in.svg")]
    pub zoom_in: Handle<Icon>,

    #[asset(path = "icons/zoom-out.svg")]
    pub zoom_out: Handle<Icon>,
}
