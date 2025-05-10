use bevy::asset::Asset;
use bevy::{prelude::*, reflect::TypePath};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Asset, TypePath)]
pub struct IconAssets {
    #[asset(path = "icons/accessibility.svg")]
    pub accessibility: Handle<Image>,

    #[asset(path = "icons/activity-log.svg")]
    pub activity_log: Handle<Image>,

    #[asset(path = "icons/align-baseline.svg")]
    pub align_baseline: Handle<Image>,

    #[asset(path = "icons/align-bottom.svg")]
    pub align_bottom: Handle<Image>,

    #[asset(path = "icons/align-center-horizontally.svg")]
    pub align_center_horizontally: Handle<Image>,

    #[asset(path = "icons/align-center-vertically.svg")]
    pub align_center_vertically: Handle<Image>,

    #[asset(path = "icons/align-left.svg")]
    pub align_left: Handle<Image>,

    #[asset(path = "icons/align-right.svg")]
    pub align_right: Handle<Image>,

    #[asset(path = "icons/align-top.svg")]
    pub align_top: Handle<Image>,

    #[asset(path = "icons/all-sides.svg")]
    pub all_sides: Handle<Image>,

    #[asset(path = "icons/angle.svg")]
    pub angle: Handle<Image>,

    #[asset(path = "icons/archive.svg")]
    pub archive: Handle<Image>,

    #[asset(path = "icons/arrow-bottom-left.svg")]
    pub arrow_bottom_left: Handle<Image>,

    #[asset(path = "icons/arrow-bottom-right.svg")]
    pub arrow_bottom_right: Handle<Image>,

    #[asset(path = "icons/arrow-down.svg")]
    pub arrow_down: Handle<Image>,

    #[asset(path = "icons/arrow-left.svg")]
    pub arrow_left: Handle<Image>,

    #[asset(path = "icons/arrow-right.svg")]
    pub arrow_right: Handle<Image>,

    #[asset(path = "icons/arrow-top-left.svg")]
    pub arrow_top_left: Handle<Image>,

    #[asset(path = "icons/arrow-top-right.svg")]
    pub arrow_top_right: Handle<Image>,

    #[asset(path = "icons/arrow-up.svg")]
    pub arrow_up: Handle<Image>,

    #[asset(path = "icons/aspect-ratio.svg")]
    pub aspect_ratio: Handle<Image>,

    #[asset(path = "icons/avatar.svg")]
    pub avatar: Handle<Image>,

    #[asset(path = "icons/backpack.svg")]
    pub backpack: Handle<Image>,

    #[asset(path = "icons/badge.svg")]
    pub badge: Handle<Image>,

    #[asset(path = "icons/bar-chart.svg")]
    pub bar_chart: Handle<Image>,

    #[asset(path = "icons/bell.svg")]
    pub bell: Handle<Image>,

    #[asset(path = "icons/blending-mode.svg")]
    pub blending_mode: Handle<Image>,

    #[asset(path = "icons/bookmark-filled.svg")]
    pub bookmark_filled: Handle<Image>,

    #[asset(path = "icons/bookmark.svg")]
    pub bookmark: Handle<Image>,

    #[asset(path = "icons/border-all.svg")]
    pub border_all: Handle<Image>,

    #[asset(path = "icons/border-bottom.svg")]
    pub border_bottom: Handle<Image>,

    #[asset(path = "icons/border-dashed.svg")]
    pub border_dashed: Handle<Image>,

    #[asset(path = "icons/border-dotted.svg")]
    pub border_dotted: Handle<Image>,

    #[asset(path = "icons/border-left.svg")]
    pub border_left: Handle<Image>,

    #[asset(path = "icons/border-none.svg")]
    pub border_none: Handle<Image>,

    #[asset(path = "icons/border-right.svg")]
    pub border_right: Handle<Image>,

    #[asset(path = "icons/border-solid.svg")]
    pub border_solid: Handle<Image>,

    #[asset(path = "icons/border-split.svg")]
    pub border_split: Handle<Image>,

