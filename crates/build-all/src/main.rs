use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() {
    let workspace_root = env::current_dir().expect("Failed to get current directory");
    println!("🔨 Building all FFI bindings...\n");

    let mut built_any = false;

    // Python (requires interpreter)
    if let Some(python) = find_python_interpreter() {
        println!("📦 Python (PyO3)");
        build_crate("prefade-py", Some(("PYO3_PYTHON", python.as_path())), &workspace_root, "Python");
        let py_source = artifact(&workspace_root, "_prefade_native", "dll", "dylib", "so");
        let py_dest = if cfg!(windows) {
            workspace_root.join("python/prefade/_prefade_native.pyd")
        } else {
            workspace_root.join("python/prefade/_prefade_native.so")
        };
        copy_file(&py_source, &py_dest, "Python");
        built_any = true;
    } else {
        println!("⚠️  Skipping Python: interpreter not found");
    }

    // Node (skip if Node is absent)
    if command_available(if cfg!(windows) { "node.exe" } else { "node" }) {
        println!("\n📦 Node.js (napi-rs)");
        build_crate("prefade-node", None, &workspace_root, "Node.js");
        let node_source = artifact(&workspace_root, "prefade_node", "dll", "dylib", "so");
        let node_dest = workspace_root.join("node/prefade_node.node");
        copy_file(&node_source, &node_dest, "Node.js");
        built_any = true;
    } else {
        println!("⚠️  Skipping Node.js: interpreter not found");
    }

    // Lua (skip if Lua is absent)
    if command_available(if cfg!(windows) { "lua.exe" } else { "lua" }) {
        println!("\n📦 Lua (mlua)");
        build_crate("prefade-lua", None, &workspace_root, "Lua");
        let lua_source = artifact(&workspace_root, "prefade_lua", "dll", "dylib", "so");
        let lua_dest = if cfg!(windows) {
            workspace_root.join("lua/prefade/prefade_native.dll")
        } else {
            workspace_root.join("lua/prefade/prefade_native.so")
        };
        copy_file(&lua_source, &lua_dest, "Lua");
        built_any = true;
    } else {
        println!("⚠️  Skipping Lua: interpreter not found");
    }

    if built_any {
        println!("\n✅ Bindings built and distributed.");
        println!("Next steps:");
        println!("  • Node.js: cd node && npm run build && node dist/main.js");
        println!("  • Lua:    cd lua && lua prefade/main.lua");
        println!("  • Python: cd python && uv run python use-case1/main.py");
    } else {
        println!("⚠️  Nothing built: missing all interpreters (Python/Node/Lua)");
    }
}

fn build_crate(crate_name: &str, extra_env: Option<(&str, &Path)>, root: &Path, label: &str) {
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--release", "-p", crate_name]).current_dir(root);
    if let Some((key, val)) = extra_env {
        cmd.env(key, val);
    }
    let status = cmd.status().unwrap_or_else(|e| panic!("Failed to build {}: {}", label, e));
    if !status.success() {
        panic!("{} build failed", label);
    }
}

fn copy_file(source: &PathBuf, dest: &PathBuf, lang: &str) {
    if let Err(e) = std::fs::copy(source, dest) {
        panic!("Failed to copy {} binary: {}\n  Source: {}\n  Dest: {}", lang, e, source.display(), dest.display());
    }
    println!("  ✓ {} → {}", source.file_name().unwrap().to_string_lossy(), dest.display());
}

fn artifact(root: &Path, stem: &str, win_ext: &str, mac_ext: &str, unix_ext: &str) -> PathBuf {
    if cfg!(windows) {
        root.join(format!("target/release/{}.{}", stem, win_ext))
    } else if cfg!(target_os = "macos") {
        root.join(format!("target/release/lib{}.{}", stem, mac_ext))
    } else {
        root.join(format!("target/release/lib{}.{}", stem, unix_ext))
    }
}

fn command_available(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
    .stdout(Stdio::null())
    .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn find_python_interpreter() -> Option<PathBuf> {
    if let Ok(python) = env::var("PYO3_PYTHON") {
        let path = PathBuf::from(python);
        if path.exists() {
            return Some(path);
        }
    }

    let workspace_root = env::current_dir().ok()?;
    let venv_python = if cfg!(windows) {
        workspace_root.join("python/.venv/Scripts/python.exe")
    } else {
        workspace_root.join("python/.venv/bin/python")
    };
    if venv_python.exists() {
        return Some(venv_python);
    }

    let python_commands = if cfg!(windows) {
        vec!["python", "python3", "py"]
    } else {
        vec!["python3", "python"]
    };

    for cmd in python_commands {
        if let Ok(output) = Command::new(cmd)
            .arg("-c")
            .arg("import sys; print(sys.executable)")
            .output()
        {
            if output.status.success() {
                if let Ok(path_str) = String::from_utf8(output.stdout) {
                    let path = PathBuf::from(path_str.trim());
                    if path.exists() {
                        return Some(path);
                    }
                }
            }
        }
    }

    None
}
