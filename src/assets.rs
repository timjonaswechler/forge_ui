use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
struct IconAssets {
    #[asset(path = "icons/play.svg")]
    pub play: Handle<&'static [u8]>,
    #[asset(path = "icons/pause.svg")]
    pub pause: Handle<&'static [u8]>,
    #[asset(path = "icons/accessibility.svg")]
    pub accessibility: Handle<&'static [u8]>,

    #[asset(path = "icons/activity-log.svg")]
    pub activity_log: Handle<&'static [u8]>,

    #[asset(path = "icons/align-baseline.svg")]
    pub align_baseline: Handle<&'static [u8]>,

    #[asset(path = "icons/align-bottom.svg")]
    pub align_bottom: Handle<&'static [u8]>,

    #[asset(path = "icons/align-center-horizontally.svg")]
    pub align_center_horizontally: Handle<&'static [u8]>,

    #[asset(path = "icons/align-center-vertically.svg")]
    pub align_center_vertically: Handle<&'static [u8]>,

    #[asset(path = "icons/align-left.svg")]
    pub align_left: Handle<&'static [u8]>,

    #[asset(path = "icons/align-right.svg")]
    pub align_right: Handle<&'static [u8]>,

    #[asset(path = "icons/align-top.svg")]
    pub align_top: Handle<&'static [u8]>,

    #[asset(path = "icons/all-sides.svg")]
    pub all_sides: Handle<&'static [u8]>,

    #[asset(path = "icons/angle.svg")]
    pub angle: Handle<&'static [u8]>,

    #[asset(path = "icons/archive.svg")]
    pub archive: Handle<&'static [u8]>,

    #[asset(path = "icons/arrow-bottom-left.svg")]
    pub arrow_bottom_left: Handle<&'static [u8]>,

    #[asset(path = "icons/arrow-bottom-right.svg")]
    pub arrow_bottom_right: Handle<&'static [u8]>,

    #[asset(path = "icons/arrow-down.svg")]
    pub arrow_down: Handle<&'static [u8]>,

    #[asset(path = "icons/arrow-left.svg")]
    pub arrow_left: Handle<&'static [u8]>,

    #[asset(path = "icons/arrow-right.svg")]
    pub arrow_right: Handle<&'static [u8]>,

    #[asset(path = "icons/arrow-top-left.svg")]
    pub arrow_top_left: Handle<&'static [u8]>,

    #[asset(path = "icons/arrow-top-right.svg")]
    pub arrow_top_right: Handle<&'static [u8]>,

    #[asset(path = "icons/arrow-up.svg")]
    pub arrow_up: Handle<&'static [u8]>,

    #[asset(path = "icons/aspect-ratio.svg")]
    pub aspect_ratio: Handle<&'static [u8]>,

    #[asset(path = "icons/avatar.svg")]
    pub avatar: Handle<&'static [u8]>,

    #[asset(path = "icons/backpack.svg")]
    pub backpack: Handle<&'static [u8]>,

    #[asset(path = "icons/badge.svg")]
    pub badge: Handle<&'static [u8]>,

    #[asset(path = "icons/bar-chart.svg")]
    pub bar_chart: Handle<&'static [u8]>,

    #[asset(path = "icons/bell.svg")]
    pub bell: Handle<&'static [u8]>,

    #[asset(path = "icons/blending-mode.svg")]
    pub blending_mode: Handle<&'static [u8]>,

    #[asset(path = "icons/bookmark-filled.svg")]
    pub bookmark_filled: Handle<&'static [u8]>,

    #[asset(path = "icons/bookmark.svg")]
    pub bookmark: Handle<&'static [u8]>,

    #[asset(path = "icons/border-all.svg")]
    pub border_all: Handle<&'static [u8]>,

    #[asset(path = "icons/border-bottom.svg")]
    pub border_bottom: Handle<&'static [u8]>,

    #[asset(path = "icons/border-dashed.svg")]
    pub border_dashed: Handle<&'static [u8]>,

    #[asset(path = "icons/border-dotted.svg")]
    pub border_dotted: Handle<&'static [u8]>,

    #[asset(path = "icons/border-left.svg")]
    pub border_left: Handle<&'static [u8]>,

    #[asset(path = "icons/border-none.svg")]
    pub border_none: Handle<&'static [u8]>,

    #[asset(path = "icons/border-right.svg")]
    pub border_right: Handle<&'static [u8]>,

    #[asset(path = "icons/border-solid.svg")]
    pub border_solid: Handle<&'static [u8]>,

    #[asset(path = "icons/border-split.svg")]
    pub border_split: Handle<&'static [u8]>,

    #[asset(path = "icons/border-style.svg")]
    pub border_style: Handle<&'static [u8]>,