    #[asset(path = "icons/border-style.svg")]
    pub border_style: Handle<Image>,

    #[asset(path = "icons/border-top.svg")]
    pub border_top: Handle<Image>,

    #[asset(path = "icons/border-width.svg")]
    pub border_width: Handle<Image>,

    #[asset(path = "icons/box-model.svg")]
    pub box_model: Handle<Image>,

    #[asset(path = "icons/button.svg")]
    pub button: Handle<Image>,

    #[asset(path = "icons/calendar.svg")]
    pub calendar: Handle<Image>,

    #[asset(path = "icons/camera.svg")]
    pub camera: Handle<Image>,

    #[asset(path = "icons/card-stack-minus.svg")]
    pub card_stack_minus: Handle<Image>,

    #[asset(path = "icons/card-stack-plus.svg")]
    pub card_stack_plus: Handle<Image>,

    #[asset(path = "icons/card-stack.svg")]
    pub card_stack: Handle<Image>,

    #[asset(path = "icons/caret-down.svg")]
    pub caret_down: Handle<Image>,

    #[asset(path = "icons/caret-left.svg")]
    pub caret_left: Handle<Image>,

    #[asset(path = "icons/caret-right.svg")]
    pub caret_right: Handle<Image>,

    #[asset(path = "icons/caret-sort.svg")]
    pub caret_sort: Handle<Image>,

    #[asset(path = "icons/caret-up.svg")]
    pub caret_up: Handle<Image>,

    #[asset(path = "icons/chat-bubble.svg")]
    pub chat_bubble: Handle<Image>,

    #[asset(path = "icons/check-circled.svg")]
    pub check_circled: Handle<Image>,

    #[asset(path = "icons/check.svg")]
    pub check: Handle<Image>,

    #[asset(path = "icons/checkbox.svg")]
    pub checkbox: Handle<Image>,

    #[asset(path = "icons/chevron-down.svg")]
    pub chevron_down: Handle<Image>,

    #[asset(path = "icons/chevron-left.svg")]
    pub chevron_left: Handle<Image>,

    #[asset(path = "icons/chevron-right.svg")]
    pub chevron_right: Handle<Image>,

    #[asset(path = "icons/chevron-up.svg")]
    pub chevron_up: Handle<Image>,

    #[asset(path = "icons/circle-backslash.svg")]
    pub circle_backslash: Handle<Image>,

    #[asset(path = "icons/circle.svg")]
    pub circle: Handle<Image>,

    #[asset(path = "icons/clipboard-copy.svg")]
    pub clipboard_copy: Handle<Image>,

    #[asset(path = "icons/clipboard.svg")]
    pub clipboard: Handle<Image>,

    #[asset(path = "icons/clock.svg")]
    pub clock: Handle<Image>,

    #[asset(path = "icons/code.svg")]
    pub code: Handle<Image>,

    #[asset(path = "icons/codesandbox-logo.svg")]
    pub codesandbox_logo: Handle<Image>,

    #[asset(path = "icons/color-wheel.svg")]
    pub color_wheel: Handle<Image>,

    #[asset(path = "icons/column-spacing.svg")]
    pub column_spacing: Handle<Image>,

    #[asset(path = "icons/columns.svg")]
    pub columns: Handle<Image>,

    #[asset(path = "icons/commit.svg")]
    pub commit: Handle<Image>,

    #[asset(path = "icons/component-1.svg")]
    pub component_1: Handle<Image>,

    #[asset(path = "icons/component-2.svg")]
    pub component_2: Handle<Image>,

    #[asset(path = "icons/component-boolean.svg")]
    pub component_boolean: Handle<Image>,

    #[asset(path = "icons/component-instance.svg")]
    pub component_instance: Handle<Image>,

    #[asset(path = "icons/component-none.svg")]
    pub component_none: Handle<Image>,

    #[asset(path = "icons/component-placeholder.svg")]
    pub component_placeholder: Handle<Image>,

    #[asset(path = "icons/container.svg")]
    pub container: Handle<Image>,

