arduino nano mesh
=================

## Goal
This is the attempt of creation mesh-like data transferring protocol
using cheap, and simple parts, which has to cost less then 3$ per node.

Was designed to use with mostly spreaded Atmega328p chips, but not only.

## Warning
This protocol does not provide data cryption.

## Main parts
Main part of this protocol - is the `Transciever` structure.
The purpose of the `Transicever` - is to `send`, `receive`, and transit
packets with data which is inside of them.

The `Transciever` needs to be initialized with Two values:
   1 - `DeviceIdentifyer` - Is the structure, that presents device
   identification address in the pool of nodes.
   2 - `listen period` - milliseconds type, which is the `u32` type alias. It stands for,
   period in which, the device will be held from speaking to ether. The purpose of
   it - is to prevent the ether from being jammed by the devices.
       
The `Transciever` instance shall be periodically updated via
   the `update` method. It handles all the internally work, like
   transitting packets, sending packets, that were not being sent yet
   and listen for ether, for incomming data, check it's correctness (checksum)
   and then extract packets payload, in case of packet has reached it's
   destination.

The `send` method require next arguments to be provided:
1. `data` - type of `PacketDataBytes` - which is the alias of heapless::Vec - vector
   of data bytes of specific size, defined in `src/transciever/packet/config.rs`
2. `destication_device_identifyer`: `DeviceIdentifyer` instance. which presents
   the identifyer of receiving device.
3. `lifetime` - The number of intermediate nodes (`Transcievers`) - the packet
   will be able to pass. Each hop between (nodes / devices / 'Transceivers`)
   reduces lifetime of packet by 1. The purpose of it - is to void the ether being
   jammed by lost packets, which might be re-transmitted inifinite number of times.

The `receive` mothod optionally returns `PacketDataBytes`, in case, if transceiver
has data, which was sent exactly to that instance of `Transciever`, received.

## Note
Under the hood, the data is packed into `Packet` instance. The `Packet`
data fields can be configured via `src/transciever/packet/config.rs` and via
`src/transciever/packet/types.rs`. Modification of these files, can help
adjust packet structure on the fly, and the rest of program will keep up
with that changes.

## Note
Keep in mind, that all nodes shall have same version of protocol installed
in order to be able to communicate with each other. The reason of that, that
node expects specific structure of `Packet`, in order to be able
to parse it, check sum, verify destination address, etc..
So if different devices has different presentation of `Packet` structure,
thay may not be able to communicate.

Written entirely in rust.

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
