# vst_plugin_parameters_derive

derive macro for `vst::plugin::PluginParameters` in [vst-rs](https://github.com/RustAudio/vst-rs)

## Example

```rust
use vst::plugin::PluginParameters;
use vst::util::AtomicFloat;
use vst_plugin_parameters_derive::{NumPluginParameters, PluginParameters};

// `NumPluginParameters` is used for tracking the number of total parameters.

#[allow(dead_code)]
#[derive(PluginParameters, NumPluginParameters)]
struct MultipleParams {
    // You can set name, label, text
    #[param(name = "x", label = "label", text = "text")]
    x: AtomicFloat,
    #[param(name = "y")]
    y: AtomicFloat,
    #[param(name = "z")]
    z: AtomicFloat,
}

#[allow(dead_code)]
#[derive(PluginParameters, NumPluginParameters)]
struct NestedParams {
    #[params]
    x: MultipleParams,
    #[params]
    y: MultipleParams,
    #[params]
    z: MultipleParams,
}

impl Default for MultipleParams {
    fn default() -> Self {
        Self {
            x: AtomicFloat::new(0.5),
            y: AtomicFloat::new(0.2),
            z: AtomicFloat::new(0.4),
        }
    }
}

fn main() {
    let p = NestedParams {
        x: Default::default(),
        y: Default::default(),
        z: Default::default(),
    };

    assert_eq!(p.get_parameter_name(0), "x");
    assert_eq!(p.get_parameter_name(1), "y");
    assert_eq!(p.get_parameter_name(2), "z");
    assert_eq!(p.get_parameter_name(3), "x");
    assert_eq!(p.get_parameter_name(4), "y");
    assert_eq!(p.get_parameter_name(5), "z");
    assert_eq!(p.get_parameter_name(6), "x");
    assert_eq!(p.get_parameter_name(7), "y");
    assert_eq!(p.get_parameter_name(8), "z");

    assert_eq!(p.get_parameter(0), 0.5);
    assert_eq!(p.get_parameter(1), 0.2);
    assert_eq!(p.get_parameter(2), 0.4);
    assert_eq!(p.get_parameter(3), 0.5);
    assert_eq!(p.get_parameter(4), 0.2);
    assert_eq!(p.get_parameter(5), 0.4);
    assert_eq!(p.get_parameter(6), 0.5);
    assert_eq!(p.get_parameter(7), 0.2);
    assert_eq!(p.get_parameter(8), 0.4);

    p.set_parameter(0, 0.0);
    assert_eq!(p.get_parameter(0), 0.0);
    p.set_parameter(1, 0.0);
    assert_eq!(p.get_parameter(1), 0.0);
    p.set_parameter(2, 0.0);
    assert_eq!(p.get_parameter(2), 0.0);
}
```