    #[asset(path = "icons/border-top.svg")]
    pub border_top: Handle<&'static [u8]>,

    #[asset(path = "icons/border-width.svg")]
    pub border_width: Handle<&'static [u8]>,

    #[asset(path = "icons/box-model.svg")]
    pub box_model: Handle<&'static [u8]>,

    #[asset(path = "icons/button.svg")]
    pub button: Handle<&'static [u8]>,

    #[asset(path = "icons/calendar.svg")]
    pub calendar: Handle<&'static [u8]>,

    #[asset(path = "icons/camera.svg")]
    pub camera: Handle<&'static [u8]>,

    #[asset(path = "icons/card-stack-minus.svg")]
    pub card_stack_minus: Handle<&'static [u8]>,

    #[asset(path = "icons/card-stack-plus.svg")]
    pub card_stack_plus: Handle<&'static [u8]>,

    #[asset(path = "icons/card-stack.svg")]
    pub card_stack: Handle<&'static [u8]>,

    #[asset(path = "icons/caret-down.svg")]
    pub caret_down: Handle<&'static [u8]>,

    #[asset(path = "icons/caret-left.svg")]
    pub caret_left: Handle<&'static [u8]>,

    #[asset(path = "icons/caret-right.svg")]
    pub caret_right: Handle<&'static [u8]>,

    #[asset(path = "icons/caret-sort.svg")]
    pub caret_sort: Handle<&'static [u8]>,

    #[asset(path = "icons/caret-up.svg")]
    pub caret_up: Handle<&'static [u8]>,

    #[asset(path = "icons/chat-bubble.svg")]
    pub chat_bubble: Handle<&'static [u8]>,

    #[asset(path = "icons/check-circled.svg")]
    pub check_circled: Handle<&'static [u8]>,

    #[asset(path = "icons/check.svg")]
    pub check: Handle<&'static [u8]>,

    #[asset(path = "icons/checkbox.svg")]
    pub checkbox: Handle<&'static [u8]>,

    #[asset(path = "icons/chevron-down.svg")]
    pub chevron_down: Handle<&'static [u8]>,

    #[asset(path = "icons/chevron-left.svg")]
    pub chevron_left: Handle<&'static [u8]>,

    #[asset(path = "icons/chevron-right.svg")]
    pub chevron_right: Handle<&'static [u8]>,

    #[asset(path = "icons/chevron-up.svg")]
    pub chevron_up: Handle<&'static [u8]>,

    #[asset(path = "icons/circle-backslash.svg")]
    pub circle_backslash: Handle<&'static [u8]>,

    #[asset(path = "icons/circle.svg")]
    pub circle: Handle<&'static [u8]>,

    #[asset(path = "icons/clipboard-copy.svg")]
    pub clipboard_copy: Handle<&'static [u8]>,

    #[asset(path = "icons/clipboard.svg")]
    pub clipboard: Handle<&'static [u8]>,

    #[asset(path = "icons/clock.svg")]
    pub clock: Handle<&'static [u8]>,

    #[asset(path = "icons/code.svg")]
    pub code: Handle<&'static [u8]>,

    #[asset(path = "icons/codesandbox-logo.svg")]
    pub codesandbox_logo: Handle<&'static [u8]>,

    #[asset(path = "icons/color-wheel.svg")]
    pub color_wheel: Handle<&'static [u8]>,

    #[asset(path = "icons/column-spacing.svg")]
    pub column_spacing: Handle<&'static [u8]>,

    #[asset(path = "icons/columns.svg")]
    pub columns: Handle<&'static [u8]>,

    #[asset(path = "icons/commit.svg")]
    pub commit: Handle<&'static [u8]>,

    #[asset(path = "icons/component-1.svg")]
    pub component_1: Handle<&'static [u8]>,

    #[asset(path = "icons/component-2.svg")]
    pub component_2: Handle<&'static [u8]>,

    #[asset(path = "icons/component-boolean.svg")]
    pub component_boolean: Handle<&'static [u8]>,

    #[asset(path = "icons/component-instance.svg")]
    pub component_instance: Handle<&'static [u8]>,

    #[asset(path = "icons/component-none.svg")]
    pub component_none: Handle<&'static [u8]>,

    #[asset(path = "icons/component-placeholder.svg")]
    pub component_placeholder: Handle<&'static [u8]>,

    #[asset(path = "icons/container.svg")]
    pub container: Handle<&'static [u8]>,

    #[asset(path = "icons/cookie.svg")]
    pub cookie: Handle<&'static [u8]>,

    #[asset(path = "icons/copy.svg")]
    pub copy: Handle<&'static [u8]>,

    #[asset(path = "icons/corner-bottom-left.svg")]
    pub corner_bottom_left: Handle<&'static [u8]>,

