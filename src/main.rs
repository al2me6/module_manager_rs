use std::ffi::OsStr;
use std::path::PathBuf;
use std::rc::Rc;

use clap::Parser;
use module_manager_rs::file::File;
use module_manager_rs::module_manager::ModuleManager;
use module_manager_rs::raw_patch::RawPatches;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command()]
struct Arguments {
    game_data: PathBuf,
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let args = Arguments::parse();
    let full_path = args.game_data.canonicalize()?;
    log::info!("GameData path: {full_path:?}");

    let file_storage = {
        let mut file_storage = vec![];

        for entry in WalkDir::new(full_path).sort_by_file_name() {
            let cfg = entry?.into_path();
            // TODO: ignore PluginData.
            if !cfg.is_file() || cfg.extension() != Some(OsStr::new("cfg")) {
                continue;
            }
            file_storage.push(File::new(Rc::from(&*cfg), std::fs::read_to_string(cfg)?));
        }
        file_storage
    };
    let raw_patches = {
        let mut raw_patches = RawPatches::default();
        for cfg in &file_storage {
            log::info!("parsing {:?}", cfg.path);
            raw_patches.files.push(File::new(
                Rc::clone(&cfg.path),
                ksp_cfg_formatter::parse_to_ast(&cfg.contents)?,
            ))
        }
        raw_patches
    };

    let patcher = ModuleManager::new(raw_patches, std::iter::empty())?;
    let database = patcher.execute()?;

    println!("{database}");

    Ok(())
}
