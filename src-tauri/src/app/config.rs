use std::any::Any;

struct Partition {
    device: String,
    mode: String,
    efi: String,
    swap: String,
    swap_size: String,
    partitions: Vec<Box<dyn Any>>,
    install_along_partitions: String,
}

struct Bootloader {
    bootloader_type: String,
    location: String,
}

struct Locale {
    locale: Vec<String>,
    timezone: String,
    virtkeymap: String,
    x11keymap: String,
}

struct Networking {
    hostname: String,
    ipv6: String,
}

struct Params {
    cores: i32,
    jobs: i32,
    keep: bool,
}

pub struct Config {
    partition: Partition,
    bootloader: Bootloader,
    locale: Locale,
    networking: Networking,
    users: Box<dyn Any>,
    rootpass: String,
    desktop: String,
    theme: String,
    displayManager: String,
    browser: String,
    packagesStore: Box<dyn Any>,
    extra_packages: Vec<String>,
    kernel: String,
    snapper: String,
    zramd: String,
    hardened: String,
    flatpak: String,
    params: Params,
}