    #[asset(path = "icons/corner-bottom-right.svg")]
    pub corner_bottom_right: Handle<&'static [u8]>,

    #[asset(path = "icons/corner-top-left.svg")]
    pub corner_top_left: Handle<&'static [u8]>,

    #[asset(path = "icons/corner-top-right.svg")]
    pub corner_top_right: Handle<&'static [u8]>,

    #[asset(path = "icons/corners.svg")]
    pub corners: Handle<&'static [u8]>,

    #[asset(path = "icons/countdown-timer.svg")]
    pub countdown_timer: Handle<&'static [u8]>,

    #[asset(path = "icons/counter-clockwise-clock.svg")]
    pub counter_clockwise_clock: Handle<&'static [u8]>,

    #[asset(path = "icons/crop.svg")]
    pub crop: Handle<&'static [u8]>,

    #[asset(path = "icons/cross-1.svg")]
    pub cross_1: Handle<&'static [u8]>,

    #[asset(path = "icons/cross-2.svg")]
    pub cross_2: Handle<&'static [u8]>,

    #[asset(path = "icons/cross-circled.svg")]
    pub cross_circled: Handle<&'static [u8]>,

    #[asset(path = "icons/crosshair-1.svg")]
    pub crosshair_1: Handle<&'static [u8]>,

    #[asset(path = "icons/crosshair-2.svg")]
    pub crosshair_2: Handle<&'static [u8]>,

    #[asset(path = "icons/crumpled-paper.svg")]
    pub crumpled_paper: Handle<&'static [u8]>,

    #[asset(path = "icons/cube.svg")]
    pub cube: Handle<&'static [u8]>,

    #[asset(path = "icons/cursor-arrow.svg")]
    pub cursor_arrow: Handle<&'static [u8]>,

    #[asset(path = "icons/cursor-text.svg")]
    pub cursor_text: Handle<&'static [u8]>,

    #[asset(path = "icons/dash.svg")]
    pub dash: Handle<&'static [u8]>,

    #[asset(path = "icons/dashboard.svg")]
    pub dashboard: Handle<&'static [u8]>,

    #[asset(path = "icons/desktop.svg")]
    pub desktop: Handle<&'static [u8]>,

    #[asset(path = "icons/dimensions.svg")]
    pub dimensions: Handle<&'static [u8]>,

    #[asset(path = "icons/disc.svg")]
    pub disc: Handle<&'static [u8]>,

    #[asset(path = "icons/discord-logo.svg")]
    pub discord_logo: Handle<&'static [u8]>,

    #[asset(path = "icons/divider-horizontal.svg")]
    pub divider_horizontal: Handle<&'static [u8]>,

    #[asset(path = "icons/divider-vertical.svg")]
    pub divider_vertical: Handle<&'static [u8]>,

    #[asset(path = "icons/dot-filled.svg")]
    pub dot_filled: Handle<&'static [u8]>,

    #[asset(path = "icons/dot.svg")]
    pub dot: Handle<&'static [u8]>,

    #[asset(path = "icons/dots-horizontal.svg")]
    pub dots_horizontal: Handle<&'static [u8]>,

    #[asset(path = "icons/dots-vertical.svg")]
    pub dots_vertical: Handle<&'static [u8]>,

    #[asset(path = "icons/double-arrow-down.svg")]
    pub double_arrow_down: Handle<&'static [u8]>,

    #[asset(path = "icons/double-arrow-left.svg")]
    pub double_arrow_left: Handle<&'static [u8]>,

    #[asset(path = "icons/double-arrow-right.svg")]
    pub double_arrow_right: Handle<&'static [u8]>,

    #[asset(path = "icons/double-arrow-up.svg")]
    pub double_arrow_up: Handle<&'static [u8]>,

    #[asset(path = "icons/download.svg")]
    pub download: Handle<&'static [u8]>,

    #[asset(path = "icons/drag.svg")]
    pub drag: Handle<&'static [u8]>,

    #[asset(path = "icons/drag-handle-dots-1.svg")]
    pub drag_handle_dots_1: Handle<&'static [u8]>,

    #[asset(path = "icons/drag-handle-dots-2.svg")]
    pub drag_handle_dots_2: Handle<&'static [u8]>,

    #[asset(path = "icons/drag-handle-horizontal.svg")]
    pub drag_handle_horizontal: Handle<&'static [u8]>,

    #[asset(path = "icons/drag-handle-vertical.svg")]
    pub drag_handle_vertical: Handle<&'static [u8]>,

    #[asset(path = "icons/drawing-pin-filled.svg")]
    pub drawing_pin_filled: Handle<&'static [u8]>,

    #[asset(path = "icons/drawing-pin.svg")]
    pub drawing_pin: Handle<&'static [u8]>,

