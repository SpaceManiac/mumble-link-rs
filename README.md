mumble-link [![](https://meritbadge.herokuapp.com/mumble-link)](https://crates.io/crates/mumble-link) [![](https://img.shields.io/badge/docs-online-2020ff.svg)](http://wombat.platymuus.com/rustdoc/mumble_link_master/)
==========

**mumble-link** provides an API for using the [Mumble Link][link] plugin
for position-aware VoIP communications.

[link]: https://wiki.mumble.info/wiki/Link

Connect to Mumble link with `MumbleLink::new()`, set the context or player
identity as needed, and call `update()` every frame with the position data.
