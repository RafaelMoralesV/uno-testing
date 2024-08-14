with import <nixpkgs> { };
stdenv.mkDerivation {
  name = "avr-stuff";
  buildInputs =
    [ avrdude ravedude usbutils arduino pkgsCross.avr.buildPackages.gcc8 ];
  shellHook = ''
    export PATH=$PATH:${arduino}/share/arduino/hardware/tools/avr/bin/
    export RAVEDUDE_PORT=/dev/ttyUSB0
  '';
}
