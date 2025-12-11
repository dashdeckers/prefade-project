use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let root = std::env::current_dir().expect("Failed to get current directory");

    // Build Python bindings
    if let Some(python) = find_python() {
        println!("Building Python bindings...");
        cargo_build("prefade-py", &root, Some(("PYO3_PYTHON", &python)));
        copy_artifact(&root, "_prefade_native", "python/prefade/_prefade_native");
    }

    // Build Node.js bindings
    println!("Building Node.js bindings...");
    cargo_build("prefade-node", &root, None);
    copy_artifact(&root, "prefade_node", "node/prefade_node");

    println!("Done.");
}

fn cargo_build(crate_name: &str, root: &Path, env: Option<(&str, &Path)>) {
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--release", "-p", crate_name]).current_dir(root);
    if let Some((k, v)) = env {
        cmd.env(k, v);
    }
    assert!(cmd.status().expect("cargo failed").success(), "Build failed: {crate_name}");
}

fn copy_artifact(root: &Path, name: &str, dest_base: &str) {
    let (src_name, dest_ext) = if cfg!(windows) {
        (format!("{name}.dll"), "pyd")
    } else if cfg!(target_os = "macos") {
        (format!("lib{name}.dylib"), "so")
    } else {
        (format!("lib{name}.so"), "so")
    };

    let src = root.join("target/release").join(&src_name);
    let dest = if dest_base.contains("node") {
        root.join(format!("{dest_base}.node"))
    } else {
        root.join(format!("{dest_base}.{dest_ext}"))
    };

    std::fs::copy(&src, &dest)
        .unwrap_or_else(|e| panic!("Copy failed: {} -> {}: {e}", src.display(), dest.display()));
}

fn find_python() -> Option<PathBuf> {
    // Check PYO3_PYTHON env var
    if let Ok(p) = std::env::var("PYO3_PYTHON") {
        let path = PathBuf::from(p);
        if path.exists() {
            return Some(path);
        }
    }

    // Check local venv
    let root = std::env::current_dir().ok()?;
    let venv = if cfg!(windows) {
        root.join("python/.venv/Scripts/python.exe")
    } else {
        root.join("python/.venv/bin/python")
    };
    if venv.exists() {
        return Some(venv);
    }

    // Try system Python
    for cmd in ["python3", "python"] {
        if let Ok(out) = Command::new(cmd).args(["-c", "import sys; print(sys.executable)"]).output() {
            if out.status.success() {
                let path = PathBuf::from(String::from_utf8_lossy(&out.stdout).trim());
                if path.exists() {
                    return Some(path);
                }
            }
        }
    }

    None
}
