# gooseboy-tracing
Compatibility layer for [tracing](https://github.com/tokio-rs/tracing) for [Gooseboy](https://github.com/awildergoose/gooseboy-rs)
Simply initialize like this:

```rs
#[gooseboy::main]
fn main() {
    // with the info level
    gooseboy_tracing::init().unwrap();
    // or with a specific level
    gooseboy_tracing::init_with_level(tracing::level_filters::LevelFilter::TRACE).unwrap();
}
```
