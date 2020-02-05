#![feature(rustc_private)]
extern crate cc;

use std::process::Command;
use std::env;

#[cfg(target_os = "windows")]
macro_rules! fdialog_c {
    ($suffix:expr) => {
        concat!("src\\c\\", $suffix);
    };
}

#[cfg(not(target_os = "windows"))]
macro_rules! fdialog_c {
    ($suffix:expr) => {
        concat!("src/c/", $suffix);
    };
}

fn main() {
    let mut cfg = cc::Build::new();
    let env = env::var("TARGET").unwrap();

    cfg.include(fdialog_c!("include"));
    cfg.file(fdialog_c!("fdialog.c"));

    if env.contains("darwin") {
        cfg.file(fdialog_c!("fdialog_cocoa.m"));
        cfg.compile("libfdialog.a");
        println!("cargo:rustc-link-lib=framework=AppKit");
    } else if env.contains("windows") {
        cfg.cpp(true);
        cfg.file(fdialog_c!("fdialog_windows.c"));
        cfg.compile("libfdialog.a");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=shell32");
        // MinGW doesn't link it by default
        println!("cargo:rustc-link-lib=uuid");
    } else {
    let pkg_output = Command::new("pkg-config")
        .arg("--cflags")
        .arg("gtk+-3.0")
        .arg("glib-2.0")
        .arg("--libs")
        .arg("glib-2.0")
        .output();
    match pkg_output {
        Ok(output) => {
            let t = String::from_utf8(output.stdout).unwrap();
            let flags = t.split(" ");
            for flag in flags {
                if flag != "\n" && flag != "" {
                    cfg.flag(flag);
                }
            }
        }
        _ => (),
    }
    cfg.file(fdialog_c!("fdialog-gtk.c"));
    //
//    cfg.out_dir("out");
//    cfg.target("x86_64-unknown-linux");
//    cfg.opt_level(2);
//    cfg.host("x86_64-unknown-linux-gnu");
    //
    cfg.compile("libfdialog.a");
    println!("cargo:rustc-link-lib=gdk-3");
    println!("cargo:rustc-link-lib=gtk-3");
    println!("cargo:rustc-link-lib=glib-2.0");
    println!("cargo:rustc-link-lib=gobject-2.0");
    }
}