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

pub fn create_fs() {
    info!("Creating fs");

	// vangers-rs resources
	create_file("res/shader/quat.inc.glsl", include_bytes!("../res/shader/quat.inc.glsl"));
	create_file("res/shader/debug.wgsl", include_bytes!("../res/shader/debug.wgsl"));
	create_file("res/shader/object.wgsl", include_bytes!("../res/shader/object.wgsl"));
	create_file("res/shader/encode.inc.glsl", include_bytes!("../res/shader/encode.inc.glsl"));
	create_file("res/shader/quat.inc.wgsl", include_bytes!("../res/shader/quat.inc.wgsl"));
	create_file("res/shader/globals.inc.wgsl", include_bytes!("../res/shader/globals.inc.wgsl"));
	create_file("res/shader/downsample.glsl", include_bytes!("../res/shader/downsample.glsl"));
	create_file("res/shader/body.inc.wgsl", include_bytes!("../res/shader/body.inc.wgsl"));
	create_file("res/shader/shape.inc.glsl", include_bytes!("../res/shader/shape.inc.glsl"));
	create_file("res/shader/surface.inc.wgsl", include_bytes!("../res/shader/surface.inc.wgsl"));
	create_file("res/shader/body.inc.glsl", include_bytes!("../res/shader/body.inc.glsl"));
	create_file("res/shader/globals.inc.glsl", include_bytes!("../res/shader/globals.inc.glsl"));
	create_file("res/shader/debug_shape.glsl", include_bytes!("../res/shader/debug_shape.glsl"));
	create_file("res/shader/shadow.inc.wgsl", include_bytes!("../res/shader/shadow.inc.wgsl"));
	create_file("res/shader/surface.inc.glsl", include_bytes!("../res/shader/surface.inc.glsl"));
	create_file("res/shader/color.inc.wgsl", include_bytes!("../res/shader/color.inc.wgsl"));
	create_file("res/shader/terrain/locals.inc.wgsl", include_bytes!("../res/shader/terrain/locals.inc.wgsl"));
	create_file("res/shader/terrain/ray.wgsl", include_bytes!("../res/shader/terrain/ray.wgsl"));
	create_file("res/shader/terrain/paint.wgsl", include_bytes!("../res/shader/terrain/paint.wgsl"));
	create_file("res/shader/terrain/mip.wgsl", include_bytes!("../res/shader/terrain/mip.wgsl"));
	create_file("res/shader/terrain/scatter.wgsl", include_bytes!("../res/shader/terrain/scatter.wgsl"));
	create_file("res/shader/terrain/slice.wgsl", include_bytes!("../res/shader/terrain/slice.wgsl"));
	create_file("res/shader/physics/collision.inc.glsl", include_bytes!("../res/shader/physics/collision.inc.glsl"));
	create_file("res/shader/physics/collision_clear.glsl", include_bytes!("../res/shader/physics/collision_clear.glsl"));
	create_file("res/shader/physics/body_step.glsl", include_bytes!("../res/shader/physics/body_step.glsl"));
	create_file("res/shader/physics/collision_add.glsl", include_bytes!("../res/shader/physics/collision_add.glsl"));
	create_file("res/shader/physics/body_push.glsl", include_bytes!("../res/shader/physics/body_push.glsl"));
	create_file("res/shader/physics/pulse.inc.glsl", include_bytes!("../res/shader/physics/pulse.inc.glsl"));
	create_file("res/shader/physics/body_gather.glsl", include_bytes!("../res/shader/physics/body_gather.glsl"));

	// vangers resouces
	create_file("config/settings.ron", settings().as_bytes());
	// create_file("data/device.lst", include_bytes!("../res_linux/data/device.lst"));
	// create_file("data/jungle.lst", include_bytes!("../res_linux/data/jungle.lst"));
	// create_file("data/vangers.bmp", include_bytes!("../res_linux/data/vangers.bmp"));
	// create_file("data/logfile.txt", include_bytes!("../res_linux/data/logfile.txt"));
	// create_file("data/vangers.prm", include_bytes!("../res_linux/data/vangers.prm"));
	// create_file("data/vangers_mac.bmp", include_bytes!("../res_linux/data/vangers_mac.bmp"));
	// create_file("data/tmp_f1.txt", include_bytes!("../res_linux/data/tmp_f1.txt"));
	// create_file("data/item.prm", include_bytes!("../res_linux/data/item.prm"));
	// create_file("data/passages.prm", include_bytes!("../res_linux/data/passages.prm"));
	// create_file("data/diagen.text", include_bytes!("../res_linux/data/diagen.text"));
	// create_file("data/options.dat", include_bytes!("../res_linux/data/options.dat"));
	// create_file("data/hd_background.bmp", include_bytes!("../res_linux/data/hd_background.bmp"));
	// create_file("data/diagen_eng.text", include_bytes!("../res_linux/data/diagen_eng.text"));
	// create_file("data/price.prm", include_bytes!("../res_linux/data/price.prm"));
	// create_file("data/worlds.prm", include_bytes!("../res_linux/data/worlds.prm"));
	create_file("data/escaves.prm", include_bytes!("../res_linux/data/escaves.prm"));
	// create_file("data/tabutask.prm", include_bytes!("../res_linux/data/tabutask.prm"));
	// create_file("data/zfont.fnt", include_bytes!("../res_linux/data/zfont.fnt"));
	// create_file("data/road.fnt", include_bytes!("../res_linux/data/road.fnt"));
	// create_file("data/controls.dat", include_bytes!("../res_linux/data/controls.dat"));
	create_file("data/common.prm", include_bytes!("../res_linux/data/common.prm"));
	// create_file("data/vangers.ico", include_bytes!("../res_linux/data/vangers.ico"));
	create_file("data/bunches.prm", include_bytes!("../res_linux/data/bunches.prm"));
	// create_file("data/spots.prm", include_bytes!("../res_linux/data/spots.prm"));
	create_file("data/game.lst", include_bytes!("../res_linux/data/game.lst"));
	create_file("data/car.prm", include_bytes!("../res_linux/data/car.prm"));
	create_file("data/wrlds.dat", include_bytes!("../res_linux/data/wrlds.dat"));

    // the chain
    create_file("data/thechain/threall/world.ini", include_bytes!("../res_linux/data/thechain/threall/world.ini"));        create_file("data/thechain/threall/output.vmc", include_bytes!("../res_linux/data/thechain/threall/output.vmc"));
    create_file("data/thechain/threall/terrain.prm", include_bytes!("../res_linux/data/thechain/threall/terrain.prm"));
    create_file("data/thechain/threall/output.vpr", include_bytes!("../res_linux/data/thechain/threall/output.vpr"));
    create_file("data/thechain/threall/harmony.pal", include_bytes!("../res_linux/data/thechain/threall/harmony.pal"));

    create_file("data/thechain/boozeena/world.ini", include_bytes!("../res_linux/data/thechain/boozeena/world.ini"));
    create_file("data/thechain/boozeena/output.vmc", include_bytes!("../res_linux/data/thechain/boozeena/output.vmc"));
    create_file("data/thechain/boozeena/terrain.prm", include_bytes!("../res_linux/data/thechain/boozeena/terrain.prm"));
    create_file("data/thechain/boozeena/output.vpr", include_bytes!("../res_linux/data/thechain/boozeena/output.vpr"));
    create_file("data/thechain/boozeena/harmony.pal", include_bytes!("../res_linux/data/thechain/boozeena/harmony.pal"));

    create_file("data/thechain/weexow/world.ini", include_bytes!("../res_linux/data/thechain/weexow/world.ini"));
    create_file("data/thechain/weexow/output.vmc", include_bytes!("../res_linux/data/thechain/weexow/output.vmc"));
    create_file("data/thechain/weexow/terrain.prm", include_bytes!("../res_linux/data/thechain/weexow/terrain.prm"));
    create_file("data/thechain/weexow/output.vpr", include_bytes!("../res_linux/data/thechain/weexow/output.vpr"));
    create_file("data/thechain/weexow/harmony.pal", include_bytes!("../res_linux/data/thechain/weexow/harmony.pal"));

    create_file("data/thechain/xplo/world.ini", include_bytes!("../res_linux/data/thechain/xplo/world.ini"));
    create_file("data/thechain/xplo/output.vmc", include_bytes!("../res_linux/data/thechain/xplo/output.vmc"));
    create_file("data/thechain/xplo/terrain.prm", include_bytes!("../res_linux/data/thechain/xplo/terrain.prm"));
    create_file("data/thechain/xplo/output.vpr", include_bytes!("../res_linux/data/thechain/xplo/output.vpr"));
    create_file("data/thechain/xplo/harmony.pal", include_bytes!("../res_linux/data/thechain/xplo/harmony.pal"));
    
    create_file("data/thechain/hmok/world.ini", include_bytes!("../res_linux/data/thechain/hmok/world.ini"));
    create_file("data/thechain/hmok/output.vmc", include_bytes!("../res_linux/data/thechain/hmok/output.vmc"));
    create_file("data/thechain/hmok/terrain.prm", include_bytes!("../res_linux/data/thechain/hmok/terrain.prm"));
    create_file("data/thechain/hmok/output.vpr", include_bytes!("../res_linux/data/thechain/hmok/output.vpr"));
    create_file("data/thechain/hmok/harmony.pal", include_bytes!("../res_linux/data/thechain/hmok/harmony.pal"));
    
    create_file("data/thechain/ark-a-znoy/world.ini", include_bytes!("../res_linux/data/thechain/ark-a-znoy/world.ini"));
    create_file("data/thechain/ark-a-znoy/output.vmc", include_bytes!("../res_linux/data/thechain/ark-a-znoy/output.vmc"));
    create_file("data/thechain/ark-a-znoy/terrain.prm", include_bytes!("../res_linux/data/thechain/ark-a-znoy/terrain.prm"));
    create_file("data/thechain/ark-a-znoy/output.vpr", include_bytes!("../res_linux/data/thechain/ark-a-znoy/output.vpr"));
    create_file("data/thechain/ark-a-znoy/harmony.pal", include_bytes!("../res_linux/data/thechain/ark-a-znoy/harmony.pal"));
    
    create_file("data/thechain/khox/world.ini", include_bytes!("../res_linux/data/thechain/khox/world.ini"));
    create_file("data/thechain/khox/output.vmc", include_bytes!("../res_linux/data/thechain/khox/output.vmc"));
    create_file("data/thechain/khox/terrain.prm", include_bytes!("../res_linux/data/thechain/khox/terrain.prm"));
    create_file("data/thechain/khox/output.vpr", include_bytes!("../res_linux/data/thechain/khox/output.vpr"));
    create_file("data/thechain/khox/harmony.pal", include_bytes!("../res_linux/data/thechain/khox/harmony.pal"));
    
	// create_file("data/resource/bml/fireball.bml", include_bytes!("../res_linux/data/resource/bml/fireball.bml"));
	// create_file("data/resource/bml/sign.fnt", include_bytes!("../res_linux/data/resource/bml/sign.fnt"));
	// create_file("data/resource/bml/explos.bml", include_bytes!("../res_linux/data/resource/bml/explos.bml"));
	// create_file("data/resource/bml/dust.bml", include_bytes!("../res_linux/data/resource/bml/dust.bml"));
	// create_file("data/resource/bml/compas.bml", include_bytes!("../res_linux/data/resource/bml/compas.bml"));
	// create_file("data/resource/bml/mole.bml", include_bytes!("../res_linux/data/resource/bml/mole.bml"));
	// create_file("data/resource/bml/tnt.bml", include_bytes!("../res_linux/data/resource/bml/tnt.bml"));
	// create_file("data/resource/mss/16x16.mss", include_bytes!("../res_linux/data/resource/mss/16x16.mss"));
	// create_file("data/resource/mss/128x128.mss", include_bytes!("../res_linux/data/resource/mss/128x128.mss"));
	// create_file("data/resource/mss/64x64st.mss", include_bytes!("../res_linux/data/resource/mss/64x64st.mss"));
	// create_file("data/resource/mss/tpmss.mss", include_bytes!("../res_linux/data/resource/mss/tpmss.mss"));
	// create_file("data/resource/mss/064x064.mss", include_bytes!("../res_linux/data/resource/mss/064x064.mss"));
	// create_file("data/resource/mss/out.mss", include_bytes!("../res_linux/data/resource/mss/out.mss"));
	// create_file("data/resource/mss/016x016.mss", include_bytes!("../res_linux/data/resource/mss/016x016.mss"));
	// create_file("data/resource/mss/rotor.mss", include_bytes!("../res_linux/data/resource/mss/rotor.mss"));
	// create_file("data/resource/mlvot/fgrib0.vot", include_bytes!("../res_linux/data/resource/mlvot/fgrib0.vot"));
	// create_file("data/resource/mlvot/bugtrl4.vot", include_bytes!("../res_linux/data/resource/mlvot/bugtrl4.vot"));
	// create_file("data/resource/mlvot/bugtrl3.vot", include_bytes!("../res_linux/data/resource/mlvot/bugtrl3.vot"));
	// create_file("data/resource/mlvot/fgrib1.vot", include_bytes!("../res_linux/data/resource/mlvot/fgrib1.vot"));
	// create_file("data/resource/mlvot/bugtrl6.vot", include_bytes!("../res_linux/data/resource/mlvot/bugtrl6.vot"));
	// create_file("data/resource/mlvot/bugtrl7.vot", include_bytes!("../res_linux/data/resource/mlvot/bugtrl7.vot"));
	// create_file("data/resource/mlvot/ngrib1.vot", include_bytes!("../res_linux/data/resource/mlvot/ngrib1.vot"));
	// create_file("data/resource/mlvot/exptrl2.vot", include_bytes!("../res_linux/data/resource/mlvot/exptrl2.vot"));
	// create_file("data/resource/mlvot/exptrl1.vot", include_bytes!("../res_linux/data/resource/mlvot/exptrl1.vot"));
	// create_file("data/resource/mlvot/bugtrl0.vot", include_bytes!("../res_linux/data/resource/mlvot/bugtrl0.vot"));
	// create_file("data/resource/mlvot/ggrib0.vot", include_bytes!("../res_linux/data/resource/mlvot/ggrib0.vot"));
	// create_file("data/resource/mlvot/exptrl0.vot", include_bytes!("../res_linux/data/resource/mlvot/exptrl0.vot"));
	// create_file("data/resource/mlvot/bugtrl1.vot", include_bytes!("../res_linux/data/resource/mlvot/bugtrl1.vot"));
	// create_file("data/resource/mlvot/bugtrl2.vot", include_bytes!("../res_linux/data/resource/mlvot/bugtrl2.vot"));
	// create_file("data/resource/mlvot/bugtrl5.vot", include_bytes!("../res_linux/data/resource/mlvot/bugtrl5.vot"));
	// create_file("data/resource/mlvot/ggrib1.vot", include_bytes!("../res_linux/data/resource/mlvot/ggrib1.vot"));
	// create_file("data/resource/mlvot/exptrl3.vot", include_bytes!("../res_linux/data/resource/mlvot/exptrl3.vot"));
	create_file("data/resource/pal/necross.pal", include_bytes!("../res_linux/data/resource/pal/necross.pal"));
	create_file("data/resource/pal/necross1.pal", include_bytes!("../res_linux/data/resource/pal/necross1.pal"));
	create_file("data/resource/pal/fostral2.pal", include_bytes!("../res_linux/data/resource/pal/fostral2.pal"));
	create_file("data/resource/pal/xplo.pal", include_bytes!("../res_linux/data/resource/pal/xplo.pal"));
	create_file("data/resource/pal/necross2.pal", include_bytes!("../res_linux/data/resource/pal/necross2.pal"));
	create_file("data/resource/pal/fostral1.pal", include_bytes!("../res_linux/data/resource/pal/fostral1.pal"));
	create_file("data/resource/pal/glorx1.pal", include_bytes!("../res_linux/data/resource/pal/glorx1.pal"));
	create_file("data/resource/pal/glorx2.pal", include_bytes!("../res_linux/data/resource/pal/glorx2.pal"));
	create_file("data/resource/pal/fostral.pal", include_bytes!("../res_linux/data/resource/pal/fostral.pal"));
	create_file("data/resource/pal/objects.pal", include_bytes!("../res_linux/data/resource/pal/objects.pal"));
	create_file("data/resource/pal/glorx.pal", include_bytes!("../res_linux/data/resource/pal/glorx.pal"));
	// create_file("data/resource/crypts/crypt9.vlc", include_bytes!("../res_linux/data/resource/crypts/crypt9.vlc"));
	// create_file("data/resource/crypts/crypt3.vlc", include_bytes!("../res_linux/data/resource/crypts/crypt3.vlc"));
	// create_file("data/resource/crypts/crypt4.vlc", include_bytes!("../res_linux/data/resource/crypts/crypt4.vlc"));
	// create_file("data/resource/crypts/crypt10.vlc", include_bytes!("../res_linux/data/resource/crypts/crypt10.vlc"));
	// create_file("data/resource/crypts/crypt1.vlc", include_bytes!("../res_linux/data/resource/crypts/crypt1.vlc"));
	// create_file("data/resource/crypts/crypt8.vlc", include_bytes!("../res_linux/data/resource/crypts/crypt8.vlc"));
	// create_file("data/resource/crypts/crypt7.vlc", include_bytes!("../res_linux/data/resource/crypts/crypt7.vlc"));
	// create_file("data/resource/crypts/crypt6.vlc", include_bytes!("../res_linux/data/resource/crypts/crypt6.vlc"));
	// create_file("data/resource/crypts/skyfarmer.vlc", include_bytes!("../res_linux/data/resource/crypts/skyfarmer.vlc"));
	// create_file("data/resource/crypts/crypt2.vlc", include_bytes!("../res_linux/data/resource/crypts/crypt2.vlc"));
	// create_file("data/resource/crypts/crypt0.vlc", include_bytes!("../res_linux/data/resource/crypts/crypt0.vlc"));
	// create_file("data/resource/crypts/crypt5.vlc", include_bytes!("../res_linux/data/resource/crypts/crypt5.vlc"));
	// create_file("data/resource/m3d/unique/u4a.m3d", include_bytes!("../res_linux/data/resource/m3d/unique/u4a.m3d"));
	// create_file("data/resource/m3d/unique/u2b.m3d", include_bytes!("../res_linux/data/resource/m3d/unique/u2b.m3d"));
	// create_file("data/resource/m3d/unique/u1b.m3d", include_bytes!("../res_linux/data/resource/m3d/unique/u1b.m3d"));
	// create_file("data/resource/m3d/unique/u2a.m3d", include_bytes!("../res_linux/data/resource/m3d/unique/u2a.m3d"));
	// create_file("data/resource/m3d/unique/u1a.m3d", include_bytes!("../res_linux/data/resource/m3d/unique/u1a.m3d"));
	// create_file("data/resource/m3d/unique/u3b.m3d", include_bytes!("../res_linux/data/resource/m3d/unique/u3b.m3d"));
	// create_file("data/resource/m3d/unique/u3a.m3d", include_bytes!("../res_linux/data/resource/m3d/unique/u3a.m3d"));
	// create_file("data/resource/m3d/unique/u4b.m3d", include_bytes!("../res_linux/data/resource/m3d/unique/u4b.m3d"));
	// create_file("data/resource/m3d/unique/u5a.m3d", include_bytes!("../res_linux/data/resource/m3d/unique/u5a.m3d"));
	// create_file("data/resource/m3d/unique/u5b.m3d", include_bytes!("../res_linux/data/resource/m3d/unique/u5b.m3d"));
	// create_file("data/resource/m3d/items/i6.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i6.m3d"));
	// create_file("data/resource/m3d/items/i23.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i23.m3d"));
	// create_file("data/resource/m3d/items/i34.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i34.m3d"));
	// create_file("data/resource/m3d/items/i37.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i37.m3d"));
	// create_file("data/resource/m3d/items/i39.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i39.m3d"));
	// create_file("data/resource/m3d/items/i8.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i8.m3d"));
	// create_file("data/resource/m3d/items/i2.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i2.m3d"));
	// create_file("data/resource/m3d/items/i18.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i18.m3d"));
	// create_file("data/resource/m3d/items/i11.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i11.m3d"));
	// create_file("data/resource/m3d/items/i1.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i1.m3d"));
	// create_file("data/resource/m3d/items/i7.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i7.m3d"));
	// create_file("data/resource/m3d/items/i31.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i31.m3d"));
	// create_file("data/resource/m3d/items/i30.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i30.m3d"));
	// create_file("data/resource/m3d/items/i27.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i27.m3d"));
	// create_file("data/resource/m3d/items/i5.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i5.m3d"));
	// create_file("data/resource/m3d/items/i19.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i19.m3d"));
	// create_file("data/resource/m3d/items/i25.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i25.m3d"));
	// create_file("data/resource/m3d/items/i22.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i22.m3d"));
	// create_file("data/resource/m3d/items/i40.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i40.m3d"));
	// create_file("data/resource/m3d/items/i36.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i36.m3d"));
	// create_file("data/resource/m3d/items/i26.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i26.m3d"));
	// create_file("data/resource/m3d/items/i10.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i10.m3d"));
	// create_file("data/resource/m3d/items/i29.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i29.m3d"));
	// create_file("data/resource/m3d/items/i13.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i13.m3d"));
	// create_file("data/resource/m3d/items/i33.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i33.m3d"));
	// create_file("data/resource/m3d/items/i43.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i43.m3d"));
	// create_file("data/resource/m3d/items/i42.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i42.m3d"));
	// create_file("data/resource/m3d/items/i3.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i3.m3d"));
	// create_file("data/resource/m3d/items/i17.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i17.m3d"));
	// create_file("data/resource/m3d/items/i38.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i38.m3d"));
	// create_file("data/resource/m3d/items/i21.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i21.m3d"));
	// create_file("data/resource/m3d/items/i4.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i4.m3d"));
	// create_file("data/resource/m3d/items/i28.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i28.m3d"));
	// create_file("data/resource/m3d/items/i16.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i16.m3d"));
	// create_file("data/resource/m3d/items/i32.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i32.m3d"));
	// create_file("data/resource/m3d/items/i41.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i41.m3d"));
	// create_file("data/resource/m3d/items/i14.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i14.m3d"));
	// create_file("data/resource/m3d/items/i24.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i24.m3d"));
	// create_file("data/resource/m3d/items/i20.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i20.m3d"));
	// create_file("data/resource/m3d/items/i12.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i12.m3d"));
	// create_file("data/resource/m3d/items/i35.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i35.m3d"));
	// create_file("data/resource/m3d/items/i9.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i9.m3d"));
	// create_file("data/resource/m3d/items/i15.m3d", include_bytes!("../res_linux/data/resource/m3d/items/i15.m3d"));
	// create_file("data/resource/m3d/fauna/f2.m3d", include_bytes!("../res_linux/data/resource/m3d/fauna/f2.m3d"));
	// create_file("data/resource/m3d/fauna/f1.m3d", include_bytes!("../res_linux/data/resource/m3d/fauna/f1.m3d"));
	// create_file("data/resource/m3d/weapon/w2h.m3d", include_bytes!("../res_linux/data/resource/m3d/weapon/w2h.m3d"));
	// create_file("data/resource/m3d/weapon/w5.m3d", include_bytes!("../res_linux/data/resource/m3d/weapon/w5.m3d"));
	// create_file("data/resource/m3d/weapon/w4.m3d", include_bytes!("../res_linux/data/resource/m3d/weapon/w4.m3d"));
	// create_file("data/resource/m3d/weapon/w6.m3d", include_bytes!("../res_linux/data/resource/m3d/weapon/w6.m3d"));
	// create_file("data/resource/m3d/weapon/w2l.m3d", include_bytes!("../res_linux/data/resource/m3d/weapon/w2l.m3d"));
	// create_file("data/resource/m3d/weapon/w1h.m3d", include_bytes!("../res_linux/data/resource/m3d/weapon/w1h.m3d"));
	// create_file("data/resource/m3d/weapon/w3l.m3d", include_bytes!("../res_linux/data/resource/m3d/weapon/w3l.m3d"));
	// create_file("data/resource/m3d/weapon/w1l.m3d", include_bytes!("../res_linux/data/resource/m3d/weapon/w1l.m3d"));
	// create_file("data/resource/m3d/weapon/w3h.m3d", include_bytes!("../res_linux/data/resource/m3d/weapon/w3h.m3d"));
	// create_file("data/resource/m3d/ammun/am1.m3d", include_bytes!("../res_linux/data/resource/m3d/ammun/am1.m3d"));
	// create_file("data/resource/m3d/ammun/am2.m3d", include_bytes!("../res_linux/data/resource/m3d/ammun/am2.m3d"));
	// create_file("data/resource/m3d/ammun/am3.m3d", include_bytes!("../res_linux/data/resource/m3d/ammun/am3.m3d"));
	// create_file("data/resource/m3d/animated/a2.a3d", include_bytes!("../res_linux/data/resource/m3d/animated/a2.a3d"));
	// create_file("data/resource/m3d/animated/a1.a3d", include_bytes!("../res_linux/data/resource/m3d/animated/a1.a3d"));
	create_file("data/resource/m3d/mechous/m13.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/m13.m3d"));
	create_file("data/resource/m3d/mechous/m10.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/m10.m3d"));
	create_file("data/resource/m3d/mechous/m5.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/m5.prm"));
	create_file("data/resource/m3d/mechous/m9.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/m9.m3d"));
	create_file("data/resource/m3d/mechous/u5.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/u5.prm"));
	create_file("data/resource/m3d/mechous/m1.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/m1.prm"));
	create_file("data/resource/m3d/mechous/u3.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/u3.m3d"));
	create_file("data/resource/m3d/mechous/m11.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/m11.prm"));
	create_file("data/resource/m3d/mechous/u2.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/u2.prm"));
	create_file("data/resource/m3d/mechous/u1.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/u1.prm"));
	create_file("data/resource/m3d/mechous/r2.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/r2.m3d"));
	create_file("data/resource/m3d/mechous/r3.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/r3.m3d"));
	create_file("data/resource/m3d/mechous/r4.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/r4.m3d"));
	create_file("data/resource/m3d/mechous/m3.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/m3.prm"));
	create_file("data/resource/m3d/mechous/m14.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/m14.m3d"));
	create_file("data/resource/m3d/mechous/m9.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/m9.prm"));
	create_file("data/resource/m3d/mechous/m7.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/m7.prm"));
	create_file("data/resource/m3d/mechous/m10.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/m10.prm"));
	create_file("data/resource/m3d/mechous/default.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/default.prm"));
	create_file("data/resource/m3d/mechous/r1.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/r1.m3d"));
	create_file("data/resource/m3d/mechous/m13.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/m13.prm"));
	create_file("data/resource/m3d/mechous/m12.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/m12.prm"));
	create_file("data/resource/m3d/mechous/r4.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/r4.prm"));
	create_file("data/resource/m3d/mechous/m11.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/m11.m3d"));
	create_file("data/resource/m3d/mechous/m2.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/m2.prm"));
	create_file("data/resource/m3d/mechous/m6.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/m6.m3d"));
	create_file("data/resource/m3d/mechous/m12.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/m12.m3d"));
	create_file("data/resource/m3d/mechous/u2.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/u2.m3d"));
	create_file("data/resource/m3d/mechous/m1.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/m1.m3d"));
	create_file("data/resource/m3d/mechous/m4.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/m4.prm"));
	create_file("data/resource/m3d/mechous/m8.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/m8.prm"));
	create_file("data/resource/m3d/mechous/u1.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/u1.m3d"));
	create_file("data/resource/m3d/mechous/m4.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/m4.m3d"));
	create_file("data/resource/m3d/mechous/m14.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/m14.prm"));
	create_file("data/resource/m3d/mechous/r5.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/r5.m3d"));
	create_file("data/resource/m3d/mechous/r3.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/r3.prm"));
	create_file("data/resource/m3d/mechous/m3.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/m3.m3d"));
	create_file("data/resource/m3d/mechous/r5.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/r5.prm"));
	create_file("data/resource/m3d/mechous/u4.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/u4.m3d"));
	create_file("data/resource/m3d/mechous/u4.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/u4.prm"));
	create_file("data/resource/m3d/mechous/m6.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/m6.prm"));
	create_file("data/resource/m3d/mechous/r2.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/r2.prm"));
	create_file("data/resource/m3d/mechous/r1.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/r1.prm"));
	create_file("data/resource/m3d/mechous/m5.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/m5.m3d"));
	create_file("data/resource/m3d/mechous/m7.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/m7.m3d"));
	create_file("data/resource/m3d/mechous/m8.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/m8.m3d"));
	create_file("data/resource/m3d/mechous/u5.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/u5.m3d"));
	create_file("data/resource/m3d/mechous/m2.m3d", include_bytes!("../res_linux/data/resource/m3d/mechous/m2.m3d"));
	create_file("data/resource/m3d/mechous/u3.prm", include_bytes!("../res_linux/data/resource/m3d/mechous/u3.prm"));
}