use log::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

extern crate console_error_panic_hook;

use web_sys::{Document, Element, HtmlElement, Window };

use crate::boilerplate::Application;

static mut BINDED: bool = false;

pub fn bind_once(mut _application: &mut dyn Application) {
    use std::collections::HashMap;
    use winit::event::{ElementState, KeyboardInput, VirtualKeyCode as Key, ModifiersState};

    unsafe {
        if (BINDED) {
            return;
        }

       BINDED = true;
    }

    let application = unsafe { std::mem::transmute::<&mut dyn Application, &'static mut dyn Application>(_application) };

    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let canvas = document
        .get_element_by_id("canvas")
        .expect("should have #canvas on the page");

    {
        let mappings: HashMap<String, Key> = [
            (String::from("w"), Key::W), (String::from("ц"), Key::W),
            (String::from("a"), Key::A), (String::from("a"), Key::A),
            (String::from("s"), Key::S), (String::from("ы"), Key::S),
            (String::from("d"), Key::D), (String::from("в"), Key::D),
            (String::from("p"), Key::P), (String::from("з"), Key::P),
            (String::from("r"), Key::R), (String::from("к"), Key::R),
            (String::from("e"), Key::E), (String::from("у"), Key::E),
            (String::from("q"), Key::Q), (String::from("й"), Key::Q),
            (String::from("shift"), Key::LShift),
            (String::from("alt"), Key::LAlt),
        ].iter().cloned().collect();

        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let pressed = event.type_() == "keydown";
            let key = event.key().to_ascii_lowercase();

            match mappings.get(&key) {
                Some(key) => application.on_key(KeyboardInput {
                    state: if pressed { ElementState::Pressed } else { ElementState::Released }, 
                    virtual_keycode: Some(*key),
                    modifiers: ModifiersState::default(),
                    scancode: 0,
                }),
                None => {
                    false
                },
            };
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .expect("should be able to bind keydown listener");
        canvas.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())
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

    #[wasm_bindgen(js_namespace = window)]
    pub fn settings() -> String;
}

fn create_file(file: &str, buf: &[u8]) {
    use std::{fs::File, io::Write};

    info!("Creating file {}", file);
    File::create(file).expect(&format!("Unable to create {}", file))
        .write(buf)
        .expect(&format!("Unable to write in {}", file));
}

#[macro_use]
mod rsrc {
    macro_rules! original {
        () => { "../res_linux/" };
    }

    macro_rules! vange_rs {
        () => { "../" };
    }

    macro_rules! create_file {
        ($path:expr, $src_dir:expr) => ({
            create_file(
                $path,
                include_bytes!(
                    concat!($src_dir, $path)
                )
            );
        });
    }
}

pub fn create_fs() {
    info!("Creating fs");

    // vangers-rs resources
    create_file!("shader/quat.inc.glsl", vange_rs!());
    create_file!("shader/debug.wgsl", vange_rs!());
    create_file!("shader/object.wgsl", vange_rs!());
    create_file!("shader/encode.inc.glsl", vange_rs!());
    create_file!("shader/quat.inc.wgsl", vange_rs!());
    create_file!("shader/globals.inc.wgsl", vange_rs!());
    create_file!("shader/downsample.glsl", vange_rs!());
    create_file!("shader/body.inc.wgsl", vange_rs!());
    create_file!("shader/shape.inc.glsl", vange_rs!());
    create_file!("shader/surface.inc.wgsl", vange_rs!());
    create_file!("shader/body.inc.glsl", vange_rs!());
    create_file!("shader/globals.inc.glsl", vange_rs!());
    create_file!("shader/debug_shape.glsl", vange_rs!());
    create_file!("shader/shadow.inc.wgsl", vange_rs!());
    create_file!("shader/surface.inc.glsl", vange_rs!());
    create_file!("shader/color.inc.wgsl", vange_rs!());
    create_file!("shader/terrain/locals.inc.wgsl", vange_rs!());
    create_file!("shader/terrain/ray.wgsl", vange_rs!());
    create_file!("shader/terrain/paint.wgsl", vange_rs!());
    create_file!("shader/terrain/mip.wgsl", vange_rs!());
    create_file!("shader/terrain/scatter.wgsl", vange_rs!());
    create_file!("shader/terrain/slice.wgsl", vange_rs!());
    create_file!("shader/physics/collision.inc.glsl", vange_rs!());
    create_file!("shader/physics/collision_clear.glsl", vange_rs!());
    create_file!("shader/physics/body_step.glsl", vange_rs!());
    create_file!("shader/physics/collision_add.glsl", vange_rs!());
    create_file!("shader/physics/body_push.glsl", vange_rs!());
    create_file!("shader/physics/pulse.inc.glsl", vange_rs!());
    create_file!("shader/physics/body_gather.glsl", vange_rs!());

    // vangers resouces
    create_file("config/settings.ron", settings().as_bytes());
    // create_file!("data/device.lst", original!());
    // create_file!("data/jungle.lst", original!());
    // create_file!("data/vangers.bmp", original!());
    // create_file!("data/logfile.txt", original!());
    // create_file!("data/vangers.prm", original!());
    // create_file!("data/vangers_mac.bmp", original!());
    // create_file!("data/tmp_f1.txt", original!());
    // create_file!("data/item.prm", original!());
    // create_file!("data/passages.prm", original!());
    // create_file!("data/diagen.text", original!());
    // create_file!("data/options.dat", original!());
    // create_file!("data/hd_background.bmp", original!());
    // create_file!("data/diagen_eng.text", original!());
    // create_file!("data/price.prm", original!());
    // create_file!("data/worlds.prm", original!());
    create_file!("data/escaves.prm", original!());
    // create_file!("data/tabutask.prm", original!());
    // create_file!("data/zfont.fnt", original!());
    // create_file!("data/road.fnt", original!());
    // create_file!("data/controls.dat", original!());
    create_file!("data/common.prm", original!());
    // create_file!("data/vangers.ico", original!());
    create_file!("data/bunches.prm", original!());
    // create_file!("data/spots.prm", original!());
    create_file!("data/game.lst", original!());
    create_file!("data/car.prm", original!());
    create_file!("data/wrlds.dat", original!());

    // the chain
    create_file!("data/thechain/threall/world.ini", original!());
    create_file!("data/thechain/threall/output.vmc", original!());
    create_file!("data/thechain/threall/terrain.prm", original!());
    create_file!("data/thechain/threall/output.vpr", original!());
    create_file!("data/thechain/threall/harmony.pal", original!());

    create_file!("data/thechain/boozeena/world.ini", original!());
    create_file!("data/thechain/boozeena/output.vmc", original!());
    create_file!("data/thechain/boozeena/terrain.prm", original!());
    create_file!("data/thechain/boozeena/output.vpr", original!());
    create_file!("data/thechain/boozeena/harmony.pal", original!());

    create_file!("data/thechain/weexow/world.ini", original!());
    create_file!("data/thechain/weexow/output.vmc", original!());
    create_file!("data/thechain/weexow/terrain.prm", original!());
    create_file!("data/thechain/weexow/output.vpr", original!());
    create_file!("data/thechain/weexow/harmony.pal", original!());

    create_file!("data/thechain/xplo/world.ini", original!());
    create_file!("data/thechain/xplo/output.vmc", original!());
    create_file!("data/thechain/xplo/terrain.prm", original!());
    create_file!("data/thechain/xplo/output.vpr", original!());
    create_file!("data/thechain/xplo/harmony.pal", original!());

    create_file!("data/thechain/hmok/world.ini", original!());
    create_file!("data/thechain/hmok/output.vmc", original!());
    create_file!("data/thechain/hmok/terrain.prm", original!());
    create_file!("data/thechain/hmok/output.vpr", original!());
    create_file!("data/thechain/hmok/harmony.pal", original!());

    create_file!("data/thechain/ark-a-znoy/world.ini", original!());
    create_file!("data/thechain/ark-a-znoy/output.vmc", original!());
    create_file!("data/thechain/ark-a-znoy/terrain.prm", original!());
    create_file!("data/thechain/ark-a-znoy/output.vpr", original!());
    create_file!("data/thechain/ark-a-znoy/harmony.pal", original!());

    create_file!("data/thechain/khox/world.ini", original!());
    create_file!("data/thechain/khox/output.vmc", original!());
    create_file!("data/thechain/khox/terrain.prm", original!());
    create_file!("data/thechain/khox/output.vpr", original!());
    create_file!("data/thechain/khox/harmony.pal", original!());

    // create_file!("data/resource/bml/fireball.bml", original!());
    // create_file!("data/resource/bml/sign.fnt", original!());
    // create_file!("data/resource/bml/explos.bml", original!());
    // create_file!("data/resource/bml/dust.bml", original!());
    // create_file!("data/resource/bml/compas.bml", original!());
    // create_file!("data/resource/bml/mole.bml", original!());
    // create_file!("data/resource/bml/tnt.bml", original!());
    // create_file!("data/resource/mss/16x16.mss", original!());
    // create_file!("data/resource/mss/128x128.mss", original!());
    // create_file!("data/resource/mss/64x64st.mss", original!());
    // create_file!("data/resource/mss/tpmss.mss", original!());
    // create_file!("data/resource/mss/064x064.mss", original!());
    // create_file!("data/resource/mss/out.mss", original!());
    // create_file!("data/resource/mss/016x016.mss", original!());
    // create_file!("data/resource/mss/rotor.mss", original!());
    // create_file!("data/resource/mlvot/fgrib0.vot", original!());
    // create_file!("data/resource/mlvot/bugtrl4.vot", original!());
    // create_file!("data/resource/mlvot/bugtrl3.vot", original!());
    // create_file!("data/resource/mlvot/fgrib1.vot", original!());
    // create_file!("data/resource/mlvot/bugtrl6.vot", original!());
    // create_file!("data/resource/mlvot/bugtrl7.vot", original!());
    // create_file!("data/resource/mlvot/ngrib1.vot", original!());
    // create_file!("data/resource/mlvot/exptrl2.vot", original!());
    // create_file!("data/resource/mlvot/exptrl1.vot", original!());
    // create_file!("data/resource/mlvot/bugtrl0.vot", original!());
    // create_file!("data/resource/mlvot/ggrib0.vot", original!());
    // create_file!("data/resource/mlvot/exptrl0.vot", original!());
    // create_file!("data/resource/mlvot/bugtrl1.vot", original!());
    // create_file!("data/resource/mlvot/bugtrl2.vot", original!());
    // create_file!("data/resource/mlvot/bugtrl5.vot", original!());
    // create_file!("data/resource/mlvot/ggrib1.vot", original!());
    // create_file!("data/resource/mlvot/exptrl3.vot", original!());
    create_file!("data/resource/pal/necross.pal", original!());
    create_file!("data/resource/pal/necross1.pal", original!());
    create_file!("data/resource/pal/fostral2.pal", original!());
    create_file!("data/resource/pal/xplo.pal", original!());
    create_file!("data/resource/pal/necross2.pal", original!());
    create_file!("data/resource/pal/fostral1.pal", original!());
    create_file!("data/resource/pal/glorx1.pal", original!());
    create_file!("data/resource/pal/glorx2.pal", original!());
    create_file!("data/resource/pal/fostral.pal", original!());
    create_file!("data/resource/pal/objects.pal", original!());
    create_file!("data/resource/pal/glorx.pal", original!());
    // create_file!("data/resource/crypts/crypt9.vlc", original!());
    // create_file!("data/resource/crypts/crypt3.vlc", original!());
    // create_file!("data/resource/crypts/crypt4.vlc", original!());
    // create_file!("data/resource/crypts/crypt10.vlc", original!());
    // create_file!("data/resource/crypts/crypt1.vlc", original!());
    // create_file!("data/resource/crypts/crypt8.vlc", original!());
    // create_file!("data/resource/crypts/crypt7.vlc", original!());
    // create_file!("data/resource/crypts/crypt6.vlc", original!());
    // create_file!("data/resource/crypts/skyfarmer.vlc", original!());
    // create_file!("data/resource/crypts/crypt2.vlc", original!());
    // create_file!("data/resource/crypts/crypt0.vlc", original!());
    // create_file!("data/resource/crypts/crypt5.vlc", original!());
    // create_file!("data/resource/m3d/unique/u4a.m3d", original!());
    // create_file!("data/resource/m3d/unique/u2b.m3d", original!());
    // create_file!("data/resource/m3d/unique/u1b.m3d", original!());
    // create_file!("data/resource/m3d/unique/u2a.m3d", original!());
    // create_file!("data/resource/m3d/unique/u1a.m3d", original!());
    // create_file!("data/resource/m3d/unique/u3b.m3d", original!());
    // create_file!("data/resource/m3d/unique/u3a.m3d", original!());
    // create_file!("data/resource/m3d/unique/u4b.m3d", original!());
    // create_file!("data/resource/m3d/unique/u5a.m3d", original!());
    // create_file!("data/resource/m3d/unique/u5b.m3d", original!());
    // create_file!("data/resource/m3d/items/i6.m3d", original!());
    // create_file!("data/resource/m3d/items/i23.m3d", original!());
    // create_file!("data/resource/m3d/items/i34.m3d", original!());
    // create_file!("data/resource/m3d/items/i37.m3d", original!());
    // create_file!("data/resource/m3d/items/i39.m3d", original!());
    // create_file!("data/resource/m3d/items/i8.m3d", original!());
    // create_file!("data/resource/m3d/items/i2.m3d", original!());
    // create_file!("data/resource/m3d/items/i18.m3d", original!());
    // create_file!("data/resource/m3d/items/i11.m3d", original!());
    // create_file!("data/resource/m3d/items/i1.m3d", original!());
    // create_file!("data/resource/m3d/items/i7.m3d", original!());
    // create_file!("data/resource/m3d/items/i31.m3d", original!());
    // create_file!("data/resource/m3d/items/i30.m3d", original!());
    // create_file!("data/resource/m3d/items/i27.m3d", original!());
    // create_file!("data/resource/m3d/items/i5.m3d", original!());
    // create_file!("data/resource/m3d/items/i19.m3d", original!());
    // create_file!("data/resource/m3d/items/i25.m3d", original!());
    // create_file!("data/resource/m3d/items/i22.m3d", original!());
    // create_file!("data/resource/m3d/items/i40.m3d", original!());
    // create_file!("data/resource/m3d/items/i36.m3d", original!());
    // create_file!("data/resource/m3d/items/i26.m3d", original!());
    // create_file!("data/resource/m3d/items/i10.m3d", original!());
    // create_file!("data/resource/m3d/items/i29.m3d", original!());
    // create_file!("data/resource/m3d/items/i13.m3d", original!());
    // create_file!("data/resource/m3d/items/i33.m3d", original!());
    // create_file!("data/resource/m3d/items/i43.m3d", original!());
    // create_file!("data/resource/m3d/items/i42.m3d", original!());
    // create_file!("data/resource/m3d/items/i3.m3d", original!());
    // create_file!("data/resource/m3d/items/i17.m3d", original!());
    // create_file!("data/resource/m3d/items/i38.m3d", original!());
    // create_file!("data/resource/m3d/items/i21.m3d", original!());
    // create_file!("data/resource/m3d/items/i4.m3d", original!());
    // create_file!("data/resource/m3d/items/i28.m3d", original!());
    // create_file!("data/resource/m3d/items/i16.m3d", original!());
    // create_file!("data/resource/m3d/items/i32.m3d", original!());
    // create_file!("data/resource/m3d/items/i41.m3d", original!());
    // create_file!("data/resource/m3d/items/i14.m3d", original!());
    // create_file!("data/resource/m3d/items/i24.m3d", original!());
    // create_file!("data/resource/m3d/items/i20.m3d", original!());
    // create_file!("data/resource/m3d/items/i12.m3d", original!());
    // create_file!("data/resource/m3d/items/i35.m3d", original!());
    // create_file!("data/resource/m3d/items/i9.m3d", original!());
    // create_file!("data/resource/m3d/items/i15.m3d", original!());
    // create_file!("data/resource/m3d/fauna/f2.m3d", original!());
    // create_file!("data/resource/m3d/fauna/f1.m3d", original!());
    // create_file!("data/resource/m3d/weapon/w2h.m3d", original!());
    // create_file!("data/resource/m3d/weapon/w5.m3d", original!());
    // create_file!("data/resource/m3d/weapon/w4.m3d", original!());
    // create_file!("data/resource/m3d/weapon/w6.m3d", original!());
    // create_file!("data/resource/m3d/weapon/w2l.m3d", original!());
    // create_file!("data/resource/m3d/weapon/w1h.m3d", original!());
    // create_file!("data/resource/m3d/weapon/w3l.m3d", original!());
    // create_file!("data/resource/m3d/weapon/w1l.m3d", original!());
    // create_file!("data/resource/m3d/weapon/w3h.m3d", original!());
    // create_file!("data/resource/m3d/ammun/am1.m3d", original!());
    // create_file!("data/resource/m3d/ammun/am2.m3d", original!());
    // create_file!("data/resource/m3d/ammun/am3.m3d", original!());
    // create_file!("data/resource/m3d/animated/a2.a3d", original!());
    // create_file!("data/resource/m3d/animated/a1.a3d", original!());
    create_file!("data/resource/m3d/mechous/m13.m3d", original!());
    create_file!("data/resource/m3d/mechous/m10.m3d", original!());
    create_file!("data/resource/m3d/mechous/m5.prm", original!());
    create_file!("data/resource/m3d/mechous/m9.m3d", original!());
    create_file!("data/resource/m3d/mechous/u5.prm", original!());
    create_file!("data/resource/m3d/mechous/m1.prm", original!());
    create_file!("data/resource/m3d/mechous/u3.m3d", original!());
    create_file!("data/resource/m3d/mechous/m11.prm", original!());
    create_file!("data/resource/m3d/mechous/u2.prm", original!());
    create_file!("data/resource/m3d/mechous/u1.prm", original!());
    create_file!("data/resource/m3d/mechous/r2.m3d", original!());
    create_file!("data/resource/m3d/mechous/r3.m3d", original!());
    create_file!("data/resource/m3d/mechous/r4.m3d", original!());
    create_file!("data/resource/m3d/mechous/m3.prm", original!());
    create_file!("data/resource/m3d/mechous/m14.m3d", original!());
    create_file!("data/resource/m3d/mechous/m9.prm", original!());
    create_file!("data/resource/m3d/mechous/m7.prm", original!());
    create_file!("data/resource/m3d/mechous/m10.prm", original!());
    create_file!("data/resource/m3d/mechous/default.prm", original!());
    create_file!("data/resource/m3d/mechous/r1.m3d", original!());
    create_file!("data/resource/m3d/mechous/m13.prm", original!());
    create_file!("data/resource/m3d/mechous/m12.prm", original!());
    create_file!("data/resource/m3d/mechous/r4.prm", original!());
    create_file!("data/resource/m3d/mechous/m11.m3d", original!());
    create_file!("data/resource/m3d/mechous/m2.prm", original!());
    create_file!("data/resource/m3d/mechous/m6.m3d", original!());
    create_file!("data/resource/m3d/mechous/m12.m3d", original!());
    create_file!("data/resource/m3d/mechous/u2.m3d", original!());
    create_file!("data/resource/m3d/mechous/m1.m3d", original!());
    create_file!("data/resource/m3d/mechous/m4.prm", original!());
    create_file!("data/resource/m3d/mechous/m8.prm", original!());
    create_file!("data/resource/m3d/mechous/u1.m3d", original!());
    create_file!("data/resource/m3d/mechous/m4.m3d", original!());
    create_file!("data/resource/m3d/mechous/m14.prm", original!());
    create_file!("data/resource/m3d/mechous/r5.m3d", original!());
    create_file!("data/resource/m3d/mechous/r3.prm", original!());
    create_file!("data/resource/m3d/mechous/m3.m3d", original!());
    create_file!("data/resource/m3d/mechous/r5.prm", original!());
    create_file!("data/resource/m3d/mechous/u4.m3d", original!());
    create_file!("data/resource/m3d/mechous/u4.prm", original!());
    create_file!("data/resource/m3d/mechous/m6.prm", original!());
    create_file!("data/resource/m3d/mechous/r2.prm", original!());
    create_file!("data/resource/m3d/mechous/r1.prm", original!());
    create_file!("data/resource/m3d/mechous/m5.m3d", original!());
    create_file!("data/resource/m3d/mechous/m7.m3d", original!());
    create_file!("data/resource/m3d/mechous/m8.m3d", original!());
    create_file!("data/resource/m3d/mechous/u5.m3d", original!());
    create_file!("data/resource/m3d/mechous/m2.m3d", original!());
    create_file!("data/resource/m3d/mechous/u3.prm", original!());
}
