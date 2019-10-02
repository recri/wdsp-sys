# Rust interface to wdsp software defined radio
Wdsp is a software defined radio library written by Warren Pratt (NR0V) and ported to linux and android by John Melton (g0orx/n6lyt).
# Install wdsp
```
  git clone https://github.com/g0orx/wdsp.git
  cd wdsp
  make
  sudo make install
```
# Generate bindings, if necessary.
```
  bindgen wrapper.h -o src/bindings.rs
```
# Generate and test crate
One of the tests takes a very long time to run as it generates fftw3 wisdom for a large number of large fft sizes
and stores it in ./.wdspWisdom00.  You may want to take a coffee break while it's running.
```
  cargo build
  cargo test
```	
