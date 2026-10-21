use crate::spec::{Arch, Cc, Env, LinkSelfContainedDefault, LinkerFlavor, Os, Target, TargetMetadata, base};

pub(crate) fn target() -> Target {
    let mut options = base::wasm::options();
    options.os = Os::Wasi;
    options.env = Env::P1;
    options.add_pre_link_args(LinkerFlavor::WasmLld(Cc::No), &["-mwasm64"]);
    options.add_pre_link_args(LinkerFlavor::WasmLld(Cc::Yes), &["--target=wasm64-wasip1"]);
    options.link_self_contained = LinkSelfContainedDefault::False;
    options.crt_static_default = true;
    options.crt_static_respected = true;
    options.crt_static_allows_dylibs = true;
    options.entry_name = "__main_void".into();
    options.has_thread_local = false;
    options.features = "+bulk-memory,+mutable-globals,+sign-ext,+nontrapping-fptoint".into();

    Target {
        llvm_target: "wasm64-wasip1".into(),
        metadata: TargetMetadata {
            description: Some("WebAssembly with WASI (64-bit, experimental)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout: "e-m:e-p:64:64-p10:8:8-p20:8:8-i64:64-i128:128-n32:64-S128-ni:1:10:20".into(),
        arch: Arch::Wasm64,
        options,
    }
}
