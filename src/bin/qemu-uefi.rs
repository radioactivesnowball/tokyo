use std::process;
use std::process::Command;

fn main() {
    let mut command = Command::new("qemu-system-x86_64");
    command.arg("-drive").arg(format!("format=raw,file={}", env!("UEFI_IMAGE")));
    command.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
    command.arg("-serial").arg("stdio");
    let exit_status = command.status().unwrap();
    process::exit(exit_status.code().unwrap_or(-1));
}