    #[asset(path = "icons/cookie.svg")]
    pub cookie: Handle<Image>,

    #[asset(path = "icons/copy.svg")]
    pub copy: Handle<Image>,

    #[asset(path = "icons/corner-bottom-left.svg")]
    pub corner_bottom_left: Handle<Image>,

    #[asset(path = "icons/corner-bottom-right.svg")]
    pub corner_bottom_right: Handle<Image>,

    #[asset(path = "icons/corner-top-left.svg")]
    pub corner_top_left: Handle<Image>,

    #[asset(path = "icons/corner-top-right.svg")]
    pub corner_top_right: Handle<Image>,

    #[asset(path = "icons/corners.svg")]
    pub corners: Handle<Image>,

    #[asset(path = "icons/countdown-timer.svg")]
    pub countdown_timer: Handle<Image>,

    #[asset(path = "icons/counter-clockwise-clock.svg")]
    pub counter_clockwise_clock: Handle<Image>,

    #[asset(path = "icons/crop.svg")]
    pub crop: Handle<Image>,

    #[asset(path = "icons/cross-1.svg")]
    pub cross_1: Handle<Image>,

    #[asset(path = "icons/cross-2.svg")]
    pub cross_2: Handle<Image>,

    #[asset(path = "icons/cross-circled.svg")]
    pub cross_circled: Handle<Image>,

    #[asset(path = "icons/crosshair-1.svg")]
    pub crosshair_1: Handle<Image>,

    #[asset(path = "icons/crosshair-2.svg")]
    pub crosshair_2: Handle<Image>,

    #[asset(path = "icons/crumpled-paper.svg")]
    pub crumpled_paper: Handle<Image>,

    #[asset(path = "icons/cube.svg")]
    pub cube: Handle<Image>,

    #[asset(path = "icons/cursor-arrow.svg")]
    pub cursor_arrow: Handle<Image>,

    #[asset(path = "icons/cursor-text.svg")]
    pub cursor_text: Handle<Image>,

    #[asset(path = "icons/dash.svg")]
    pub dash: Handle<Image>,

    #[asset(path = "icons/dashboard.svg")]
    pub dashboard: Handle<Image>,

    #[asset(path = "icons/desktop.svg")]
    pub desktop: Handle<Image>,

    #[asset(path = "icons/dimensions.svg")]
    pub dimensions: Handle<Image>,

    #[asset(path = "icons/disc.svg")]
    pub disc: Handle<Image>,

    #[asset(path = "icons/discord-logo.svg")]
    pub discord_logo: Handle<Image>,

    #[asset(path = "icons/divider-horizontal.svg")]
    pub divider_horizontal: Handle<Image>,

    #[asset(path = "icons/divider-vertical.svg")]
    pub divider_vertical: Handle<Image>,

    #[asset(path = "icons/dot-filled.svg")]
    pub dot_filled: Handle<Image>,

    #[asset(path = "icons/dot.svg")]
    pub dot: Handle<Image>,

    #[asset(path = "icons/dots-horizontal.svg")]
    pub dots_horizontal: Handle<Image>,

    #[asset(path = "icons/dots-vertical.svg")]
    pub dots_vertical: Handle<Image>,

    #[asset(path = "icons/double-arrow-down.svg")]
    pub double_arrow_down: Handle<Image>,

    #[asset(path = "icons/double-arrow-left.svg")]
    pub double_arrow_left: Handle<Image>,

    #[asset(path = "icons/double-arrow-right.svg")]
    pub double_arrow_right: Handle<Image>,

    #[asset(path = "icons/double-arrow-up.svg")]
    pub double_arrow_up: Handle<Image>,

    #[asset(path = "icons/download.svg")]
    pub download: Handle<Image>,

    #[asset(path = "icons/drag.svg")]
    pub drag: Handle<Image>,

    #[asset(path = "icons/drag-handle-dots-1.svg")]
    pub drag_handle_dots_1: Handle<Image>,

    #[asset(path = "icons/drag-handle-dots-2.svg")]
    pub drag_handle_dots_2: Handle<Image>,