    #[asset(path = "icons/dropdown-menu.svg")]
    pub dropdown_menu: Handle<&'static [u8]>,

    #[asset(path = "icons/enter-full-screen.svg")]
    pub enter_full_screen: Handle<&'static [u8]>,

    #[asset(path = "icons/enter.svg")]
    pub enter: Handle<&'static [u8]>,

    #[asset(path = "icons/envelope-closed.svg")]
    pub envelope_closed: Handle<&'static [u8]>,

    #[asset(path = "icons/envelope-open.svg")]
    pub envelope_open: Handle<&'static [u8]>,

    #[asset(path = "icons/eraser.svg")]
    pub eraser: Handle<&'static [u8]>,

    #[asset(path = "icons/exclamation-triangle.svg")]
    pub exclamation_triangle: Handle<&'static [u8]>,

    #[asset(path = "icons/exit-full-screen.svg")]
    pub exit_full_screen: Handle<&'static [u8]>,

    #[asset(path = "icons/exit.svg")]
    pub exit: Handle<&'static [u8]>,

    #[asset(path = "icons/external-link.svg")]
    pub external_link: Handle<&'static [u8]>,

    #[asset(path = "icons/eye-closed.svg")]
    pub eye_closed: Handle<&'static [u8]>,

    #[asset(path = "icons/eye-none.svg")]
    pub eye_none: Handle<&'static [u8]>,

    #[asset(path = "icons/eye-open.svg")]
    pub eye_open: Handle<&'static [u8]>,

    #[asset(path = "icons/face.svg")]
    pub face: Handle<&'static [u8]>,

    #[asset(path = "icons/figma-logo.svg")]
    pub figma_logo: Handle<&'static [u8]>,

    #[asset(path = "icons/file-minus.svg")]
    pub file_minus: Handle<&'static [u8]>,

    #[asset(path = "icons/file-plus.svg")]
    pub file_plus: Handle<&'static [u8]>,

    #[asset(path = "icons/file-text.svg")]
    pub file_text: Handle<&'static [u8]>,

    #[asset(path = "icons/file.svg")]
    pub file: Handle<&'static [u8]>,

    #[asset(path = "icons/font-bold.svg")]
    pub font_bold: Handle<&'static [u8]>,

    #[asset(path = "icons/font-family.svg")]
    pub font_family: Handle<&'static [u8]>,

    #[asset(path = "icons/font-italic.svg")]
    pub font_italic: Handle<&'static [u8]>,

    #[asset(path = "icons/font-roman.svg")]
    pub font_roman: Handle<&'static [u8]>,

    #[asset(path = "icons/font-size.svg")]
    pub font_size: Handle<&'static [u8]>,

    #[asset(path = "icons/font-style.svg")]
    pub font_style: Handle<&'static [u8]>,

    #[asset(path = "icons/frame.svg")]
    pub frame: Handle<&'static [u8]>,

    #[asset(path = "icons/framer-logo.svg")]
    pub framer_logo: Handle<&'static [u8]>,

    #[asset(path = "icons/gear.svg")]
    pub gear: Handle<&'static [u8]>,

    #[asset(path = "icons/github-logo.svg")]
    pub github_logo: Handle<&'static [u8]>,

    #[asset(path = "icons/globe.svg")]
    pub globe: Handle<&'static [u8]>,

    #[asset(path = "icons/grid.svg")]
    pub grid: Handle<&'static [u8]>,

    #[asset(path = "icons/group.svg")]
    pub group: Handle<&'static [u8]>,

    #[asset(path = "icons/half-1.svg")]
    pub half_1: Handle<&'static [u8]>,

    #[asset(path = "icons/half-2.svg")]
    pub half_2: Handle<&'static [u8]>,

    #[asset(path = "icons/hamburger-menu.svg")]
    pub hamburger_menu: Handle<&'static [u8]>,

    #[asset(path = "icons/hand.svg")]
    pub hand: Handle<&'static [u8]>,

    #[asset(path = "icons/heading.svg")]
    pub heading: Handle<&'static [u8]>,

    #[asset(path = "icons/heart-filled.svg")]
    pub heart_filled: Handle<&'static [u8]>,

    #[asset(path = "icons/heart.svg")]
    pub heart: Handle<&'static [u8]>,

    #[asset(path = "icons/height.svg")]
    pub height: Handle<&'static [u8]>,

    #[asset(path = "icons/hobby-knife.svg")]
    pub hobby_knife: Handle<&'static [u8]>,

    #[asset(path = "icons/home.svg")]
    pub home: Handle<&'static [u8]>,

    #[asset(path = "icons/iconjar-logo.svg")]
    pub iconjar_logo: Handle<&'static [u8]>,

    #[asset(path = "icons/id-card.svg")]
    pub id_card: Handle<&'static [u8]>,

