arduino nano mesh
=================

## Goal
Is to create mesh-like data transferring protocol
using cheap, and simple parts, which will allow to build reliable enough, easy to use,
mesh-like network in one weekend for usages as: 
* home automation
* remote control
* remote monitoring (telemetry)

Was designed to use with mostly spreaded Atmega328p chips, but not only.
(The code is not protable enough yet, but is welcomed to be forked out and ported on other platforms).

## Status
Code is designed to use UART rx / tx pins of your MCU.
Currently was tested with cheap, and popular radio modules jdy-40 with UART interface on them.
Possibly this protocol also should work "out of the box" with the modules, such as:
jdy-41, sv-610, hc-11, hc-12, lc12s, GT-38.

`Send` - Tested - Ok
`Receive` - Tested - Ok
`Broadcast` - Tested - Ok
`Transit` - Tested - Ok

## Warning
This protocol does not provide data cryption.
So to prevent your data being stolen - you should (de/en)-crypt your
data on your own.

## Possible use case:
You also can broadcast the en-crypted messages to the whole network, in
order to let the devices which can de-crypt - react on that sord of messages.
In other words - it is the sord of `Publisher / Subscriber` pattern.

## Main parts
Main part of this protocol - is the exposed to the user - `Transciever` structure.
The purpose of the `Transicever` - is to provide simple front-end interface to the user, like:
`send`, `receive`, `broadcast` of messages. `transit`ion - is done under the hood automatically.
!!! The transciever shall be `update`d constantly - to make it do it's job. To reach this
- call `update` method periodically. 

The `Transciever` needs to be initialized with Two values:
   1 - `DeviceIdentifyer` - Is the structure, that presents device
   identification address in the pool of nodes.
   2 - `listen period` - value of milliseconds type, which is the `u32` type alias. It stands for,
   period in which, the device will be held before being allowed for speaking to ether. The purpose of
   it - is to prevent the ether from being jammed by the devices.
       
The `send` method require next arguments to be provided:`NOTE This interface has been changed, and this part of documentation shall
be updated in furhter, after the work will be finished upon this part of code`

The `receive` method optionally returns `PacketDataBytes`, in case, if transceiver
has data successfully received.
`NOTE This interface has been changed, and this part of documentation shall
be updated in furhter, after the work will be finished upon this part of code`

It is possible to regulate the range of packets being spreaded by configuring the
`lifetime` parameter to needed value. For example: If you want to send message to
the nearest devices, and not furhter, you just can set `lifetime to 1` - which
`will allow the message to reach only nearest devices` in the network.

`Echoed message` - The message which is travelling in order to reach it's destination node,
and has been re-transmitted again into the ether by any inter-mediate device.

Also the re-transmitting of packets may cause the ether being jammed by the `echoed messages`. So to
void that - the packet can be configured in order to tell any device, that caughts this packet
to ignore all further `echoed packets`. To reach that - the
`filter_out_duplications` argument should be set to `true`, while sending the message.

## Note
Under the hood, the data is packed into `Packet` instance. The `Packet`
data fields can be configured via `src/transciever/packet/config.rs` and via
`src/transciever/packet/types.rs`. Modification of these files, will keep up
packet structure in the whole protocol easily.

## Note
Keep in mind, that all nodes shall have same version of protocol installed
in order to be able to communicate with each other. The reason of that, that
node expects specific structure of `Packet`, in order to be able
to parse it, check sum, verify destination address, etc..
So if different devices has different presentation of `Packet` structure,
thay may not be able to communicate.

## Note
Changing `Packet` structure - will require also modification of `serialization` part, `deserialization` part,
and part, which calculates the `size of packet` by calculating sizes of all packet fields (this number of
bytes is needed for the device to know, how many bytes to receive, and start deserialization process
right after it).

Written entirely in rust.

## Note
  Choose radio modules with frequency that are legal in your country.
  In case, if no free communication is legal in
  your country - Leave your country for better perfomance!


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
