[![alt text](./astrocat-small.jpg "Astro-cat")](https://io9.gizmodo.com/my-god-its-full-of-cats-the-very-best-artwork-of-cats-1676879337?utm_campaign=socialflow_io9_facebook&utm_source=io9_facebook&utm_medium=socialflow)

catnip
=======

_catnip_ is a TCP/IP stack written in [Rust](https://www.rust-lang.org/) that focuses on being an embeddable, low-latency solution for user-space networking.


features
--------

- low-latency
- fully deterministic
- built with coroutines (instead of explicit state machines)
- polling **async**/**await** with declarative retry policies
```rust
let ack_segment = r#await!(
    TcpPeerState::handshake(state.clone(), cxn.clone()),
    rt.now(),
    Retry::binary_exponential(
        options.tcp.handshake_timeout,
        options.tcp.handshake_retries
    )
)?;
```
- embeddable
- C/C++ interoperability

usage statement
---------------

_catnip_ is prototype code. As such, we provide no guarantees
that it will work and you are assuming any risks with using the code.
We welcome comments and feedback. Please send any questions or
comments to _irene dot zhang at microsoft dot com_ or
_mirobert at microsoft dot com_.  By sending feedback, you are
consenting to your feedback being used in the further development of
this project.