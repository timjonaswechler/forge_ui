// src/theme/runtime/layout.rs
use crate::theme::data::UiLayoutData;

#[derive(Debug, Clone)]
pub struct UiLayout {
    pub spacing: f32,
    pub padding: UiSpacing,
    pub margin: UiSpacing,
    pub gap: UiSpacing,
    pub radius: UiRadius,
    pub border: UiSpacing,
}

#[derive(Debug, Clone)]
pub struct UiSpacing {
    pub xs: f32,
    pub sm: f32,
    pub base: f32,
    pub lg: f32,
    pub xl: f32,
    pub x2l: f32,
    pub x3l: f32,
    pub x4l: f32,
    pub x5l: f32,
}

#[derive(Debug, Clone)]
pub struct UiRadius {
    pub xs: f32,
    pub sm: f32,
    pub base: f32,
    pub lg: f32,
    pub xl: f32,
    pub x2l: f32,
    pub x3l: f32,
    pub x4l: f32,
    pub full: f32,
}

pub fn build(data: &UiLayoutData, rem: f32, ui_scaling: f32) -> UiLayout {
    let base = data.spacing * rem * ui_scaling;
    let s = |v: f32| v * base;
    UiLayout {
        spacing: base,
        padding: UiSpacing {
            xs: s(data.padding.xs),
            sm: s(data.padding.sm),
            base: s(data.padding.base),
            lg: s(data.padding.lg),
            xl: s(data.padding.xl),
            x2l: s(data.padding.x2l),
            x3l: s(data.padding.x3l),
            x4l: s(data.padding.x4l),
            x5l: s(data.padding.x5l),
        },
        margin: UiSpacing {
            xs: s(data.margin.xs),
            sm: s(data.margin.sm),
            base: s(data.margin.base),
            lg: s(data.margin.lg),
            xl: s(data.margin.xl),
            x2l: s(data.margin.x2l),
            x3l: s(data.margin.x3l),
            x4l: s(data.margin.x4l),
            x5l: s(data.margin.x5l),
        },
        gap: UiSpacing {
            xs: s(data.gap.xs),
            sm: s(data.gap.sm),
            base: s(data.gap.base),
            lg: s(data.gap.lg),
            xl: s(data.gap.xl),
            x2l: s(data.gap.x2l),
            x3l: s(data.gap.x3l),
            x4l: s(data.gap.x4l),
            x5l: s(data.gap.x5l),
        },
        radius: UiRadius {
            xs: s(data.radius.xs),
            sm: s(data.radius.sm),
            base: s(data.radius.base),
            lg: s(data.radius.lg),
            xl: s(data.radius.xl),
            x2l: s(data.radius.x2l),
            x3l: s(data.radius.x3l),
            x4l: s(data.radius.x4l),
            full: s(data.radius.full),
        },
        border: UiSpacing {
            xs: data.border.xs,
            sm: data.border.sm,
            base: data.border.base,
            lg: data.border.lg,
            xl: data.border.xl,
            x2l: data.border.x2l,
            x3l: data.border.x3l,
            x4l: data.border.x4l,
            x5l: data.border.x5l,
        },
    }
}
