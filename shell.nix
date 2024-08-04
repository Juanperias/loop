{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell
{
  buildInputs = with pkgs; [
    gnumake
    nasm
    rustup
    cowsay
    qemu
    coreboot-toolchain.i386
  ];

  shellHook = '' 
    rustup default nightly
    rustup target add i686-unknown-linux-gnu
    cowsay "Building loop!"
    make
    cowsay "Running loop!"
    qemu-system-i386 -drive format=raw,file=build/loop.bin
  '';
}
