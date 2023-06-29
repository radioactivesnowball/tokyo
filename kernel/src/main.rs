#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(iter_array_chunks)]

mod idt;
mod gdt;
mod acpi;
pub mod graphics;
pub mod alloc;

use core::panic::PanicInfo;
use bootloader_api::BootInfo;
use x86_64::instructions;
use x86_64::instructions::interrupts;
use crate::graphics::Rgb;
use crate::idt::PICS;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // TODO: implement acpi after alloc implementation
    // if let Some(rsdp) = boot_info.rsdp_addr.as_ref() {
    //     acpi::init(*rsdp);
    // }

    // framebuffer
    let mut framebuffer = boot_info.framebuffer.as_mut().expect("framebuffer should exist");
    graphics::init(framebuffer.info(), framebuffer.buffer_mut());

    gdt::init(); // global descriptor table
    idt::init(); // interrupt descriptor table

    unsafe { PICS.lock().initialize(); } // programmable interrupt controller
    interrupts::enable(); // set interrupts

    // test code, remove later
    graphics::use_view(|view| {
        view.clear(Rgb::BLACK);
        view.draw_rect((0, 0), 25, 25, Rgb::RED);
        view.draw_rect((25, 0), 25, 25, Rgb::ORANGE);
        view.draw_rect((50, 0), 25, 25, Rgb::YELLOW);
        view.draw_rect((75, 0), 25, 25, Rgb::GREEN);
        view.draw_rect((100, 0), 25, 25, Rgb::BLUE);
        view.draw_rect((125, 0), 25, 25, Rgb::PURPLE);
    });

    block_indefinitely();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    graphics::use_view(|view| {
        view.clear(Rgb::RED);
    });

    block_indefinitely();
}

fn block_indefinitely() -> ! {
    loop { instructions::hlt(); }
}