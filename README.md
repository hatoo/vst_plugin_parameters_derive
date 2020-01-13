# vst_plugin_parameters_derive

derive macro for `vst::plugin::PluginParameters` in [vst-rs](https://github.com/RustAudio/vst-rs)

## Example

```rust
use vst::plugin::PluginParameters;
use vst::util::AtomicFloat;
use vst_plugin_parameters_derive::{NumPluginParameters, PluginParameters};
// `NumPluginParameters` is used for tracking the number of total parameters.

#[test]
fn demo() {
    #[allow(dead_code)]
    #[derive(PluginParameters, NumPluginParameters)]
    struct Params {
        // You can parameter's name.
        #[param(name = "custom name")]
        x: AtomicFloat,
        // You can parameter's label.
        #[param(label = "label")]
        y: AtomicFloat,
        #[param]
        z: AtomicFloat,
    }

    assert_eq!(Params::num_parameters(), 3);

    let p = Params {
        x: AtomicFloat::new(0.5),
        y: AtomicFloat::new(0.2),
        z: AtomicFloat::new(0.4),
    };

    assert_eq!(p.get_parameter_name(0), "custom name");
    assert_eq!(p.get_parameter_name(1), "y");
    assert_eq!(p.get_parameter_name(2), "z");

    assert_eq!(p.get_parameter(0), 0.5);
    assert_eq!(p.get_parameter(1), 0.2);
    assert_eq!(p.get_parameter(2), 0.4);

    #[allow(dead_code)]
    #[derive(PluginParameters, NumPluginParameters)]
    struct NestedParams {
        #[params]
        p1: Params,
        #[params(prefix = "prefix ")]
        p2: Params,
    }

    let p = NestedParams {
        p1: Params {
            x: AtomicFloat::new(0.5),
            y: AtomicFloat::new(0.2),
            z: AtomicFloat::new(0.4),
        },
        p2: Params {
            x: AtomicFloat::new(0.5),
            y: AtomicFloat::new(0.2),
            z: AtomicFloat::new(0.4),
        }
    };

    assert_eq!(p.get_parameter_name(0), "custom name");
    assert_eq!(p.get_parameter_name(1), "y");
    assert_eq!(p.get_parameter_name(2), "z");
    assert_eq!(p.get_parameter_name(3), "prefix custom name");
    assert_eq!(p.get_parameter_name(4), "prefix y");
    assert_eq!(p.get_parameter_name(5), "prefix z");
}
```