    #[asset(path = "icons/drag-handle-horizontal.svg")]
    pub drag_handle_horizontal: Handle<Image>,

    #[asset(path = "icons/drag-handle-vertical.svg")]
    pub drag_handle_vertical: Handle<Image>,

    #[asset(path = "icons/drawing-pin-filled.svg")]
    pub drawing_pin_filled: Handle<Image>,

    #[asset(path = "icons/drawing-pin.svg")]
    pub drawing_pin: Handle<Image>,

    #[asset(path = "icons/dropdown-menu.svg")]
    pub dropdown_menu: Handle<Image>,

    #[asset(path = "icons/enter-full-screen.svg")]
    pub enter_full_screen: Handle<Image>,

    #[asset(path = "icons/enter.svg")]
    pub enter: Handle<Image>,

    #[asset(path = "icons/envelope-closed.svg")]
    pub envelope_closed: Handle<Image>,

    #[asset(path = "icons/envelope-open.svg")]
    pub envelope_open: Handle<Image>,

    #[asset(path = "icons/eraser.svg")]
    pub eraser: Handle<Image>,

    #[asset(path = "icons/exclamation-triangle.svg")]
    pub exclamation_triangle: Handle<Image>,

    #[asset(path = "icons/exit-full-screen.svg")]
    pub exit_full_screen: Handle<Image>,

    #[asset(path = "icons/exit.svg")]
    pub exit: Handle<Image>,

    #[asset(path = "icons/external-link.svg")]
    pub external_link: Handle<Image>,

    #[asset(path = "icons/eye-closed.svg")]
    pub eye_closed: Handle<Image>,

    #[asset(path = "icons/eye-none.svg")]
    pub eye_none: Handle<Image>,

    #[asset(path = "icons/eye-open.svg")]
    pub eye_open: Handle<Image>,

    #[asset(path = "icons/face.svg")]
    pub face: Handle<Image>,

    #[asset(path = "icons/figma-logo.svg")]
    pub figma_logo: Handle<Image>,

    #[asset(path = "icons/file-minus.svg")]
    pub file_minus: Handle<Image>,

    #[asset(path = "icons/file-plus.svg")]
    pub file_plus: Handle<Image>,

    #[asset(path = "icons/file-text.svg")]
    pub file_text: Handle<Image>,

    #[asset(path = "icons/file.svg")]
    pub file: Handle<Image>,

    #[asset(path = "icons/font-bold.svg")]
    pub font_bold: Handle<Image>,

    #[asset(path = "icons/font-family.svg")]
    pub font_family: Handle<Image>,

    #[asset(path = "icons/font-italic.svg")]
    pub font_italic: Handle<Image>,

    #[asset(path = "icons/font-roman.svg")]
    pub font_roman: Handle<Image>,

    #[asset(path = "icons/font-size.svg")]
    pub font_size: Handle<Image>,

    #[asset(path = "icons/font-style.svg")]
    pub font_style: Handle<Image>,

    #[asset(path = "icons/frame.svg")]
    pub frame: Handle<Image>,

    #[asset(path = "icons/framer-logo.svg")]
    pub framer_logo: Handle<Image>,

    #[asset(path = "icons/gear.svg")]
    pub gear: Handle<Image>,

    #[asset(path = "icons/github-logo.svg")]
    pub github_logo: Handle<Image>,

    #[asset(path = "icons/globe.svg")]
    pub globe: Handle<Image>,

    #[asset(path = "icons/grid.svg")]
    pub grid: Handle<Image>,

    #[asset(path = "icons/group.svg")]
    pub group: Handle<Image>,

    #[asset(path = "icons/half-1.svg")]
    pub half_1: Handle<Image>,

    #[asset(path = "icons/half-2.svg")]
    pub half_2: Handle<Image>,

    #[asset(path = "icons/hamburger-menu.svg")]
    pub hamburger_menu: Handle<Image>,

    #[asset(path = "icons/hand.svg")]
    pub hand: Handle<Image>,

