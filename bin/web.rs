use log::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

extern crate console_error_panic_hook;

use web_sys::{Document, Element, HtmlElement, Window};

use crate::boilerplate::Application;

static mut BINDED: bool = false;

pub fn bind_once(mut _application: &mut dyn Application) {
    use std::collections::HashMap;
    use winit::event::{ElementState, KeyboardInput, ModifiersState, VirtualKeyCode as Key};

    unsafe {
        if (BINDED) {
            return;
        }

        BINDED = true;
    }

    let application = unsafe {
        std::mem::transmute::<&mut dyn Application, &'static mut dyn Application>(_application)
    };

    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let canvas = document
        .get_element_by_id("canvas")
        .expect("should have #canvas on the page");

    {
        let mappings: HashMap<String, Key> = [
            (String::from("w"), Key::W),
            (String::from("ц"), Key::W),
            (String::from("a"), Key::A),
            (String::from("a"), Key::A),
            (String::from("s"), Key::S),
            (String::from("ы"), Key::S),
            (String::from("d"), Key::D),
            (String::from("в"), Key::D),
            (String::from("p"), Key::P),
            (String::from("з"), Key::P),
            (String::from("r"), Key::R),
            (String::from("к"), Key::R),
            (String::from("e"), Key::E),
            (String::from("у"), Key::E),
            (String::from("q"), Key::Q),
            (String::from("й"), Key::Q),
            (String::from("shift"), Key::LShift),
            (String::from("alt"), Key::LAlt),
        ]
        .iter()
        .cloned()
        .collect();

        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let pressed = event.type_() == "keydown";
            let key = event.key().to_ascii_lowercase();

            match mappings.get(&key) {
                Some(key) => application.on_key(KeyboardInput {
                    state: if pressed {
                        ElementState::Pressed
                    } else {
                        ElementState::Released
                    },
                    virtual_keycode: Some(*key),
                    modifiers: ModifiersState::default(),
                    scancode: 0,
                }),
                None => false,
            };
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .expect("should be able to bind keydown listener");
        canvas
            .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())
            .expect("should be able to bind keyup listener");
        closure.forget();
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log(a: &str);

    #[wasm_bindgen(js_namespace = Date, js_name = now)]
    pub fn now() -> f64;

    #[wasm_bindgen(js_namespace = window, js_name = getFile)]
    pub fn get_file(file: &str) -> String;
}

fn create_file(file: &str, buf: &[u8]) {
    use std::{fs::File, io::Write};

    info!("Creating file {}", file);
    File::create(file)
        .expect(&format!("Unable to create {}", file))
        .write(buf)
        .expect(&format!("Unable to write in {}", file));
}

#[macro_use]
mod rsrc {
    macro_rules! internal {
        () => {
            "../res_linux/"
        };
    }

    macro_rules! create_file {
        ($path:expr, $src_dir:expr) => {{
            create_file($path, include_bytes!(concat!($src_dir, $path)));
        }};
    }
}

