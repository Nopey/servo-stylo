/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! A list of static preferences exposed to the style crate. These should
//! be kept sync with the preferences used by the style.
#[macro_export]
macro_rules! pref {
    ("browser.display.permit_backplate") => {
        false
    };
    ("browser.display.use_document_fonts") => {
        false
    };
    ("dom.customHighlightAPI.enabled") => {
        false
    };
    ("dom.element.popover.enabled") => {
        false
    };
    ("gfx.font_rendering.opentype_svg.enabled") => {
        false
    };
    ("layout.css.at-scope.enabled") => {
        false
    };
    ("layout.css.basic-shape-rect.enabled") => {
        false
    };
    ("layout.css.basic-shape-shape.enabled") => {
        false
    };
    ("layout.css.basic-shape-xywh.enabled") => {
        false
    };
    ("layout.css.contain-intrinsic-size.enabled") => {
        false
    };
    ("layout.css.container-queries.enabled") => {
        false
    };
    ("layout.css.content-visibility.enabled") => {
        false
    };
    ("layout.css.control-characters.visible") => {
        false
    };
    ("layout.css.cross-fade.enabled") => {
        false
    };
    ("layout.css.element-content-none.enabled") => {
        false
    };
    ("layout.css.fit-content-function.enabled") => {
        false
    };
    ("layout.css.font-palette.enabled") => {
        false
    };
    ("layout.css.font-tech.enabled") => {
        false
    };
    ("layout.css.font-variant-emoji.enabled") => {
        false
    };
    ("layout.css.font-variations.enabled") => {
        false
    };
    ("layout.css.forced-color-adjust.enabled") => {
        false
    };
    ("layout.css.forced-colors.enabled") => {
        false
    };
    ("layout.css.gradient-color-interpolation-method.enabled") => {
        false
    };
    ("layout.css.grid-template-masonry-value.enabled") => {
        false
    };
    ("layout.css.has-selector.enabled") => {
        false
    };
    ("layout.css.import-supports.enabled") => {
        false
    };
    ("layout.css.inverted-colors.enabled") => {
        false
    };
    ("layout.css.light-dark.enabled") => {
        false
    };
    ("layout.css.margin-rules.enabled") => {
        false
    };
    ("layout.css.marker.restricted") => {
        false
    };
    ("layout.css.math-depth.enabled") => {
        false
    };
    ("layout.css.math-style.enabled") => {
        false
    };
    ("layout.css.motion-path-url.enabled") => {
        false
    };
    ("layout.css.moz-control-character-visibility.enabled") => {
        false
    };
    ("layout.css.nesting.enabled") => {
        false
    };
    ("layout.css.overflow-moz-hidden-unscrollable.enabled") => {
        false
    };
    ("layout.css.page-orientation.enabled") => {
        false
    };
    ("layout.css.prefers-contrast.enabled") => {
        false
    };
    ("layout.css.prefers-reduced-transparency.enabled") => {
        false
    };
    ("layout.css.properties-and-values.enabled") => {
        false
    };
    ("layout.css.relative-color-syntax.enabled") => {
        false
    };
    ("layout.css.scroll-driven-animations.enabled") => {
        false
    };
    ("layout.css.size-adjust.enabled") => {
        false
    };
    ("layout.css.starting-style-at-rules.enabled") => {
        false
    };
    ("layout.css.stylo-local-work-queue.in-main-thread") => {
        32
    };
    ("layout.css.stylo-local-work-queue.in-worker") => {
        0
    };
    ("layout.css.stylo-threads") => {
        false
    };
    ("layout.css.stylo-work-unit-size") => {
        16
    };
    ("layout.css.system-ui.enabled") => {
        false
    };
    ("layout.css.text-indent-keywords.enabled") => {
        false
    };
    ("layout.css.transition-behavior.enabled") => {
        false
    };
}