    #[asset(path = "icons/heading.svg")]
    pub heading: Handle<Image>,

    #[asset(path = "icons/heart-filled.svg")]
    pub heart_filled: Handle<Image>,

    #[asset(path = "icons/heart.svg")]
    pub heart: Handle<Image>,

    #[asset(path = "icons/height.svg")]
    pub height: Handle<Image>,

    #[asset(path = "icons/hobby-knife.svg")]
    pub hobby_knife: Handle<Image>,

    #[asset(path = "icons/home.svg")]
    pub home: Handle<Image>,

    #[asset(path = "icons/iconjar-logo.svg")]
    pub iconjar_logo: Handle<Image>,

    #[asset(path = "icons/id-card.svg")]
    pub id_card: Handle<Image>,

    #[asset(path = "icons/image.svg")]
    pub image: Handle<Image>,

    #[asset(path = "icons/info-circled.svg")]
    pub info_circled: Handle<Image>,

    #[asset(path = "icons/input.svg")]
    pub input: Handle<Image>,

    #[asset(path = "icons/instagram-logo.svg")]
    pub instagram_logo: Handle<Image>,

    #[asset(path = "icons/keyboard.svg")]
    pub keyboard: Handle<Image>,

    #[asset(path = "icons/lap-timer.svg")]
    pub lap_timer: Handle<Image>,

    #[asset(path = "icons/laptop.svg")]
    pub laptop: Handle<Image>,

    #[asset(path = "icons/layers.svg")]
    pub layers: Handle<Image>,

    #[asset(path = "icons/layout.svg")]
    pub layout: Handle<Image>,

    #[asset(path = "icons/letter-case-capitalize.svg")]
    pub letter_case_capitalize: Handle<Image>,

    #[asset(path = "icons/letter-case-lowercase.svg")]
    pub letter_case_lowercase: Handle<Image>,

    #[asset(path = "icons/letter-case-toggle.svg")]
    pub letter_case_toggle: Handle<Image>,

    #[asset(path = "icons/letter-case-uppercase.svg")]
    pub letter_case_uppercase: Handle<Image>,

    #[asset(path = "icons/letter-spacing.svg")]
    pub letter_spacing: Handle<Image>,

    #[asset(path = "icons/lightning-bolt.svg")]
    pub lightning_bolt: Handle<Image>,

    #[asset(path = "icons/line-height.svg")]
    pub line_height: Handle<Image>,

    #[asset(path = "icons/link-1.svg")]
    pub link_1: Handle<Image>,

    #[asset(path = "icons/link-2.svg")]
    pub link_2: Handle<Image>,

    #[asset(path = "icons/link-break-1.svg")]
    pub link_break_1: Handle<Image>,

    #[asset(path = "icons/link-break-2.svg")]
    pub link_break_2: Handle<Image>,

    #[asset(path = "icons/link-none-1.svg")]
    pub link_none_1: Handle<Image>,

    #[asset(path = "icons/link-none-2.svg")]
    pub link_none_2: Handle<Image>,

    #[asset(path = "icons/linkedin-logo.svg")]
    pub linkedin_logo: Handle<Image>,

    #[asset(path = "icons/list-bullet.svg")]
    pub list_bullet: Handle<Image>,

    #[asset(path = "icons/lock-closed.svg")]
    pub lock_closed: Handle<Image>,

    #[asset(path = "icons/lock-open-1.svg")]
    pub lock_open_1: Handle<Image>,

    #[asset(path = "icons/lock-open-2.svg")]
    pub lock_open_2: Handle<Image>,

    #[asset(path = "icons/loop.svg")]
    pub loop_icon: Handle<Image>,

    #[asset(path = "icons/magic-wand.svg")]
    pub magic_wand: Handle<Image>,

    #[asset(path = "icons/magnifying-glass.svg")]
    pub magnifying_glass: Handle<Image>,

    #[asset(path = "icons/margin.svg")]
    pub margin: Handle<Image>,