    #[asset(path = "icons/image.svg")]
    pub image: Handle<&'static [u8]>,

    #[asset(path = "icons/info-circled.svg")]
    pub info_circled: Handle<&'static [u8]>,

    #[asset(path = "icons/input.svg")]
    pub input: Handle<&'static [u8]>,

    #[asset(path = "icons/instagram-logo.svg")]
    pub instagram_logo: Handle<&'static [u8]>,

    #[asset(path = "icons/keyboard.svg")]
    pub keyboard: Handle<&'static [u8]>,

    #[asset(path = "icons/lap-timer.svg")]
    pub lap_timer: Handle<&'static [u8]>,

    #[asset(path = "icons/laptop.svg")]
    pub laptop: Handle<&'static [u8]>,

    #[asset(path = "icons/layers.svg")]
    pub layers: Handle<&'static [u8]>,

    #[asset(path = "icons/layout.svg")]
    pub layout: Handle<&'static [u8]>,

    #[asset(path = "icons/letter-case-capitalize.svg")]
    pub letter_case_capitalize: Handle<&'static [u8]>,

    #[asset(path = "icons/letter-case-lowercase.svg")]
    pub letter_case_lowercase: Handle<&'static [u8]>,

    #[asset(path = "icons/letter-case-toggle.svg")]
    pub letter_case_toggle: Handle<&'static [u8]>,

    #[asset(path = "icons/letter-case-uppercase.svg")]
    pub letter_case_uppercase: Handle<&'static [u8]>,

    #[asset(path = "icons/letter-spacing.svg")]
    pub letter_spacing: Handle<&'static [u8]>,

    #[asset(path = "icons/lightning-bolt.svg")]
    pub lightning_bolt: Handle<&'static [u8]>,

    #[asset(path = "icons/line-height.svg")]
    pub line_height: Handle<&'static [u8]>,

    #[asset(path = "icons/link-1.svg")]
    pub link_1: Handle<&'static [u8]>,

    #[asset(path = "icons/link-2.svg")]
    pub link_2: Handle<&'static [u8]>,

    #[asset(path = "icons/link-break-1.svg")]
    pub link_break_1: Handle<&'static [u8]>,

    #[asset(path = "icons/link-break-2.svg")]
    pub link_break_2: Handle<&'static [u8]>,

    #[asset(path = "icons/link-none-1.svg")]
    pub link_none_1: Handle<&'static [u8]>,

    #[asset(path = "icons/link-none-2.svg")]
    pub link_none_2: Handle<&'static [u8]>,

    #[asset(path = "icons/linkedin-logo.svg")]
    pub linkedin_logo: Handle<&'static [u8]>,

    #[asset(path = "icons/list-bullet.svg")]
    pub list_bullet: Handle<&'static [u8]>,

    #[asset(path = "icons/lock-closed.svg")]
    pub lock_closed: Handle<&'static [u8]>,

    #[asset(path = "icons/lock-open-1.svg")]
    pub lock_open_1: Handle<&'static [u8]>,

    #[asset(path = "icons/lock-open-2.svg")]
    pub lock_open_2: Handle<&'static [u8]>,

    #[asset(path = "icons/loop.svg")]
    pub loop_icon: Handle<&'static [u8]>,

    #[asset(path = "icons/magic-wand.svg")]
    pub magic_wand: Handle<&'static [u8]>,

    #[asset(path = "icons/magnifying-glass.svg")]
    pub magnifying_glass: Handle<&'static [u8]>,

    #[asset(path = "icons/margin.svg")]
    pub margin: Handle<&'static [u8]>,

    #[asset(path = "icons/mask-off.svg")]
    pub mask_off: Handle<&'static [u8]>,

    #[asset(path = "icons/mask-on.svg")]
    pub mask_on: Handle<&'static [u8]>,

    #[asset(path = "icons/minus-circled.svg")]
    pub minus_circled: Handle<&'static [u8]>,

    #[asset(path = "icons/minus.svg")]
    pub minus: Handle<&'static [u8]>,

    #[asset(path = "icons/mix.svg")]
    pub mix: Handle<&'static [u8]>,

    #[asset(path = "icons/mixer-horizontal.svg")]
    pub mixer_horizontal: Handle<&'static [u8]>,

    #[asset(path = "icons/mixer-vertical.svg")]
    pub mixer_vertical: Handle<&'static [u8]>,

    #[asset(path = "icons/mobile.svg")]
    pub mobile: Handle<&'static [u8]>,

    #[asset(path = "icons/modulz-logo.svg")]
    pub modulz_logo: Handle<&'static [u8]>,

    #[asset(path = "icons/moon.svg")]
    pub moon: Handle<&'static [u8]>,

