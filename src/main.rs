// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2025 Fundament Research Institute <https://fundament.institute>

use feather_macro::*;
use feather_ui::color::sRGB;
use feather_ui::component::button::Button;
use feather_ui::component::listbox::ListBox;
use feather_ui::component::region::Region;
use feather_ui::component::scroll_area::ScrollArea;
use feather_ui::component::shape;
use feather_ui::component::text::Text;
use feather_ui::component::textbox::TextBox;
use feather_ui::component::window::Window;
use feather_ui::component::{ChildOf, mouse_area, textbox};
use feather_ui::cosmic_text::{FamilyOwned, Wrap};
use feather_ui::layout::{base, fixed, leaf, list};
use feather_ui::persist::FnPersist;
use feather_ui::text::{EditBuffer, EditView};
use feather_ui::ultraviolet::{Vec2, Vec4};
use feather_ui::{
    AbsRect, App, DAbsRect, DPoint, DRect, DataID, FILL_DRECT, RelLimits, RelRect, Slot, SourceID,
    UNSIZED_AXIS, URect, ZERO_DABSRECT, gen_id, im, winit,
};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

#[derive(PartialEq, Clone, Debug, Default)]
struct ConfigState {
    file: String,
    nix: BTreeMap<String, EditView>,
    last: HashMap<String, String>,
}

#[derive(Default, Empty, Area, Anchor, ZIndex, Limits, RLimits, Padding, Margin)]
struct FixedData {
    area: DRect,
    anchor: DPoint,
    limits: feather_ui::DLimits,
    rlimits: feather_ui::RelLimits,
    padding: DAbsRect,
    zindex: i32,
    margin: DRect,
}

impl base::Order for FixedData {}
impl fixed::Prop for FixedData {}
impl fixed::Child for FixedData {}
impl leaf::Prop for FixedData {}
impl leaf::Padded for FixedData {}
impl list::Child for FixedData {}

#[derive(Default, Empty, Area, Direction, RLimits)]
struct ListData {
    area: DRect,
    direction: feather_ui::RowDirection,
    rlimits: feather_ui::RelLimits,
}

impl base::Limits for ListData {}
impl list::Prop for ListData {}
impl fixed::Child for ListData {}

#[derive(Clone, Empty, Area, TextEdit, Padding, Anchor, RLimits)]
struct MinimalText {
    area: DRect,
    padding: DAbsRect,
    textedit: EditView,
    anchor: DPoint,
    rlimits: feather_ui::RelLimits,
}
impl base::Direction for MinimalText {}
impl base::ZIndex for MinimalText {}
impl base::Limits for MinimalText {}
impl leaf::Padded for MinimalText {}
impl leaf::Prop for MinimalText {}
impl fixed::Child for MinimalText {}
impl textbox::Prop for MinimalText {}

struct BasicApp {}

const GREEN: sRGB = sRGB::new(0.2, 0.7, 0.4, 1.0);
const GRAY: sRGB = sRGB::new(0.45, 0.45, 0.45, 1.0);

impl FnPersist<ConfigState, im::HashMap<Arc<SourceID>, Option<Window>>> for BasicApp {
    type Store = (ConfigState, im::HashMap<Arc<SourceID>, Option<Window>>);