    #[asset(path = "icons/mask-off.svg")]
    pub mask_off: Handle<Image>,

    #[asset(path = "icons/mask-on.svg")]
    pub mask_on: Handle<Image>,

    #[asset(path = "icons/minus-circled.svg")]
    pub minus_circled: Handle<Image>,

    #[asset(path = "icons/minus.svg")]
    pub minus: Handle<Image>,

    #[asset(path = "icons/mix.svg")]
    pub mix: Handle<Image>,

    #[asset(path = "icons/mixer-horizontal.svg")]
    pub mixer_horizontal: Handle<Image>,

    #[asset(path = "icons/mixer-vertical.svg")]
    pub mixer_vertical: Handle<Image>,

    #[asset(path = "icons/mobile.svg")]
    pub mobile: Handle<Image>,

    #[asset(path = "icons/modulz-logo.svg")]
    pub modulz_logo: Handle<Image>,

    #[asset(path = "icons/moon.svg")]
    pub moon: Handle<Image>,

    #[asset(path = "icons/notion-logo.svg")]
    pub notion_logo: Handle<Image>,

    #[asset(path = "icons/opacity.svg")]
    pub opacity: Handle<Image>,

    #[asset(path = "icons/open-in-new-window.svg")]
    pub open_in_new_window: Handle<Image>,

    #[asset(path = "icons/overline.svg")]
    pub overline: Handle<Image>,

    #[asset(path = "icons/padding.svg")]
    pub padding: Handle<Image>,

    #[asset(path = "icons/paper-plane.svg")]
    pub paper_plane: Handle<Image>,

    #[asset(path = "icons/pause.svg")]
    pub pause: Handle<Image>,

    #[asset(path = "icons/pencil-1.svg")]
    pub pencil_1: Handle<Image>,

    #[asset(path = "icons/pencil-2.svg")]
    pub pencil_2: Handle<Image>,

    #[asset(path = "icons/person.svg")]
    pub person: Handle<Image>,

    #[asset(path = "icons/pie-chart.svg")]
    pub pie_chart: Handle<Image>,

    #[asset(path = "icons/pilcrow.svg")]
    pub pilcrow: Handle<Image>,

    #[asset(path = "icons/pin-bottom.svg")]
    pub pin_bottom: Handle<Image>,

    #[asset(path = "icons/pin-left.svg")]
    pub pin_left: Handle<Image>,

    #[asset(path = "icons/pin-right.svg")]
    pub pin_right: Handle<Image>,

    #[asset(path = "icons/pin-top.svg")]
    pub pin_top: Handle<Image>,

    #[asset(path = "icons/play.svg")]
    pub play: Handle<Image>,

    #[asset(path = "icons/plus-circled.svg")]
    pub plus_circled: Handle<Image>,

    #[asset(path = "icons/plus.svg")]
    pub plus: Handle<Image>,

    #[asset(path = "icons/question-mark-circled.svg")]
    pub question_mark_circled: Handle<Image>,

    #[asset(path = "icons/question-mark.svg")]
    pub question_mark: Handle<Image>,

    #[asset(path = "icons/quote.svg")]
    pub quote: Handle<Image>,

    #[asset(path = "icons/radiobutton.svg")]
    pub radiobutton: Handle<Image>,

    #[asset(path = "icons/reader.svg")]
    pub reader: Handle<Image>,

    #[asset(path = "icons/rectangle.svg")]
    pub rectangle: Handle<Image>,

    #[asset(path = "icons/reload.svg")]
    pub reload: Handle<Image>,

    #[asset(path = "icons/reset.svg")]
    pub reset: Handle<Image>,

    #[asset(path = "icons/resume.svg")]
    pub resume: Handle<Image>,

    #[asset(path = "icons/rocket.svg")]
    pub rocket: Handle<Image>,

    #[asset(path = "icons/rotate-counter-clockwise.svg")]
    pub rotate_counter_clockwise: Handle<Image>,

    #[asset(path = "icons/row-spacing.svg")]
    pub row_spacing: Handle<Image>,

