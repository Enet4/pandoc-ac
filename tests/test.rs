use pandoc_ast::{self, Pandoc};
use pandoc_ac::make_acronym_formatting;
use pandoc::{self, OutputKind};
use serde_json::from_reader;
use std::fs::File;
use tempfile::tempdir;
use pretty_assertions::assert_eq;

#[test]
pub fn test1() {
    let input_file = "resources/test.md";
    let gt_json_file = "resources/test.gt.md.json";

    let gt_json: Pandoc = from_reader(File::open(gt_json_file).unwrap()).unwrap();

    let input_json: Pandoc = {
        // use pandoc to convert Markdown to pandoc's JSON
        let mut pandoc = pandoc::new();
        pandoc.add_input(input_file);

        // intermediary data, keep it in a temporary directory
        let dir = tempdir().unwrap();
        let mdfilepath = dir.path().join("test.md.json");

        pandoc.set_output(OutputKind::File(mdfilepath.display().to_string()));
        pandoc.execute().unwrap();
        from_reader(File::open(mdfilepath).unwrap()).unwrap()
   };

    let result = make_acronym_formatting(input_json);
    
    assert_eq!(&result.meta, &gt_json.meta);
    assert_eq!(&result.blocks, &gt_json.blocks);
    assert_eq!(result.pandoc_api_version[0], result.pandoc_api_version[0]);
}