    fn init(&self) -> Self::Store {
        (Default::default(), im::HashMap::new())
    }
    fn call(
        &mut self,
        mut store: Self::Store,
        args: &ConfigState,
    ) -> (Self::Store, im::HashMap<Arc<SourceID>, Option<Window>>) {
        if store.0 != *args {
            let mut children: im::Vector<Option<Box<ChildOf<dyn list::Prop>>>> = im::Vector::new();

            for (i, (k, v)) in args.nix.iter().enumerate() {
                let rect = shape::round_rect(
                    gen_id!().child(DataID::Owned(k.to_string())),
                    feather_ui::FILL_DRECT.into(),
                    1.0,
                    0.0,
                    Vec4::broadcast(8.0),
                    sRGB::new(0.1, 0.1, 0.1, 1.0),
                    sRGB::new(0.3, 0.3, 0.3, 1.0),
                );

                let text = Text::<FixedData> {
                    id: gen_id!().child(DataID::Owned(k.to_string())),
                    props: Rc::new(FixedData {
                        area: URect {
                            abs: AbsRect::new(8.0, 0.0, 8.0, 0.0),
                            rel: RelRect::new(0.0, 0.0, UNSIZED_AXIS, UNSIZED_AXIS),
                        }
                        .into(),
                        ..Default::default()
                    }),
                    text: format!("[{k}]"),
                    font_size: 40.0,
                    line_height: 56.0,
                    ..Default::default()
                };

                let toggle = if v.get().get_content().eq_ignore_ascii_case("true") {
                    Some(true)
                } else if v.get().get_content().eq_ignore_ascii_case("false") {
                    Some(false)
                } else {
                    None
                };

                let mut parts: im::Vector<Option<Box<ChildOf<dyn fixed::Prop>>>> =
                    im::Vector::new();
                parts.push_back(Some(Box::new(rect)));
                parts.push_back(Some(Box::new(text)));

                if let Some(v) = toggle {
                    let button = {
                        let rect = shape::round_rect(
                            gen_id!().child(DataID::Owned(k.to_string())),
                            feather_ui::FILL_DRECT.into(),
                            3.0,
                            0.0,
                            Vec4::broadcast(14.0),
                            sRGB::transparent(),
                            if v { GREEN } else { GRAY },
                        );

                        let circle = shape::circle(
                            gen_id!().child(DataID::Owned(k.to_string())),
                            FixedData {
                                area: URect {
                                    abs: AbsRect::new(0.0, 0.0, 20.0, 20.0),
                                    rel: if v {
                                        RelRect::new(1.0, 0.5, 1.0, 0.5)
                                    } else {
                                        RelRect::new(0.0, 0.5, 0.0, 0.5)
                                    },
                                }
                                .into(),
                                anchor: if v {
                                    feather_ui::UPoint::new(
                                        Vec2::new(5.0, 0.0),
                                        feather_ui::RelPoint::new(1.0, 0.5),
                                    )
                                } else {
                                    feather_ui::UPoint::new(
                                        Vec2::new(-5.0, 0.0),
                                        feather_ui::RelPoint::new(0.0, 0.5),
                                    )
                                }
                                .into(),
                                ..Default::default()
                            }
                            .into(),
                            0.0,
                            0.0,
                            Vec2::new(0.0, 0.0),
                            if v { GREEN } else { GRAY },
                            sRGB::transparent(),
                        );

                        Button::<FixedData>::new(
                            gen_id!().child(DataID::Owned(k.to_string())),
                            FixedData {
                                area: URect {
                                    abs: AbsRect::new(-10.0, 0.0, 35.0, 30.0),
                                    rel: RelRect::new(1.0, 0.5, 1.0, 0.5),
                                }
                                .into(),
                                anchor: feather_ui::RelPoint(Vec2 { x: 1.0, y: 0.5 }).into(),
                                ..Default::default()
                            },
                            Slot(feather_ui::APP_SOURCE_ID.into(), i as u64 + 2),
                            feather_ui::children![fixed::Prop, rect, circle],
                        )
                    };
                    parts.push_back(Some(Box::new(button)));
                } else {
                    let textbox = TextBox::new(
                        gen_id!().child(DataID::Owned(k.to_string())),
                        MinimalText {
                            area: URect {
                                abs: AbsRect::new(10.0, 5.0, 10.0, 5.0),
                                rel: RelRect::new(1.0, 0.0, UNSIZED_AXIS, UNSIZED_AXIS),
                            }
                            .into(),
                            anchor: feather_ui::RelPoint::new(1.0, 0.0).into(),
                            rlimits: RelLimits::new(
                                Vec2::broadcast(f32::NEG_INFINITY),
                                Vec2::new(1.0, f32::INFINITY),
                            ),
                            padding: AbsRect::new(4.0, 4.0, 14.0, 4.0).into(),
                            textedit: v.clone(),
                        },
                        30.0,
                        42.0,
                        FamilyOwned::SansSerif,
                        feather_ui::color::sRGB::white(),
                        Default::default(),
                        Default::default(),
                        Wrap::Word,
                    );

                    let rect = shape::round_rect::<DRect>(
                        gen_id!().child(DataID::Owned(k.to_string())),
                        Rc::new(
                            URect {
                                abs: AbsRect::new(6.0, 6.0, -6.0, -6.0),
                                rel: RelRect::new(0.0, 0.0, 1.0, 1.0),
                            }
                            .into(),
                        ),
                        1.0,
                        0.0,
                        Vec4::broadcast(8.0),
                        sRGB::new(0.05, 0.05, 0.05, 1.0),
                        sRGB::new(0.4, 0.4, 0.4, 1.0),
                    );

                    let region = Region::new(
                        gen_id!().child(DataID::Owned(k.to_string())),
                        FixedData {
                            area: URect::from(RelRect::new(1.0, 0.0, UNSIZED_AXIS, UNSIZED_AXIS))
                                .into(),
                            anchor: feather_ui::RelPoint::new(1.0, 0.0).into(),
                            rlimits: RelLimits::new(
                                Vec2::broadcast(f32::NEG_INFINITY),
                                Vec2::new(0.5, f32::INFINITY),
                            ),
                            ..Default::default()
                        }
                        .into(),
                        feather_ui::children![fixed::Prop, rect, textbox],
                    );

                    parts.push_back(Some(Box::new(region)));
                }

                children.push_back(Some(Box::new(Region::new(
                    gen_id!().child(DataID::Owned(k.to_string())),
                    FixedData {
                        area: URect::from(RelRect::new(0.0, 0.0, 1.0, UNSIZED_AXIS)).into(),
                        padding: AbsRect::broadcast(16.0).into(),
                        margin: AbsRect::broadcast(4.0).into(),
                        ..Default::default()
                    }
                    .into(),
                    parts,
                ))));
            }

            let list = ListBox::<ListData>::new(
                gen_id!(),
                ListData {
                    area: URect {
                        abs: AbsRect::new(8.0, 8.0, -8.0, 8.0),
                        rel: RelRect::new(0.0, 0.0, 1.0, UNSIZED_AXIS),
                    }
                    .into(),
                    rlimits: feather_ui::RelLimits::new(
                        Vec2::zero(),
                        Vec2::new(f32::INFINITY, f32::INFINITY),
                    ),
                    direction: feather_ui::RowDirection::TopToBottom,
                }
                .into(),
                children,
            );

            let accept = {
                let text = Text::<FixedData> {
                    id: gen_id!(),
                    props: Rc::new(FixedData {
                        area: URect {
                            abs: AbsRect::new(8.0, 0.0, 8.0, 0.0),
                            rel: RelRect::new(0.0, 0.5, UNSIZED_AXIS, UNSIZED_AXIS),
                        }
                        .into(),
                        anchor: feather_ui::RelPoint(Vec2 { x: 0.0, y: 0.5 }).into(),
                        ..Default::default()
                    }),
                    text: "Accept Changes".into(),
                    font_size: 40.0,
                    line_height: 56.0,
                    ..Default::default()
                };

                let rect = shape::round_rect::<DRect>(
                    gen_id!(),
                    feather_ui::FILL_DRECT.into(),
                    0.0,
                    0.0,
                    Vec4::broadcast(10.0),
                    sRGB::new(0.2, 0.7, 0.4, 1.0),
                    sRGB::transparent(),
                );

                Button::<FixedData>::new(
                    gen_id!(),
                    FixedData {
                        area: URect {
                            abs: AbsRect::new(8.0, 8.0, 8.0, 48.0),
                            rel: RelRect::new(0.0, 0.0, UNSIZED_AXIS, 0.0),
                        }
                        .into(),
                        ..Default::default()
                    },
                    Slot(feather_ui::APP_SOURCE_ID.into(), 0),
                    feather_ui::children![fixed::Prop, rect, text],
                )
            };

            let discard = {
                let text = Text::<FixedData> {
                    id: gen_id!(),
                    props: Rc::new(FixedData {
                        area: URect {
                            abs: AbsRect::new(8.0, 0.0, 8.0, 0.0),
                            rel: RelRect::new(0.0, 0.5, UNSIZED_AXIS, UNSIZED_AXIS),
                        }
                        .into(),
                        anchor: feather_ui::RelPoint(Vec2 { x: 0.0, y: 0.5 }).into(),
                        ..Default::default()
                    }),
                    text: "Discard Changes".into(),
                    font_size: 40.0,
                    line_height: 56.0,
                    ..Default::default()
                };

                let rect = shape::round_rect::<DRect>(
                    gen_id!(),
                    feather_ui::FILL_DRECT.into(),
                    0.0,
                    0.0,
                    Vec4::broadcast(10.0),
                    sRGB::new(0.7, 0.2, 0.4, 1.0),
                    sRGB::transparent(),
                );

                Button::<FixedData>::new(
                    gen_id!(),
                    FixedData {
                        area: URect {
                            abs: AbsRect::new(8.0, 8.0, 8.0, 48.0),
                            rel: RelRect::new(0.5, 0.0, UNSIZED_AXIS, 0.0),
                        }
                        .into(),
                        ..Default::default()
                    },
                    Slot(feather_ui::APP_SOURCE_ID.into(), 1),
                    feather_ui::children![fixed::Prop, rect, text],
                )
            };

            let scrollarea = ScrollArea::new(
                gen_id!(),
                FixedData {
                    area: URect {
                        abs: AbsRect::new(8.0, 68.0, -8.0, -8.0),
                        rel: RelRect::new(0.0, 0.0, 1.0, 1.0),
                    }
                    .into(),
                    ..Default::default()
                },
                (None, Some(40.0)),
                ZERO_DABSRECT,
                feather_ui::children![fixed::Prop, list],
                [None],
            );

            let region = Region::new(
                gen_id!(),
                FixedData {
                    area: FILL_DRECT,
                    ..Default::default()
                }
                .into(),
                feather_ui::children![fixed::Prop, accept, discard, scrollarea],
            );

            let window = Window::new(
                gen_id!(),
                winit::window::Window::default_attributes()
                    .with_title("NixUI")
                    .with_resizable(true),
                Box::new(region),
            );

            store.1 = im::HashMap::new();
            store.1.insert(window.id.clone(), Some(window));
            store.0 = args.clone();
        }
        let windows = store.1.clone();
        (store, windows)
    }
}