    #[asset(path = "icons/rows.svg")]
    pub rows: Handle<Image>,

    #[asset(path = "icons/ruler-horizontal.svg")]
    pub ruler_horizontal: Handle<Image>,

    #[asset(path = "icons/ruler-square.svg")]
    pub ruler_square: Handle<Image>,

    #[asset(path = "icons/scissors.svg")]
    pub scissors: Handle<Image>,

    #[asset(path = "icons/section.svg")]
    pub section: Handle<Image>,

    #[asset(path = "icons/sewing-pin-filled.svg")]
    pub sewing_pin_filled: Handle<Image>,

    #[asset(path = "icons/sewing-pin.svg")]
    pub sewing_pin: Handle<Image>,

    #[asset(path = "icons/shadow-inner.svg")]
    pub shadow_inner: Handle<Image>,

    #[asset(path = "icons/shadow-none.svg")]
    pub shadow_none: Handle<Image>,

    #[asset(path = "icons/shadow-outer.svg")]
    pub shadow_outer: Handle<Image>,

    #[asset(path = "icons/shadow.svg")]
    pub shadow: Handle<Image>,

    #[asset(path = "icons/share-1.svg")]
    pub share_1: Handle<Image>,

    #[asset(path = "icons/share-2.svg")]
    pub share_2: Handle<Image>,

    #[asset(path = "icons/shuffle.svg")]
    pub shuffle: Handle<Image>,

    #[asset(path = "icons/size.svg")]
    pub size: Handle<Image>,

    #[asset(path = "icons/sketch-logo.svg")]
    pub sketch_logo: Handle<Image>,

    #[asset(path = "icons/slash.svg")]
    pub slash: Handle<Image>,

    #[asset(path = "icons/slider.svg")]
    pub slider: Handle<Image>,

    #[asset(path = "icons/space-between-horizontally.svg")]
    pub space_between_horizontally: Handle<Image>,

    #[asset(path = "icons/space-between-vertically.svg")]
    pub space_between_vertically: Handle<Image>,

    #[asset(path = "icons/space-evenly-horizontally.svg")]
    pub space_evenly_horizontally: Handle<Image>,

    #[asset(path = "icons/space-evenly-vertically.svg")]
    pub space_evenly_vertically: Handle<Image>,

    #[asset(path = "icons/speaker-loud.svg")]
    pub speaker_loud: Handle<Image>,

    #[asset(path = "icons/speaker-moderate.svg")]
    pub speaker_moderate: Handle<Image>,

    #[asset(path = "icons/speaker-off.svg")]
    pub speaker_off: Handle<Image>,

    #[asset(path = "icons/speaker-quiet.svg")]
    pub speaker_quiet: Handle<Image>,

    #[asset(path = "icons/square.svg")]
    pub square: Handle<Image>,

    #[asset(path = "icons/stack.svg")]
    pub stack: Handle<Image>,

    #[asset(path = "icons/star-filled.svg")]
    pub star_filled: Handle<Image>,

    #[asset(path = "icons/star.svg")]
    pub star: Handle<Image>,

    #[asset(path = "icons/stitches-logo.svg")]
    pub stitches_logo: Handle<Image>,

    #[asset(path = "icons/stop.svg")]
    pub stop: Handle<Image>,

    #[asset(path = "icons/stopwatch.svg")]
    pub stopwatch: Handle<Image>,

    #[asset(path = "icons/stretch-horizontally.svg")]
    pub stretch_horizontally: Handle<Image>,

    #[asset(path = "icons/stretch-vertically.svg")]
    pub stretch_vertically: Handle<Image>,

    #[asset(path = "icons/strikethrough.svg")]
    pub strikethrough: Handle<Image>,

    #[asset(path = "icons/sun.svg")]
    pub sun: Handle<Image>,

    #[asset(path = "icons/switch.svg")]
    pub switch_icon: Handle<Image>,

    #[asset(path = "icons/symbol.svg")]
    pub symbol: Handle<Image>,

    #[asset(path = "icons/table.svg")]
    pub table: Handle<Image>,

