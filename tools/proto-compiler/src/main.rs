use std::path::PathBuf;
use anyhow::Result;

fn main() -> Result<()> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let crates_root = root.join("..").join("..").join("crates");
    let proto_root = crates_root.join("proto");
    let proto_src_gen = proto_root.join("src").join("gen");
    let cnidarium_root = crates_root.join("cnidarium");
    let cnidarium_src_gen = cnidarium_root.join("src").join("gen");

    println!("root: {}", root.display());
    println!("proto_src_gen: {}", proto_src_gen.display());
    println!("cnidarium_src_gen: {}", cnidarium_src_gen.display());

    let descriptor_file_name = "proto_descriptor.bin.no_lfs";

    let mut config = prost_build::Config::new();
    configure_prost(&mut config);

    let mut cnidarium_config = prost_build::Config::new();
    configure_prost(&mut cnidarium_config);

    build_proto_files(&config, &proto_src_gen, descriptor_file_name, false)?;
    build_proto_files(&cnidarium_config, &cnidarium_src_gen, descriptor_file_name, true)?;

    build_pbjson(&proto_src_gen, descriptor_file_name)?;
    build_pbjson(&cnidarium_src_gen, descriptor_file_name)?;

    Ok(())
}

fn configure_prost(config: &mut prost_build::Config) {
    config.compile_well_known_types();
    config.extern_path(".google.protobuf", "::pbjson_types");
    config.extern_path(".ibc", "::ibc_proto::ibc");
    config.extern_path(".ics23", "::ics23");
    config.extern_path(".cosmos.ics23", "::ics23");
}

fn build_proto_files(
    config: &prost_build::Config,
    target_dir: &PathBuf,
    descriptor_file_name: &str,
    is_cnidarium: bool,
) -> Result<()> {
    let descriptor_file_path = target_dir.join(descriptor_file_name);
    config.out_dir(target_dir)
        .file_descriptor_set_path(&descriptor_file_path)
        .enable_type_names();

    let proto_files = if is_cnidarium {
        vec!["../../proto/penumbra/penumbra/cnidarium/v1alpha1/cnidarium.proto"]
    } else {
        vec![
            // List of non-cnidarium proto files
        ]
    };

    tonic_build::configure()
        .out_dir(target_dir)
        .emit_rerun_if_changed(false)
        .server_mod_attribute(".", r#"#[cfg(feature = "rpc")]"#)
        .client_mod_attribute(".", r#"#[cfg(feature = "rpc")]"#)
        .compile_with_config(
            config.clone(),
            &proto_files,
            &["../../proto/penumbra/", "../../proto/rust-vendored/"],
        )?;

    Ok(())
}

fn build_pbjson(target_dir: &PathBuf, descriptor_file_name: &str) -> Result<()> {
    let descriptor_set = std::fs::read(target_dir.join(descriptor_file_name))?;
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)?
        .out_dir(target_dir)
        .exclude([
            ".penumbra.util.tendermint_proxy.v1alpha1.ABCIQueryResponse",
            ".penumbra.util.tendermint_proxy.v1alpha1.GetBlockByHeightResponse",
            ".penumbra.util.tendermint_proxy.v1alpha1.GetStatusResponse",
        ].iter().map(|s| s.to_string()).collect::<Vec<String>>())
        .build(&[".penumbra"])?;

    Ok(())
}