    #[asset(path = "icons/notion-logo.svg")]
    pub notion_logo: Handle<&'static [u8]>,

    #[asset(path = "icons/opacity.svg")]
    pub opacity: Handle<&'static [u8]>,

    #[asset(path = "icons/open-in-new-window.svg")]
    pub open_in_new_window: Handle<&'static [u8]>,

    #[asset(path = "icons/overline.svg")]
    pub overline: Handle<&'static [u8]>,

    #[asset(path = "icons/padding.svg")]
    pub padding: Handle<&'static [u8]>,

    #[asset(path = "icons/paper-plane.svg")]
    pub paper_plane: Handle<&'static [u8]>,

    #[asset(path = "icons/pause.svg")]
    pub pause: Handle<&'static [u8]>,

    #[asset(path = "icons/pencil-1.svg")]
    pub pencil_1: Handle<&'static [u8]>,

    #[asset(path = "icons/pencil-2.svg")]
    pub pencil_2: Handle<&'static [u8]>,

    #[asset(path = "icons/person.svg")]
    pub person: Handle<&'static [u8]>,

    #[asset(path = "icons/pie-chart.svg")]
    pub pie_chart: Handle<&'static [u8]>,

    #[asset(path = "icons/pilcrow.svg")]
    pub pilcrow: Handle<&'static [u8]>,

    #[asset(path = "icons/pin-bottom.svg")]
    pub pin_bottom: Handle<&'static [u8]>,

    #[asset(path = "icons/pin-left.svg")]
    pub pin_left: Handle<&'static [u8]>,

    #[asset(path = "icons/pin-right.svg")]
    pub pin_right: Handle<&'static [u8]>,

    #[asset(path = "icons/pin-top.svg")]
    pub pin_top: Handle<&'static [u8]>,

    #[asset(path = "icons/play.svg")]
    pub play: Handle<&'static [u8]>,

    #[asset(path = "icons/plus-circled.svg")]
    pub plus_circled: Handle<&'static [u8]>,

    #[asset(path = "icons/plus.svg")]
    pub plus: Handle<&'static [u8]>,

    #[asset(path = "icons/question-mark-circled.svg")]
    pub question_mark_circled: Handle<&'static [u8]>,

    #[asset(path = "icons/question-mark.svg")]
    pub question_mark: Handle<&'static [u8]>,

    #[asset(path = "icons/quote.svg")]
    pub quote: Handle<&'static [u8]>,

    #[asset(path = "icons/radiobutton.svg")]
    pub radiobutton: Handle<&'static [u8]>,

    #[asset(path = "icons/reader.svg")]
    pub reader: Handle<&'static [u8]>,

    #[asset(path = "icons/rectangle.svg")]
    pub rectangle: Handle<&'static [u8]>,

    #[asset(path = "icons/reload.svg")]
    pub reload: Handle<&'static [u8]>,

    #[asset(path = "icons/reset.svg")]
    pub reset: Handle<&'static [u8]>,

    #[asset(path = "icons/resume.svg")]
    pub resume: Handle<&'static [u8]>,

    #[asset(path = "icons/rocket.svg")]
    pub rocket: Handle<&'static [u8]>,

    #[asset(path = "icons/rotate-counter-clockwise.svg")]
    pub rotate_counter_clockwise: Handle<&'static [u8]>,

    #[asset(path = "icons/row-spacing.svg")]
    pub row_spacing: Handle<&'static [u8]>,

    #[asset(path = "icons/rows.svg")]
    pub rows: Handle<&'static [u8]>,

    #[asset(path = "icons/ruler-horizontal.svg")]
    pub ruler_horizontal: Handle<&'static [u8]>,

    #[asset(path = "icons/ruler-square.svg")]
    pub ruler_square: Handle<&'static [u8]>,

    #[asset(path = "icons/scissors.svg")]
    pub scissors: Handle<&'static [u8]>,

    #[asset(path = "icons/section.svg")]
    pub section: Handle<&'static [u8]>,

    #[asset(path = "icons/sewing-pin-filled.svg")]
    pub sewing_pin_filled: Handle<&'static [u8]>,

    #[asset(path = "icons/sewing-pin.svg")]
    pub sewing_pin: Handle<&'static [u8]>,

    #[asset(path = "icons/shadow-inner.svg")]
    pub shadow_inner: Handle<&'static [u8]>,

    #[asset(path = "icons/shadow-none.svg")]
    pub shadow_none: Handle<&'static [u8]>,

    #[asset(path = "icons/shadow-outer.svg")]
    pub shadow_outer: Handle<&'static [u8]>,

    #[asset(path = "icons/shadow.svg")]
    pub shadow: Handle<&'static [u8]>,

    #[asset(path = "icons/share-1.svg")]
    pub share_1: Handle<&'static [u8]>,

