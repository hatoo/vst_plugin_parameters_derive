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
        // You can set prefix for name.
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
        },
    };

    assert_eq!(p.get_parameter_name(0), "custom name");
    assert_eq!(p.get_parameter_name(1), "y");
    assert_eq!(p.get_parameter_name(2), "z");
    assert_eq!(p.get_parameter_name(3), "prefix custom name");
    assert_eq!(p.get_parameter_name(4), "prefix y");
    assert_eq!(p.get_parameter_name(5), "prefix z");
}

#[test]
fn it_works() {
    #[allow(dead_code)]
    #[derive(PluginParameters, NumPluginParameters)]
    struct Foo {}
}

#[test]
fn num_single_param() {
    #[derive(PluginParameters, NumPluginParameters)]
    struct OneParam {
        #[param]
        x: AtomicFloat,
    }

    assert_eq!(OneParam::num_parameters(), 1);
}

#[test]
fn num_multiple_param() {
    #[allow(dead_code)]
    #[derive(PluginParameters, NumPluginParameters)]
    struct MultipleParam {
        #[param]
        x: AtomicFloat,
        #[param]
        y: AtomicFloat,
        #[param]
        z: AtomicFloat,
    }

    assert_eq!(MultipleParam::num_parameters(), 3);
}

#[test]
fn num_nest_params() {
    #[allow(dead_code)]
    #[derive(PluginParameters, NumPluginParameters)]
    struct MultipleParam {
        #[param]
        x: AtomicFloat,
        #[param]
        y: AtomicFloat,
        #[param]
        z: AtomicFloat,
    }

    #[allow(dead_code)]
    #[derive(PluginParameters, NumPluginParameters)]
    struct NestParams {
        #[params]
        p1: MultipleParam,
        #[params]
        p2: MultipleParam,
        #[params]
        p3: MultipleParam,
    }

    assert_eq!(NestParams::num_parameters(), 9);
}

#[test]
fn single_param() {
    #[allow(dead_code)]
    #[derive(PluginParameters, NumPluginParameters)]
    struct OneParam {
        #[param(name = "some_awesome_name", label = "label")]
        x: AtomicFloat,
    }

    let p = OneParam {
        x: AtomicFloat::new(0.5),
    };

    assert_eq!(p.get_parameter_name(0), "some_awesome_name");
    assert_eq!(p.get_parameter_label(0), "label");
    assert_eq!(p.get_parameter_text(0), "0.500");
    assert_eq!(p.get_parameter(0), 0.5);
    p.set_parameter(0, 0.0);
    assert_eq!(p.get_parameter(0), 0.0);

    #[allow(dead_code)]
    #[derive(PluginParameters, NumPluginParameters)]
    struct OneParamAutoName {
        #[param]
        x: AtomicFloat,
    }

    let p = OneParamAutoName {
        x: AtomicFloat::new(0.5),
    };

    assert_eq!(p.get_parameter_name(0), "x");
}

#[test]
fn multiple_params() {
    #[allow(dead_code)]
    #[derive(PluginParameters, NumPluginParameters)]
    struct MultipleParams {
        #[param(name = "x")]
        x: AtomicFloat,
        #[param(name = "y")]
        y: AtomicFloat,
        #[param(name = "z")]
        z: AtomicFloat,
    }

    let p = MultipleParams {
        x: AtomicFloat::new(0.5),
        y: AtomicFloat::new(0.2),
        z: AtomicFloat::new(0.4),
    };

    assert_eq!(p.get_parameter_name(0), "x");
    assert_eq!(p.get_parameter_name(1), "y");
    assert_eq!(p.get_parameter_name(2), "z");

    assert_eq!(p.get_parameter(0), 0.5);
    assert_eq!(p.get_parameter(1), 0.2);
    assert_eq!(p.get_parameter(2), 0.4);

    p.set_parameter(0, 0.0);
    assert_eq!(p.get_parameter(0), 0.0);
    p.set_parameter(1, 0.0);
    assert_eq!(p.get_parameter(1), 0.0);
    p.set_parameter(2, 0.0);
    assert_eq!(p.get_parameter(2), 0.0);
}

#[test]
fn nested_params() {
    #[allow(dead_code)]
    #[derive(PluginParameters, NumPluginParameters)]
    struct MultipleParams {
        #[param(name = "x")]
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

#[test]
fn test_prefix() {
    #[allow(dead_code)]
    #[derive(PluginParameters, NumPluginParameters)]
    struct MultipleParams {
        #[param]
        x: AtomicFloat,
        #[param]
        y: AtomicFloat,
        #[param]
        z: AtomicFloat,
    }

    #[allow(dead_code)]
    #[derive(PluginParameters, NumPluginParameters)]
    struct NestedParams {
        #[params(prefix = "params1 ")]
        x: MultipleParams,
        #[params(prefix = "params2 ")]
        y: MultipleParams,
        #[params(prefix = "params3 ")]
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

    let p = NestedParams {
        x: Default::default(),
        y: Default::default(),
        z: Default::default(),
    };

    assert_eq!(p.get_parameter_name(0), "params1 x");
    assert_eq!(p.get_parameter_name(1), "params1 y");
    assert_eq!(p.get_parameter_name(2), "params1 z");
    assert_eq!(p.get_parameter_name(3), "params2 x");
    assert_eq!(p.get_parameter_name(4), "params2 y");
    assert_eq!(p.get_parameter_name(5), "params2 z");
    assert_eq!(p.get_parameter_name(6), "params3 x");
    assert_eq!(p.get_parameter_name(7), "params3 y");
    assert_eq!(p.get_parameter_name(8), "params3 z");
}
