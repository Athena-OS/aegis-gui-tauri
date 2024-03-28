# Development Setup

Before giving us any lines of code, please read [The First Time Contributor](./first-time-contributor.md) document. It's short, sweet, and will save other's from headache. Please read it.

We have first party support for **nix** and **flake**. Those are the tools the project use to develop and build this project. Also, our preferred code editor is **VSCodium**.

- [Development Setup](#development-setup)
  - [Linux](#linux)
  - [Windows](#windows)
  - [Walled Garden Operating System](#walled-garden-operating-system)
- [Common Happenings](#common-happenings)
  - [Building for production](#building-for-production)
  - [Prevent Flake from Garbage Collected](#prevent-flake-from-garbage-collected)
  - [GLIB Error](#glib-error)


## Linux

Follow these guide to install all of the programs needed to run the project.

1. Install all the tools.
   1. Install nix to your system. Follow this link https://nixos.org/download.html and do the recommended way to install it.
   2. Enable flake in nix. Refer to https://nixos.wiki/wiki/Flakes for enabling it. If you're new to this, just use `Other Distros, without Home-Manager` section.
   3. Install direnv. Refer to https://direnv.net/docs/installation.html.
      1. If you're on Ubuntu, follow this guide https://gist.github.com/jramnai/0d4a2cc2dcd1484bfde90aad6b6a00bc.
      2. If you're on NixOS, there is already an option for direnv https://search.nixos.org/options?query=programs.direnv. You just need to set `programs.direnv.enable=true;`.
2. Clone this repository to your system using `git clone https://Athena-OS/aegis-gui-tauri.git`.
3. Your direnv should be activated and will give an error. Do `direnv allow`, and all of the dependencies like `yarn`, `cargo`, and `just` will be installed automagically.
4. Let's run the project using `just develop`.

Wait until the build is finish, then TADA! You are 1,000,000 steps ahead of others in the rise of open source in popular, mainstream, society!

## Windows

It's possible to build for this OS, but for now, you must tread this path yourself.

## Walled Garden Operating System

It's possible to build for this OS, but for now, you must tread this path yourself.

# Common Happenings

## Building for production

This project is entirely built on top of nix + flake. Please use `nix build` and see the binary inside of a newly created folder called `result`.

## Prevent Flake from Garbage Collected

Sometimes you want to free up some space with `sudo nix-collect-garbage` and it just so happens to delete the environment used for the project. You can start by using `nix develop --profile dev` to make sure it will not go away when you collect garbage again.

## GLIB Error

This most likely happens if you're not using any famous GTK Program in your system. Will also happens if you're using NixOS. If the program hangs when you click to open a path to your document or get this error,

```bash
GLib-GIO-ERROR **: 08:50:41.565: No GSettings schemas are installed on the system
```
This means you're directly using the program from terminal after you `nix build` it. If you are using NixOS please this lines to your `configuration.nix`.

```nix
  programs.bash = {
    interactiveShellInit = ''
      export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS"
    '';
  };
```