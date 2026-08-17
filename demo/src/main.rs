#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use limine::request::FramebufferRequest;
use segterm::{Terminal, TerminalConfig, fb::Framebuffer};

static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let framebuffer = FRAMEBUFFER_REQUEST
        .response()
        .expect("no framebuffer response")
        .framebuffers()
        .first()
        .copied()
        .expect("no framebuffer");

    segterm::init();

    let mut fb = Framebuffer::new(framebuffer);

    let config = TerminalConfig {
        fg: segterm::Color::WHITE,
        bg: segterm::Color::BLACK,
        ansi: true,
        ..TerminalConfig::default()
    };

    let mut terminal = Terminal::new(&mut fb, config);

    terminal.write("\x1b[32msegterm\x1b[0m initialized!\n");
    terminal.write("\x1b[36mHello from the kernel!\x1b[0m\n");
    terminal.write("Framebuffer: OK\n");
    let mut i: u16 = 0;
    loop {
        write!(terminal, "Test {i}\n").unwrap();
        i += 1;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