use feather_ui::WrapEventEx;

#[allow(clippy::type_complexity)]
fn main() {
    /*
    let optionfile = match nix_data::cache::nixos::nixosoptions().unwrap();
     */
    const TEST_PATH: &str = "configuration.nix";

    let f = std::fs::read_to_string(Path::new(TEST_PATH)).unwrap();
    let nix = nix_editor::parse::get_collection(f.clone()).unwrap();
    let mut buttons: Vec<
        Box<
            (
                dyn FnMut(
                        (u64, Box<(dyn std::any::Any + 'static)>),
                        ConfigState,
                    ) -> Result<ConfigState, ConfigState>
                    + 'static
            ),
        >,
    > = Vec::new();

    buttons.push(Box::new(
        move |_: mouse_area::MouseAreaEvent,
              mut appdata: ConfigState|
              -> Result<ConfigState, ConfigState> {
            let mut s = appdata.file;
            for (k, v) in &appdata.nix {
                if let Some(prev) = appdata.last.get(k) {
                    if prev.eq_ignore_ascii_case(&v.get().get_content()) {
                        continue;
                    }
                }

                s = nix_editor::write::write(&s, &k.to_string(), &v.get().get_content()).unwrap();
            }
            appdata.file = s;

            std::fs::write(TEST_PATH, appdata.file.clone()).unwrap();
            appdata.last = nix_editor::parse::get_collection(appdata.file.clone()).unwrap();
            appdata.nix = appdata
                .last
                .iter()
                .map(|(k, v)| (k.clone(), EditBuffer::new(v, (0, 0)).into()))
                .collect();

            Ok(appdata)
        }
        .wrap(),
    ));

    buttons.push(Box::new(
        move |_: mouse_area::MouseAreaEvent,
              mut appdata: ConfigState|
              -> Result<ConfigState, ConfigState> {
            appdata.nix = appdata
                .last
                .iter()
                .map(|(k, v)| (k.clone(), EditBuffer::new(v, (0, 0)).into()))
                .collect();

            Ok(appdata)
        }
        .wrap(),
    ));

    for i in 0..nix.len() {
        let onclick = Box::new(
            move |_: mouse_area::MouseAreaEvent,
                  mut appdata: ConfigState|
                  -> Result<ConfigState, ConfigState> {
                if let Some((_, v)) = appdata.nix.iter_mut().nth(i) {
                    if v.get().get_content().eq_ignore_ascii_case("true") {
                        v.get().set_content("false");
                    } else if v.get().get_content().eq_ignore_ascii_case("false") {
                        v.get().set_content("true");
                    }
                    *v = v.clone();
                }

                Ok(appdata)
            }
            .wrap(),
        );
        buttons.push(onclick);
    }

    let (mut app, event_loop): (App<ConfigState, BasicApp>, winit::event_loop::EventLoop<()>) =
        App::new(
            ConfigState {
                file: f,
                nix: nix
                    .iter()
                    .map(|(k, v)| (k.clone(), EditBuffer::new(v, (0, 0)).into()))
                    .collect(),
                last: nix,
            },
            buttons,
            BasicApp {},
            |_| (),
        )
        .unwrap();

    event_loop.run_app(&mut app).unwrap();
}