    #[asset(path = "icons/target.svg")]
    pub target: Handle<Image>,

    #[asset(path = "icons/text-align-bottom.svg")]
    pub text_align_bottom: Handle<Image>,

    #[asset(path = "icons/text-align-center.svg")]
    pub text_align_center: Handle<Image>,

    #[asset(path = "icons/text-align-justify.svg")]
    pub text_align_justify: Handle<Image>,

    #[asset(path = "icons/text-align-left.svg")]
    pub text_align_left: Handle<Image>,

    #[asset(path = "icons/text-align-middle.svg")]
    pub text_align_middle: Handle<Image>,

    #[asset(path = "icons/text-align-right.svg")]
    pub text_align_right: Handle<Image>,

    #[asset(path = "icons/text-align-top.svg")]
    pub text_align_top: Handle<Image>,

    #[asset(path = "icons/text-none.svg")]
    pub text_none: Handle<Image>,

    #[asset(path = "icons/text.svg")]
    pub text: Handle<Image>,

    #[asset(path = "icons/thick-arrow-down.svg")]
    pub thick_arrow_down: Handle<Image>,

    #[asset(path = "icons/thick-arrow-left.svg")]
    pub thick_arrow_left: Handle<Image>,

    #[asset(path = "icons/thick-arrow-right.svg")]
    pub thick_arrow_right: Handle<Image>,

    #[asset(path = "icons/thick-arrow-up.svg")]
    pub thick_arrow_up: Handle<Image>,

    #[asset(path = "icons/timer.svg")]
    pub timer: Handle<Image>,

    #[asset(path = "icons/tokens.svg")]
    pub tokens: Handle<Image>,

    #[asset(path = "icons/track-next.svg")]
    pub track_next: Handle<Image>,

    #[asset(path = "icons/track-previous.svg")]
    pub track_previous: Handle<Image>,

    #[asset(path = "icons/transform.svg")]
    pub transform: Handle<Image>,

    #[asset(path = "icons/transparency-grid.svg")]
    pub transparency_grid: Handle<Image>,

    #[asset(path = "icons/trash.svg")]
    pub trash: Handle<Image>,

    #[asset(path = "icons/triangle-down.svg")]
    pub triangle_down: Handle<Image>,

    #[asset(path = "icons/triangle-left.svg")]
    pub triangle_left: Handle<Image>,

    #[asset(path = "icons/triangle-right.svg")]
    pub triangle_right: Handle<Image>,

    #[asset(path = "icons/triangle-up.svg")]
    pub triangle_up: Handle<Image>,

    #[asset(path = "icons/twitter-logo.svg")]
    pub twitter_logo: Handle<Image>,

    #[asset(path = "icons/underline.svg")]
    pub underline: Handle<Image>,

    #[asset(path = "icons/update.svg")]
    pub update: Handle<Image>,

    #[asset(path = "icons/upload.svg")]
    pub upload: Handle<Image>,

    #[asset(path = "icons/value-none.svg")]
    pub value_none: Handle<Image>,

    #[asset(path = "icons/value.svg")]
    pub value: Handle<Image>,

    #[asset(path = "icons/vercel-logo.svg")]
    pub vercel_logo: Handle<Image>,

    #[asset(path = "icons/video.svg")]
    pub video: Handle<Image>,

    #[asset(path = "icons/view-grid.svg")]
    pub view_grid: Handle<Image>,

    #[asset(path = "icons/view-horizontal.svg")]
    pub view_horizontal: Handle<Image>,

    #[asset(path = "icons/view-none.svg")]
    pub view_none: Handle<Image>,

    #[asset(path = "icons/view-vertical.svg")]
    pub view_vertical: Handle<Image>,

    #[asset(path = "icons/width.svg")]
    pub width: Handle<Image>,

    #[asset(path = "icons/zoom-in.svg")]
    pub zoom_in: Handle<Image>,

    #[asset(path = "icons/zoom-out.svg")]
    pub zoom_out: Handle<Image>,
}
