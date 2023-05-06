use crate::components::context::extend_context_with_overlay;
use crate::components::position::Position;
use leptos::*;

use crate::components::context::LeafletMapContext;

macro_rules! option_effect {
    ($e:ident) => {};
}

#[component(transparent)]
pub fn Marker(
    cx: Scope,
    /// Position for the Marker
    #[prop(into)]
    position: MaybeSignal<Position>,
    #[prop(into, optional)] draggable: MaybeSignal<bool>,
    #[prop(into, optional)] keyboard: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] title: Option<MaybeSignal<String>>,
    #[prop(into, optional)] alt: Option<MaybeSignal<String>>,
    #[prop(into, optional)] interactive: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] z_index_offset: Option<MaybeSignal<f64>>,
    #[prop(into, optional)] opacity: Option<MaybeSignal<f64>>,
    #[prop(into, optional)] rise_on_hover: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] rise_offset: Option<MaybeSignal<f64>>,
    #[prop(into, optional)] pane: Option<MaybeSignal<String>>,
    #[prop(into, optional)] shadow_pane: Option<MaybeSignal<String>>,
    #[prop(into, optional)] bubbling_mouse_events: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] auto_pan: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] auto_pan_padding: Option<MaybeSignal<(u32, u32)>>,
    #[prop(into, optional)] auto_pan_speed: Option<MaybeSignal<f64>>,
    #[prop(into, optional)] icon_url: Option<MaybeSignal<String>>,
    #[prop(into, optional)] icon_size: Option<MaybeSignal<(u32, u32)>>,
    #[prop(into, optional)] attribution: Option<MaybeSignal<String>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let position_tracking = position.clone();
    let map_context = use_context::<LeafletMapContext>(cx).expect("Map context not found");

    let (child, _) = cx.run_child_scope(|cx| {
        let overlay = extend_context_with_overlay(cx);
        create_effect(cx, move |_| {
            if let Some(map) = map_context.map() {
                log!("Adding marker");
                let mut options = leaflet::MarkerOptions::new();
                let drag = draggable.get_untracked();
                if drag {
                    options.draggable(drag);
                }
                if let Some(keyboard) = keyboard {
                    options.keyboard(keyboard.get_untracked());
                }
                if let Some(title) = &title {
                    options.title(&title.get_untracked());
                }
                if let Some(alt) = &alt {
                    options.alt(&alt.get_untracked());
                }
                if let Some(interactive) = interactive {
                    options.interactive(interactive.get_untracked());
                }
                if let Some(z_index_offset) = z_index_offset {
                    options.z_index_offset(z_index_offset.get_untracked());
                }
                if let Some(opacity) = opacity {
                    options.opacity(opacity.get_untracked());
                }
                if let Some(rise_on_hover) = rise_on_hover {
                    options.rise_on_hover(rise_on_hover.get_untracked());
                }
                if let Some(rise_offset) = rise_offset {
                    options.rise_offset(rise_offset.get_untracked());
                }
                if let Some(pane) = &pane {
                    options.pane(&pane.get_untracked());
                }
                if let Some(shadow_pane) = &shadow_pane {
                    options.shadow_pane(&shadow_pane.get_untracked());
                }
                if let Some(bubbling_mouse_events) = bubbling_mouse_events {
                    options.bubbling_mouse_events(bubbling_mouse_events.get_untracked());
                }
                if let Some(auto_pan) = auto_pan {
                    options.auto_pan(auto_pan.get_untracked());
                }
                if let Some(auto_pan) = auto_pan_padding {
                    let (x, y) = auto_pan.get_untracked();
                    options.auto_pan_padding(leaflet::Point::new(x, y));
                }
                if let Some(auto_pan_speed) = auto_pan_speed {
                    options.auto_pan_speed(auto_pan_speed.get_untracked());
                }
                if let Some(icon_url) = &icon_url {
                    let mut icon_options = leaflet::IconOptions::new();
                    icon_options.icon_url(&icon_url.get_untracked());
                    if let Some(size) = icon_size {
                        let (x, y) = size.get_untracked();
                        icon_options.icon_size(leaflet::Point::new(x, y));
                    }
                    let icon = leaflet::Icon::new(&icon_options);
                    options.icon(icon);
                }
                if let Some(attribution) = &attribution {
                    options.attribution(&attribution.get_untracked());
                }
                let marker =
                    leaflet::Marker::newWithOptions(&position.get_untracked().into(), &options);
                marker.addTo(&map);
                overlay.set_container(&marker);

                on_cleanup(cx, move || {
                    marker.remove();
                });
            };
        });

        create_effect(cx, move |_| {
            position_tracking.track();
            if let Some(marker) = overlay.container::<leaflet::Marker>() {
                log!("Updating marker");
                marker.setLatLng(&position_tracking.get_untracked().into());
            }
        });

        create_effect(cx, move |_| {
            if let Some(marker) = overlay.container::<leaflet::Marker>() {
                log!("Updating marker");
                if let Some(opacity) = &opacity {
                    marker.setOpacity(opacity.get());
                }
            }
        });

        create_effect(cx, move |_| {
            if let Some(marker) = overlay.container::<leaflet::Marker>() {
                log!("Changing draggable");
                if draggable.get() {
                    marker.dragging().enable();
                } else {
                    marker.dragging().disable();
                }
            }
        });

        // children
        //     .map(|children| {
        //         children(cx)
        //             .as_children()
        //             .iter()
        //             .map(|child| child.into_view(cx))
        //             .collect::<Vec<_>>()
        //     })
        //     .unwrap_or_default();
        children.map(|child| child(cx))
    });
    child
}