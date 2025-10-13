use gratex::cli::convert::{convert_input, InputType};
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> anyhow::Result<()> {
    let fixtures_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut fixtures = collect_tex_fixtures(&fixtures_dir)?;

    fixtures.sort();

    for tex_path in fixtures {
        let out_path = tex_path.with_extension("out");
        println!("Updating {}", out_path.display());

        let output = convert_input(InputType::File {
            path: tex_path.clone(),
        })
        .map_err(|err| anyhow::anyhow!("failed to convert {}: {err}", tex_path.display()))?;

        fs::write(&out_path, output)?;
    }

    Ok(())
}

fn collect_tex_fixtures(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut fixtures = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("tex") {
            fixtures.push(path);
        }
    }
    Ok(fixtures)
}
