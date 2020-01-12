pub use vst_plugin_parameters_derive_impl::{PluginParameters, NumPluginParameters};

pub trait NumPluginParameters {
    fn num_parameters() -> i32;
}