pub fn create_fs() {
    info!("Creating fs");

    let externalFiles = [
        // vange-rs files
        "config/settings.ron",
        "res/shader/quat.inc.glsl",
        "res/shader/debug.wgsl",
        "res/shader/object.wgsl",
        "res/shader/encode.inc.glsl",
        "res/shader/quat.inc.wgsl",
        "res/shader/globals.inc.wgsl",
        "res/shader/downsample.glsl",
        "res/shader/body.inc.wgsl",
        "res/shader/shape.inc.glsl",
        "res/shader/surface.inc.wgsl",
        "res/shader/body.inc.glsl",
        "res/shader/globals.inc.glsl",
        "res/shader/debug_shape.glsl",
        "res/shader/shadow.inc.wgsl",
        "res/shader/surface.inc.glsl",
        "res/shader/color.inc.wgsl",
        "res/shader/terrain/locals.inc.wgsl",
        "res/shader/terrain/ray.wgsl",
        "res/shader/terrain/paint.wgsl",
        "res/shader/terrain/mip.wgsl",
        "res/shader/terrain/scatter.wgsl",
        "res/shader/terrain/slice.wgsl",
        "res/shader/physics/collision.inc.glsl",
        "res/shader/physics/collision_clear.glsl",
        "res/shader/physics/body_step.glsl",
        "res/shader/physics/collision_add.glsl",
        "res/shader/physics/body_push.glsl",
        "res/shader/physics/pulse.inc.glsl",
        "res/shader/physics/body_gather.glsl",
    ];

    for next in externalFiles {
        create_file(next, get_file(next).as_bytes());
    }

    // vangers resouces
    // create_file!("data/device.lst", internal!());
    // create_file!("data/jungle.lst", internal!());
    // create_file!("data/vangers.bmp", internal!());
    // create_file!("data/logfile.txt", internal!());
    // create_file!("data/vangers.prm", internal!());
    // create_file!("data/vangers_mac.bmp", internal!());
    // create_file!("data/tmp_f1.txt", internal!());
    // create_file!("data/item.prm", internal!());
    // create_file!("data/passages.prm", internal!());
    // create_file!("data/diagen.text", internal!());
    // create_file!("data/options.dat", internal!());
    // create_file!("data/hd_background.bmp", internal!());
    // create_file!("data/diagen_eng.text", internal!());
    // create_file!("data/price.prm", internal!());
    // create_file!("data/worlds.prm", internal!());
    create_file!("data/escaves.prm", internal!());
    // create_file!("data/tabutask.prm", internal!());
    // create_file!("data/zfont.fnt", internal!());
    // create_file!("data/road.fnt", internal!());
    // create_file!("data/controls.dat", internal!());
    create_file!("data/common.prm", internal!());
    // create_file!("data/vangers.ico", internal!());
    create_file!("data/bunches.prm", internal!());
    // create_file!("data/spots.prm", internal!());
    create_file!("data/game.lst", internal!());
    create_file!("data/car.prm", internal!());
    create_file!("data/wrlds.dat", internal!());

    // the chain
    create_file!("data/thechain/threall/world.ini", internal!());
    create_file!("data/thechain/threall/output.vmc", internal!());
    create_file!("data/thechain/threall/terrain.prm", internal!());
    create_file!("data/thechain/threall/output.vpr", internal!());
    create_file!("data/thechain/threall/harmony.pal", internal!());

    create_file!("data/thechain/boozeena/world.ini", internal!());
    create_file!("data/thechain/boozeena/output.vmc", internal!());
    create_file!("data/thechain/boozeena/terrain.prm", internal!());
    create_file!("data/thechain/boozeena/output.vpr", internal!());
    create_file!("data/thechain/boozeena/harmony.pal", internal!());

    create_file!("data/thechain/weexow/world.ini", internal!());
    create_file!("data/thechain/weexow/output.vmc", internal!());
    create_file!("data/thechain/weexow/terrain.prm", internal!());
    create_file!("data/thechain/weexow/output.vpr", internal!());
    create_file!("data/thechain/weexow/harmony.pal", internal!());

    create_file!("data/thechain/xplo/world.ini", internal!());
    create_file!("data/thechain/xplo/output.vmc", internal!());
    create_file!("data/thechain/xplo/terrain.prm", internal!());
    create_file!("data/thechain/xplo/output.vpr", internal!());
    create_file!("data/thechain/xplo/harmony.pal", internal!());

    create_file!("data/thechain/hmok/world.ini", internal!());
    create_file!("data/thechain/hmok/output.vmc", internal!());
    create_file!("data/thechain/hmok/terrain.prm", internal!());
    create_file!("data/thechain/hmok/output.vpr", internal!());
    create_file!("data/thechain/hmok/harmony.pal", internal!());

    create_file!("data/thechain/ark-a-znoy/world.ini", internal!());
    create_file!("data/thechain/ark-a-znoy/output.vmc", internal!());
    create_file!("data/thechain/ark-a-znoy/terrain.prm", internal!());
    create_file!("data/thechain/ark-a-znoy/output.vpr", internal!());
    create_file!("data/thechain/ark-a-znoy/harmony.pal", internal!());

    create_file!("data/thechain/khox/world.ini", internal!());
    create_file!("data/thechain/khox/output.vmc", internal!());
    create_file!("data/thechain/khox/terrain.prm", internal!());
    create_file!("data/thechain/khox/output.vpr", internal!());
    create_file!("data/thechain/khox/harmony.pal", internal!());

    // create_file!("data/resource/bml/fireball.bml", internal!());
    // create_file!("data/resource/bml/sign.fnt", internal!());
    // create_file!("data/resource/bml/explos.bml", internal!());
    // create_file!("data/resource/bml/dust.bml", internal!());
    // create_file!("data/resource/bml/compas.bml", internal!());
    // create_file!("data/resource/bml/mole.bml", internal!());
    // create_file!("data/resource/bml/tnt.bml", internal!());
    // create_file!("data/resource/mss/16x16.mss", internal!());
    // create_file!("data/resource/mss/128x128.mss", internal!());
    // create_file!("data/resource/mss/64x64st.mss", internal!());
    // create_file!("data/resource/mss/tpmss.mss", internal!());
    // create_file!("data/resource/mss/064x064.mss", internal!());
    // create_file!("data/resource/mss/out.mss", internal!());
    // create_file!("data/resource/mss/016x016.mss", internal!());
    // create_file!("data/resource/mss/rotor.mss", internal!());
    // create_file!("data/resource/mlvot/fgrib0.vot", internal!());
    // create_file!("data/resource/mlvot/bugtrl4.vot", internal!());
    // create_file!("data/resource/mlvot/bugtrl3.vot", internal!());
    // create_file!("data/resource/mlvot/fgrib1.vot", internal!());
    // create_file!("data/resource/mlvot/bugtrl6.vot", internal!());
    // create_file!("data/resource/mlvot/bugtrl7.vot", internal!());
    // create_file!("data/resource/mlvot/ngrib1.vot", internal!());
    // create_file!("data/resource/mlvot/exptrl2.vot", internal!());
    // create_file!("data/resource/mlvot/exptrl1.vot", internal!());
    // create_file!("data/resource/mlvot/bugtrl0.vot", internal!());
    // create_file!("data/resource/mlvot/ggrib0.vot", internal!());
    // create_file!("data/resource/mlvot/exptrl0.vot", internal!());
    // create_file!("data/resource/mlvot/bugtrl1.vot", internal!());
    // create_file!("data/resource/mlvot/bugtrl2.vot", internal!());
    // create_file!("data/resource/mlvot/bugtrl5.vot", internal!());
    // create_file!("data/resource/mlvot/ggrib1.vot", internal!());
    // create_file!("data/resource/mlvot/exptrl3.vot", internal!());
    create_file!("data/resource/pal/necross.pal", internal!());
    create_file!("data/resource/pal/necross1.pal", internal!());
    create_file!("data/resource/pal/fostral2.pal", internal!());
    create_file!("data/resource/pal/xplo.pal", internal!());
    create_file!("data/resource/pal/necross2.pal", internal!());
    create_file!("data/resource/pal/fostral1.pal", internal!());
    create_file!("data/resource/pal/glorx1.pal", internal!());
    create_file!("data/resource/pal/glorx2.pal", internal!());
    create_file!("data/resource/pal/fostral.pal", internal!());
    create_file!("data/resource/pal/objects.pal", internal!());
    create_file!("data/resource/pal/glorx.pal", internal!());
    // create_file!("data/resource/crypts/crypt9.vlc", internal!());
    // create_file!("data/resource/crypts/crypt3.vlc", internal!());
    // create_file!("data/resource/crypts/crypt4.vlc", internal!());
    // create_file!("data/resource/crypts/crypt10.vlc", internal!());
    // create_file!("data/resource/crypts/crypt1.vlc", internal!());
    // create_file!("data/resource/crypts/crypt8.vlc", internal!());
    // create_file!("data/resource/crypts/crypt7.vlc", internal!());
    // create_file!("data/resource/crypts/crypt6.vlc", internal!());
    // create_file!("data/resource/crypts/skyfarmer.vlc", internal!());
    // create_file!("data/resource/crypts/crypt2.vlc", internal!());
    // create_file!("data/resource/crypts/crypt0.vlc", internal!());
    // create_file!("data/resource/crypts/crypt5.vlc", internal!());
    // create_file!("data/resource/m3d/unique/u4a.m3d", internal!());
    // create_file!("data/resource/m3d/unique/u2b.m3d", internal!());
    // create_file!("data/resource/m3d/unique/u1b.m3d", internal!());
    // create_file!("data/resource/m3d/unique/u2a.m3d", internal!());
    // create_file!("data/resource/m3d/unique/u1a.m3d", internal!());
    // create_file!("data/resource/m3d/unique/u3b.m3d", internal!());
    // create_file!("data/resource/m3d/unique/u3a.m3d", internal!());
    // create_file!("data/resource/m3d/unique/u4b.m3d", internal!());
    // create_file!("data/resource/m3d/unique/u5a.m3d", internal!());
    // create_file!("data/resource/m3d/unique/u5b.m3d", internal!());
    // create_file!("data/resource/m3d/items/i6.m3d", internal!());
    // create_file!("data/resource/m3d/items/i23.m3d", internal!());
    // create_file!("data/resource/m3d/items/i34.m3d", internal!());
    // create_file!("data/resource/m3d/items/i37.m3d", internal!());
    // create_file!("data/resource/m3d/items/i39.m3d", internal!());
    // create_file!("data/resource/m3d/items/i8.m3d", internal!());
    // create_file!("data/resource/m3d/items/i2.m3d", internal!());
    // create_file!("data/resource/m3d/items/i18.m3d", internal!());
    // create_file!("data/resource/m3d/items/i11.m3d", internal!());
    // create_file!("data/resource/m3d/items/i1.m3d", internal!());
    // create_file!("data/resource/m3d/items/i7.m3d", internal!());
    // create_file!("data/resource/m3d/items/i31.m3d", internal!());
    // create_file!("data/resource/m3d/items/i30.m3d", internal!());
    // create_file!("data/resource/m3d/items/i27.m3d", internal!());
    // create_file!("data/resource/m3d/items/i5.m3d", internal!());
    // create_file!("data/resource/m3d/items/i19.m3d", internal!());
    // create_file!("data/resource/m3d/items/i25.m3d", internal!());
    // create_file!("data/resource/m3d/items/i22.m3d", internal!());
    // create_file!("data/resource/m3d/items/i40.m3d", internal!());
    // create_file!("data/resource/m3d/items/i36.m3d", internal!());
    // create_file!("data/resource/m3d/items/i26.m3d", internal!());
    // create_file!("data/resource/m3d/items/i10.m3d", internal!());
    // create_file!("data/resource/m3d/items/i29.m3d", internal!());
    // create_file!("data/resource/m3d/items/i13.m3d", internal!());
    // create_file!("data/resource/m3d/items/i33.m3d", internal!());
    // create_file!("data/resource/m3d/items/i43.m3d", internal!());
    // create_file!("data/resource/m3d/items/i42.m3d", internal!());
    // create_file!("data/resource/m3d/items/i3.m3d", internal!());
    // create_file!("data/resource/m3d/items/i17.m3d", internal!());
    // create_file!("data/resource/m3d/items/i38.m3d", internal!());
    // create_file!("data/resource/m3d/items/i21.m3d", internal!());
    // create_file!("data/resource/m3d/items/i4.m3d", internal!());
    // create_file!("data/resource/m3d/items/i28.m3d", internal!());
    // create_file!("data/resource/m3d/items/i16.m3d", internal!());
    // create_file!("data/resource/m3d/items/i32.m3d", internal!());
    // create_file!("data/resource/m3d/items/i41.m3d", internal!());
    // create_file!("data/resource/m3d/items/i14.m3d", internal!());
    // create_file!("data/resource/m3d/items/i24.m3d", internal!());
    // create_file!("data/resource/m3d/items/i20.m3d", internal!());
    // create_file!("data/resource/m3d/items/i12.m3d", internal!());
    // create_file!("data/resource/m3d/items/i35.m3d", internal!());
    // create_file!("data/resource/m3d/items/i9.m3d", internal!());
    // create_file!("data/resource/m3d/items/i15.m3d", internal!());
    // create_file!("data/resource/m3d/fauna/f2.m3d", internal!());
    // create_file!("data/resource/m3d/fauna/f1.m3d", internal!());
    // create_file!("data/resource/m3d/weapon/w2h.m3d", internal!());
    // create_file!("data/resource/m3d/weapon/w5.m3d", internal!());
    // create_file!("data/resource/m3d/weapon/w4.m3d", internal!());
    // create_file!("data/resource/m3d/weapon/w6.m3d", internal!());
    // create_file!("data/resource/m3d/weapon/w2l.m3d", internal!());
    // create_file!("data/resource/m3d/weapon/w1h.m3d", internal!());
    // create_file!("data/resource/m3d/weapon/w3l.m3d", internal!());
    // create_file!("data/resource/m3d/weapon/w1l.m3d", internal!());
    // create_file!("data/resource/m3d/weapon/w3h.m3d", internal!());
    // create_file!("data/resource/m3d/ammun/am1.m3d", internal!());
    // create_file!("data/resource/m3d/ammun/am2.m3d", internal!());
    // create_file!("data/resource/m3d/ammun/am3.m3d", internal!());
    // create_file!("data/resource/m3d/animated/a2.a3d", internal!());
    // create_file!("data/resource/m3d/animated/a1.a3d", internal!());
    create_file!("data/resource/m3d/mechous/m13.m3d", internal!());
    create_file!("data/resource/m3d/mechous/m10.m3d", internal!());
    create_file!("data/resource/m3d/mechous/m5.prm", internal!());
    create_file!("data/resource/m3d/mechous/m9.m3d", internal!());
    create_file!("data/resource/m3d/mechous/u5.prm", internal!());
    create_file!("data/resource/m3d/mechous/m1.prm", internal!());
    create_file!("data/resource/m3d/mechous/u3.m3d", internal!());
    create_file!("data/resource/m3d/mechous/m11.prm", internal!());
    create_file!("data/resource/m3d/mechous/u2.prm", internal!());
    create_file!("data/resource/m3d/mechous/u1.prm", internal!());
    create_file!("data/resource/m3d/mechous/r2.m3d", internal!());
    create_file!("data/resource/m3d/mechous/r3.m3d", internal!());
    create_file!("data/resource/m3d/mechous/r4.m3d", internal!());
    create_file!("data/resource/m3d/mechous/m3.prm", internal!());
    create_file!("data/resource/m3d/mechous/m14.m3d", internal!());
    create_file!("data/resource/m3d/mechous/m9.prm", internal!());
    create_file!("data/resource/m3d/mechous/m7.prm", internal!());
    create_file!("data/resource/m3d/mechous/m10.prm", internal!());
    create_file!("data/resource/m3d/mechous/default.prm", internal!());
    create_file!("data/resource/m3d/mechous/r1.m3d", internal!());
    create_file!("data/resource/m3d/mechous/m13.prm", internal!());
    create_file!("data/resource/m3d/mechous/m12.prm", internal!());
    create_file!("data/resource/m3d/mechous/r4.prm", internal!());
    create_file!("data/resource/m3d/mechous/m11.m3d", internal!());
    create_file!("data/resource/m3d/mechous/m2.prm", internal!());
    create_file!("data/resource/m3d/mechous/m6.m3d", internal!());
    create_file!("data/resource/m3d/mechous/m12.m3d", internal!());
    create_file!("data/resource/m3d/mechous/u2.m3d", internal!());
    create_file!("data/resource/m3d/mechous/m1.m3d", internal!());
    create_file!("data/resource/m3d/mechous/m4.prm", internal!());
    create_file!("data/resource/m3d/mechous/m8.prm", internal!());
    create_file!("data/resource/m3d/mechous/u1.m3d", internal!());
    create_file!("data/resource/m3d/mechous/m4.m3d", internal!());
    create_file!("data/resource/m3d/mechous/m14.prm", internal!());
    create_file!("data/resource/m3d/mechous/r5.m3d", internal!());
    create_file!("data/resource/m3d/mechous/r3.prm", internal!());
    create_file!("data/resource/m3d/mechous/m3.m3d", internal!());
    create_file!("data/resource/m3d/mechous/r5.prm", internal!());
    create_file!("data/resource/m3d/mechous/u4.m3d", internal!());
    create_file!("data/resource/m3d/mechous/u4.prm", internal!());
    create_file!("data/resource/m3d/mechous/m6.prm", internal!());
    create_file!("data/resource/m3d/mechous/r2.prm", internal!());
    create_file!("data/resource/m3d/mechous/r1.prm", internal!());
    create_file!("data/resource/m3d/mechous/m5.m3d", internal!());
    create_file!("data/resource/m3d/mechous/m7.m3d", internal!());
    create_file!("data/resource/m3d/mechous/m8.m3d", internal!());
    create_file!("data/resource/m3d/mechous/u5.m3d", internal!());
    create_file!("data/resource/m3d/mechous/m2.m3d", internal!());
    create_file!("data/resource/m3d/mechous/u3.prm", internal!());
}