    #[asset(path = "icons/share-2.svg")]
    pub share_2: Handle<&'static [u8]>,

    #[asset(path = "icons/shuffle.svg")]
    pub shuffle: Handle<&'static [u8]>,

    #[asset(path = "icons/size.svg")]
    pub size: Handle<&'static [u8]>,

    #[asset(path = "icons/sketch-logo.svg")]
    pub sketch_logo: Handle<&'static [u8]>,

    #[asset(path = "icons/slash.svg")]
    pub slash: Handle<&'static [u8]>,

    #[asset(path = "icons/slider.svg")]
    pub slider: Handle<&'static [u8]>,

    #[asset(path = "icons/space-between-horizontally.svg")]
    pub space_between_horizontally: Handle<&'static [u8]>,

    #[asset(path = "icons/space-between-vertically.svg")]
    pub space_between_vertically: Handle<&'static [u8]>,

    #[asset(path = "icons/space-evenly-horizontally.svg")]
    pub space_evenly_horizontally: Handle<&'static [u8]>,

    #[asset(path = "icons/space-evenly-vertically.svg")]
    pub space_evenly_vertically: Handle<&'static [u8]>,

    #[asset(path = "icons/speaker-loud.svg")]
    pub speaker_loud: Handle<&'static [u8]>,

    #[asset(path = "icons/speaker-moderate.svg")]
    pub speaker_moderate: Handle<&'static [u8]>,

    #[asset(path = "icons/speaker-off.svg")]
    pub speaker_off: Handle<&'static [u8]>,

    #[asset(path = "icons/speaker-quiet.svg")]
    pub speaker_quiet: Handle<&'static [u8]>,

    #[asset(path = "icons/square.svg")]
    pub square: Handle<&'static [u8]>,

    #[asset(path = "icons/stack.svg")]
    pub stack: Handle<&'static [u8]>,

    #[asset(path = "icons/star-filled.svg")]
    pub star_filled: Handle<&'static [u8]>,

    #[asset(path = "icons/star.svg")]
    pub star: Handle<&'static [u8]>,

    #[asset(path = "icons/stitches-logo.svg")]
    pub stitches_logo: Handle<&'static [u8]>,

    #[asset(path = "icons/stop.svg")]
    pub stop: Handle<&'static [u8]>,

    #[asset(path = "icons/stopwatch.svg")]
    pub stopwatch: Handle<&'static [u8]>,

    #[asset(path = "icons/stretch-horizontally.svg")]
    pub stretch_horizontally: Handle<&'static [u8]>,

    #[asset(path = "icons/stretch-vertically.svg")]
    pub stretch_vertically: Handle<&'static [u8]>,

    #[asset(path = "icons/strikethrough.svg")]
    pub strikethrough: Handle<&'static [u8]>,

    #[asset(path = "icons/sun.svg")]
    pub sun: Handle<&'static [u8]>,

    #[asset(path = "icons/switch.svg")]
    pub switch_icon: Handle<&'static [u8]>,

    #[asset(path = "icons/symbol.svg")]
    pub symbol: Handle<&'static [u8]>,

    #[asset(path = "icons/table.svg")]
    pub table: Handle<&'static [u8]>,

    #[asset(path = "icons/target.svg")]
    pub target: Handle<&'static [u8]>,

    #[asset(path = "icons/text-align-bottom.svg")]
    pub text_align_bottom: Handle<&'static [u8]>,

    #[asset(path = "icons/text-align-center.svg")]
    pub text_align_center: Handle<&'static [u8]>,

    #[asset(path = "icons/text-align-justify.svg")]
    pub text_align_justify: Handle<&'static [u8]>,

    #[asset(path = "icons/text-align-left.svg")]
    pub text_align_left: Handle<&'static [u8]>,

    #[asset(path = "icons/text-align-middle.svg")]
    pub text_align_middle: Handle<&'static [u8]>,

    #[asset(path = "icons/text-align-right.svg")]
    pub text_align_right: Handle<&'static [u8]>,

    #[asset(path = "icons/text-align-top.svg")]
    pub text_align_top: Handle<&'static [u8]>,

    #[asset(path = "icons/text-none.svg")]
    pub text_none: Handle<&'static [u8]>,

    #[asset(path = "icons/text.svg")]
    pub text: Handle<&'static [u8]>,

    #[asset(path = "icons/thick-arrow-down.svg")]
    pub thick_arrow_down: Handle<&'static [u8]>,

    #[asset(path = "icons/thick-arrow-left.svg")]
    pub thick_arrow_left: Handle<&'static [u8]>,

    #[asset(path = "icons/thick-arrow-right.svg")]
    pub thick_arrow_right: Handle<&'static [u8]>,

