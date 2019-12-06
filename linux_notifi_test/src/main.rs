use x11_dl::xlib;
// use x11_dl::keysym;
use std::mem;
use std::ptr;

// fn h(){

// }

// fn register<CB: 'static + Fn()>(
//     handler: CB,
// ){
//     const INSERT: u32 = keysym::XK_Insert;
//     const modifiers : u32= xlib::Mod1Mask;
//     let xlib = xlib::Xlib::open().unwrap();
//     unsafe{
//         let display = (xlib.XOpenDisplay)(ptr::null());
//         let keycode = (xlib.XKeysymToKeycode)(display, INSERT as u64) as i32;
//         let result = (xlib.XGrabKey)(
//             display,
//             keycode,
//             modifiers,
//             (xlib.XDefaultRootWindow)(display),
//             0,
//             xlib::GrabModeAsync,
//             xlib::GrabModeAsync,
//         );
//         if result == 0 {
//              print!("{}","Failed to register hotkey".to_string());
//         }
//         else{
//             let id = (keycode, modifiers);
//             // handler.insert(id, Box::new(handler))
//         }
//     }

// }
fn main() {
    let xlib = xlib::Xlib::open().unwrap();
    unsafe {
        let display = (xlib.XOpenDisplay)(ptr::null());
        (xlib.XSelectInput)(
            display,
            (xlib.XDefaultRootWindow)(display),
            xlib::KeyReleaseMask,
        );
        let mut event: xlib::XEvent = mem::MaybeUninit::uninit().assume_init();
        loop {
            // print!("Hello");
            // (xlib.XNextEvent)(display, &mut event);
            match event.get_type() {
                xlib::KeyPress => {
                    // if let Some(handler) = self
                    //     .handlers
                    //     .get(&(event.key.keycode as i32, event.key.state))
                    // {
                    //     handler();
                    // }
                }
                _ => {
                    print!("{:?}", event.key.keycode);
                    // break;
                }
            }
        }
    }
    // // register(h);
    // unsafe {
    //     loop {
    //     }
    // }
}
