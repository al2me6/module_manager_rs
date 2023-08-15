use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;

use anyhow::Context;
use itertools::Itertools;
use ksp_cfg_formatter::parser::{Document, Node, NodeItem};
use module_manager_rs::database::Database;
use module_manager_rs::file::File;
use module_manager_rs::module_manager::{patcher, ModuleManager};
use module_manager_rs::node_patch::NodePatch;
use module_manager_rs::raw_patch::RawPatches;
use walkdir::WalkDir;

const SNIPPETS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/snippets");

fn find_node_by_name<'a>(document: &mut Document<'a>, name: &str) -> Option<Node<'a>> {
    let idx = document
        .statements
        .iter_mut()
        .position(|statement| matches!(statement, NodeItem::Node(node) if node.identifier == name));
    idx.map(|idx| match document.statements.remove(idx) {
        NodeItem::Node(node) => node,
        _ => unreachable!(),
    })
}

fn run_snippet(path: &Path) -> anyhow::Result<()> {
    let file = std::fs::read_to_string(path)?;
    let mut cfg = ksp_cfg_formatter::parse_to_ast(&file)?;

    let dll_names = find_node_by_name(&mut cfg, "DLLS")
        .iter()
        .flat_map(|node| node.block.iter())
        .filter_map(|item| match item {
            NodeItem::KeyVal(key) if key.key == "dll" => Some(key.val),
            _ => None,
        })
        .map(ToOwned::to_owned)
        .collect_vec();

    let patch = RawPatches {
        files: vec![File {
            path: Rc::from(path),
            contents: Document {
                statements: find_node_by_name(&mut cfg, "PATCH")
                    .context("snippet does not specify a `PATCH`")?
                    .block,
            },
        }],
    };

    let expect = Database(
        find_node_by_name(&mut cfg, "EXPECT")
            .context("snippet does not specify an `EXPECT`")?
            .block
            .into_iter()
            .filter_map(|item| match item {
                NodeItem::Node(node) => Some(node),
                _ => None,
            })
            .map(|node| -> module_manager_rs::Result<_> {
                let mut data = patcher::evaluate_node_as_pure_data(
                    Rc::from(path),
                    &NodePatch::from_cst(node, true)?,
                )?;
                data.value.file_path = Some(Rc::from(path));
                Ok(Some(data))
            })
            .collect::<Result<Vec<_>, _>>()?,
    );

    let mm = ModuleManager::new(patch, dll_names.iter().map(AsRef::as_ref))
        .context("patch extraction failed")
        .unwrap();

    let evaluated = mm.execute().context("patch execution failed").unwrap();

    assert!(
        expect == evaluated,
        "expected output:\n{expect}\ngot output:\n{evaluated}"
    );

    Ok(())
}

fn main() -> anyhow::Result<()> {
    const OK: &str = "\x1b[32mok\x1b[0m";
    const FAILED: &str = "\x1b[1;31mFAILED\x1b[0m";

    println!();
    let mut passed = 0;
    let mut failed = 0;
    for snippet in WalkDir::new(SNIPPETS_PATH) {
        let snippet = snippet?.into_path();
        if !snippet.is_file() || snippet.extension() != Some(OsStr::new("cfg")) {
            continue;
        }
        println!("running snippet {}...", snippet.to_string_lossy());

        let result = std::panic::catch_unwind(|| run_snippet(&snippet));
        match result {
            Ok(inner) => {
                inner?;
                passed += 1;
            }
            Err(_) => {
                failed += 1;
            }
        }
    }
    let result = if failed == 0 { OK } else { FAILED };
    println!("\ntest result: {result}. {passed} passed; {failed} failed\n");
    Ok(())
}
