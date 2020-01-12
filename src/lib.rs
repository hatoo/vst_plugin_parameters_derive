pub use vst_plugin_parameters_derive_impl::{NumPluginParameters, PluginParameters};

pub trait NumPluginParameters {
    fn num_parameters() -> i32;
}