    #[asset(path = "icons/thick-arrow-up.svg")]
    pub thick_arrow_up: Handle<&'static [u8]>,

    #[asset(path = "icons/timer.svg")]
    pub timer: Handle<&'static [u8]>,

    #[asset(path = "icons/tokens.svg")]
    pub tokens: Handle<&'static [u8]>,

    #[asset(path = "icons/track-next.svg")]
    pub track_next: Handle<&'static [u8]>,

    #[asset(path = "icons/track-previous.svg")]
    pub track_previous: Handle<&'static [u8]>,

    #[asset(path = "icons/transform.svg")]
    pub transform: Handle<&'static [u8]>,

    #[asset(path = "icons/transparency-grid.svg")]
    pub transparency_grid: Handle<&'static [u8]>,

    #[asset(path = "icons/trash.svg")]
    pub trash: Handle<&'static [u8]>,

    #[asset(path = "icons/triangle-down.svg")]
    pub triangle_down: Handle<&'static [u8]>,

    #[asset(path = "icons/triangle-left.svg")]
    pub triangle_left: Handle<&'static [u8]>,

    #[asset(path = "icons/triangle-right.svg")]
    pub triangle_right: Handle<&'static [u8]>,

    #[asset(path = "icons/triangle-up.svg")]
    pub triangle_up: Handle<&'static [u8]>,

    #[asset(path = "icons/twitter-logo.svg")]
    pub twitter_logo: Handle<&'static [u8]>,

    #[asset(path = "icons/underline.svg")]
    pub underline: Handle<&'static [u8]>,

    #[asset(path = "icons/update.svg")]
    pub update: Handle<&'static [u8]>,

    #[asset(path = "icons/upload.svg")]
    pub upload: Handle<&'static [u8]>,

    #[asset(path = "icons/value-none.svg")]
    pub value_none: Handle<&'static [u8]>,

    #[asset(path = "icons/value.svg")]
    pub value: Handle<&'static [u8]>,

    #[asset(path = "icons/vercel-logo.svg")]
    pub vercel_logo: Handle<&'static [u8]>,

    #[asset(path = "icons/video.svg")]
    pub video: Handle<&'static [u8]>,

    #[asset(path = "icons/view-grid.svg")]
    pub view_grid: Handle<&'static [u8]>,

    #[asset(path = "icons/view-horizontal.svg")]
    pub view_horizontal: Handle<&'static [u8]>,

    #[asset(path = "icons/view-none.svg")]
    pub view_none: Handle<&'static [u8]>,

    #[asset(path = "icons/view-vertical.svg")]
    pub view_vertical: Handle<&'static [u8]>,

    #[asset(path = "icons/width.svg")]
    pub width: Handle<&'static [u8]>,

    #[asset(path = "icons/zoom-in.svg")]
    pub zoom_in: Handle<&'static [u8]>,

    #[asset(path = "icons/zoom-out.svg")]
    pub zoom_out: Handle<&'static [u8]>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    // Default
    #[asset(path = "fonts/Roboto-Regular.ttf")]
    pub default: Handle<bevy::prelude::Font>,

    // Sans
    #[asset(path = "fonts/Roboto-Light.ttf")]
    pub sans_light: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-LightItalic.ttf")]
    pub sans_light_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-Regular.ttf")]
    pub sans_regular: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-RegularItalic.ttf")]
    pub sans_regular_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-Medium.ttf")]
    pub sans_medium: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-MediumItalic.ttf")]
    pub sans_medium_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-Bold.ttf")]
    pub sans_bold: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-BoldItalic.ttf")]
    pub sans_bold_italic: Handle<bevy::prelude::Font>,

    // Serif
    #[asset(path = "fonts/NotoSerif-Light.ttf")]
    pub serif_light: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-LightItalic.ttf")]
    pub serif_light_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-Regular.ttf")]
    pub serif_regular: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-RegularItalic.ttf")]
    pub serif_regular_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-Medium.ttf")]
    pub serif_medium: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-MediumItalic.ttf")]
    pub serif_medium_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-Bold.ttf")]
    pub serif_bold: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-BoldItalic.ttf")]
    pub serif_bold_italic: Handle<bevy::prelude::Font>,

    // Mono
    #[asset(path = "fonts/RobotoMono-Light.ttf")]
    pub mono_light: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-LightItalic.ttf")]
    pub mono_light_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-Regular.ttf")]
    pub mono_regular: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-RegularItalic.ttf")]
    pub mono_regular_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-Medium.ttf")]
    pub mono_medium: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-MediumItalic.ttf")]
    pub mono_medium_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-Bold.ttf")]
    pub mono_bold: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-BoldItalic.ttf")]
    pub mono_bold_italic: Handle<bevy::prelude::Font>,
}
