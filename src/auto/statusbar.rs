// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::WindowState;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GeditStatusbar")]
    pub struct Statusbar(Object<ffi::GeditStatusbar, ffi::GeditStatusbarClass>) @extends gtk::Statusbar, gtk::Container, gtk::Widget;

    match fn {
        type_ => || ffi::gedit_statusbar_get_type(),
    }
}

impl Statusbar {
    #[doc(alias = "gedit_statusbar_new")]
    pub fn new() -> Statusbar {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::gedit_statusbar_new()).unsafe_cast()
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Statusbar`] objects.
            ///
            /// This method returns an instance of [`StatusbarBuilder`](crate::builders::StatusbarBuilder) which can be used to create [`Statusbar`] objects.
            pub fn builder() -> StatusbarBuilder {
                StatusbarBuilder::default()
            }
        

    #[doc(alias = "gedit_statusbar_clear_overwrite")]
    pub fn clear_overwrite(&self) {
        unsafe {
            ffi::gedit_statusbar_clear_overwrite(self.to_glib_none().0);
        }
    }

    //#[doc(alias = "gedit_statusbar_flash_message")]
    //pub fn flash_message(&self, context_id: u32, format: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:gedit_statusbar_flash_message() }
    //}

    #[doc(alias = "gedit_statusbar_set_overwrite")]
    pub fn set_overwrite(&self, overwrite: bool) {
        unsafe {
            ffi::gedit_statusbar_set_overwrite(self.to_glib_none().0, overwrite.into_glib());
        }
    }

    #[doc(alias = "gedit_statusbar_set_window_state")]
    pub fn set_window_state(&self, state: WindowState, num_of_errors: i32) {
        unsafe {
            ffi::gedit_statusbar_set_window_state(self.to_glib_none().0, state.into_glib(), num_of_errors);
        }
    }
}

impl Default for Statusbar {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Statusbar`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct StatusbarBuilder {
    border_width: Option<u32>,
    child: Option<gtk::Widget>,
    //resize-mode: /*Unknown type*/,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    //events: /*Unknown type*/,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    expand: Option<bool>,
    #[cfg(any(feature = "gtk_v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_20")))]
    focus_on_click: Option<bool>,
    //halign: /*Unknown type*/,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    hexpand: Option<bool>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    margin: Option<i32>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    margin_bottom: Option<i32>,
    #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_12")))]
    margin_end: Option<i32>,
    #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_12")))]
    margin_start: Option<i32>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    #[cfg(any(feature = "gtk_v3_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_8")))]
    opacity: Option<f64>,
    parent: Option<gtk::Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    //style: /*Unknown type*/,
    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    tooltip_markup: Option<String>,
    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    tooltip_text: Option<String>,
    //valign: /*Unknown type*/,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    vexpand: Option<bool>,
    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl StatusbarBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`StatusbarBuilder`].
    pub fn new() -> Self {
        Self::default()
    }


    // rustdoc-stripper-ignore-next
    /// Build the [`Statusbar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Statusbar {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
if let Some(ref border_width) = self.border_width {
                properties.push(("border-width", border_width));
            }
if let Some(ref child) = self.child {
                properties.push(("child", child));
            }
if let Some(ref app_paintable) = self.app_paintable {
                properties.push(("app-paintable", app_paintable));
            }
if let Some(ref can_default) = self.can_default {
                properties.push(("can-default", can_default));
            }
if let Some(ref can_focus) = self.can_focus {
                properties.push(("can-focus", can_focus));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref expand) = self.expand {
                properties.push(("expand", expand));
            }
        #[cfg(any(feature = "gtk_v3_20", feature = "dox"))]
if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
if let Some(ref has_default) = self.has_default {
                properties.push(("has-default", has_default));
            }
if let Some(ref has_focus) = self.has_focus {
                properties.push(("has-focus", has_focus));
            }
        #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
if let Some(ref has_tooltip) = self.has_tooltip {
                properties.push(("has-tooltip", has_tooltip));
            }
if let Some(ref height_request) = self.height_request {
                properties.push(("height-request", height_request));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref hexpand) = self.hexpand {
                properties.push(("hexpand", hexpand));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref hexpand_set) = self.hexpand_set {
                properties.push(("hexpand-set", hexpand_set));
            }
if let Some(ref is_focus) = self.is_focus {
                properties.push(("is-focus", is_focus));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref margin) = self.margin {
                properties.push(("margin", margin));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref margin_bottom) = self.margin_bottom {
                properties.push(("margin-bottom", margin_bottom));
            }
        #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
if let Some(ref margin_end) = self.margin_end {
                properties.push(("margin-end", margin_end));
            }
        #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
if let Some(ref margin_start) = self.margin_start {
                properties.push(("margin-start", margin_start));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref margin_top) = self.margin_top {
                properties.push(("margin-top", margin_top));
            }
if let Some(ref name) = self.name {
                properties.push(("name", name));
            }
if let Some(ref no_show_all) = self.no_show_all {
                properties.push(("no-show-all", no_show_all));
            }
        #[cfg(any(feature = "gtk_v3_8", feature = "dox"))]
if let Some(ref opacity) = self.opacity {
                properties.push(("opacity", opacity));
            }
if let Some(ref parent) = self.parent {
                properties.push(("parent", parent));
            }
if let Some(ref receives_default) = self.receives_default {
                properties.push(("receives-default", receives_default));
            }
if let Some(ref sensitive) = self.sensitive {
                properties.push(("sensitive", sensitive));
            }
        #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
if let Some(ref tooltip_markup) = self.tooltip_markup {
                properties.push(("tooltip-markup", tooltip_markup));
            }
        #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
if let Some(ref tooltip_text) = self.tooltip_text {
                properties.push(("tooltip-text", tooltip_text));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref vexpand) = self.vexpand {
                properties.push(("vexpand", vexpand));
            }
        #[cfg(any(feature = "gtk_v3", feature = "dox"))]
if let Some(ref vexpand_set) = self.vexpand_set {
                properties.push(("vexpand-set", vexpand_set));
            }
if let Some(ref visible) = self.visible {
                properties.push(("visible", visible));
            }
if let Some(ref width_request) = self.width_request {
                properties.push(("width-request", width_request));
            }
        glib::Object::new::<Statusbar>(&properties)

    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child(mut self, child: &impl IsA<gtk::Widget>) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "gtk_v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_20")))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_12")))]
    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    #[cfg(any(feature = "gtk_v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_12")))]
    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    #[cfg(any(feature = "gtk_v3_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3_8")))]
    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent(mut self, parent: &impl IsA<gtk::Container>) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    #[cfg(any(feature = "gtk_v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v2_12")))]
    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    #[cfg(any(feature = "gtk_v3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v3")))]
    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

impl fmt::Display for Statusbar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Statusbar")
    }
}
