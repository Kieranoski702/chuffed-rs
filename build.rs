// adapted from
// https://github.com/gokberkkocak/rust_glucose/blob/master/build.rs
// and
// 1https://rust-lang.github.io/rust-bindgen/non-system-libraries.html

use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    vendor_configure();
    bind();
    build();
}

fn vendor_configure() {
    // Generate cpp") and h files in /vendor/build
    Command::new("./prebuild.sh").output();
}

fn bind() {
    let bindings = bindgen::Builder::default()
        .header("vendor/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("createVars")
        .clang_arg("-Ivendor/chuffed/")
        .clang_arg("-Ivendor/chuffed/branching")
        .clang_arg("-Ivendor/chuffed/core")
        .clang_arg("-Ivendor/chuffed/examples")
        .clang_arg("-Ivendor/chuffed/flatzinc")
        .clang_arg("-Ivendor/chuffed/globals")
        .clang_arg("-Ivendor/chuffed/ldsb")
        .clang_arg("-Ivendor/chuffed/mdd")
        .clang_arg("-Ivendor/chuffed/mip")
        .clang_arg("-Ivendor/chuffed/mzn-models")
        .clang_arg("-Ivendor/chuffed/primitives")
        .clang_arg("-Ivendor/chuffed/support")
        .clang_arg("-Ivendor/chuffed/vars")
        .clang_arg(r"--std=gnu++11")
        .clang_arg(r"-xc++")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build() {
    let mut binding = cc::Build::new();
    let mut builder = binding
        .include("vendor/chuffed")
        .include("vendor/chuffed/branhcing")
        .include("vendor/chuffed/core")
        .include("vendor/chuffed/examples")
        .include("vendor/chuffed/flatzinc")
        .include("vendor/chuffed/globals")
        .include("vendor/chuffed/ldsb")
        .include("vendor/chuffed/mdd")
        .include("vendor/chuffed/mip")
        .include("vendor/chuffed/mzn-models")
        .include("vendor/chuffed/primitives")
        .include("vendor/chuffed/support")
        .include("vendor/chuffed/vars")
        .cpp(true)
        // vendor/minion
        .file("vendor/chuffed/branching/branching.cpp")
        .file("vendor/chuffed/branching/impact.cpp")
        .file("vendor/chuffed/branching/branching.cpp")
        .file("vendor/chuffed/vars/int-var.cpp")
        .file("vendor/chuffed/vars/int-var-el.cpp")
        .file("vendor/chuffed/vars/modelling.cpp")
        .file("vendor/chuffed/vars/int-var-sl.cpp")
        .file("vendor/chuffed/vars/bool-view.cpp")
        .file("vendor/chuffed/vars/int-var-ll.cpp")
        .file("vendor/chuffed/ldsb/ldsb.cpp")
        .file("vendor/chuffed/globals/subcircuit.cpp")
        .file("vendor/chuffed/globals/mddglobals.cpp")
        .file("vendor/chuffed/globals/sym-break.cpp")
        .file("vendor/chuffed/globals/linear-bool.cpp")
        .file("vendor/chuffed/globals/linear-bool-decomp.cpp")
        .file("vendor/chuffed/globals/well-founded.cpp")
        .file("vendor/chuffed/globals/circuit.cpp")
        .file("vendor/chuffed/globals/minimum.cpp")
        .file("vendor/chuffed/globals/bool_arg_max.cpp")
        .file("vendor/chuffed/globals/alldiff.cpp")
        .file("vendor/chuffed/globals/template.cpp")
        .file("vendor/chuffed/globals/directives.cpp")
        .file("vendor/chuffed/globals/cumulative.cpp")
        .file("vendor/chuffed/globals/cumulativeCalendar.cpp")
        .file("vendor/chuffed/globals/disjunctive.cpp")
        .file("vendor/chuffed/globals/regular.cpp")
        .file("vendor/chuffed/globals/lex.cpp")
        .file("vendor/chuffed/globals/table.cpp")
        .file("vendor/chuffed/globals/edit_distance.cpp")
        .file("vendor/chuffed/globals/EdExplFinder.cpp")
        .file("vendor/chuffed/globals/value-precede.cpp")
        .file("vendor/chuffed/globals/graph.cpp")
        .file("vendor/chuffed/globals/tree.cpp")
        .file("vendor/chuffed/globals/minimum_weight_tree.cpp")
        .file("vendor/chuffed/globals/mst.cpp")
        .file("vendor/chuffed/globals/dconnected.cpp")
        .file("vendor/chuffed/globals/dtree.cpp")
        .file("vendor/chuffed/globals/dag.cpp")
        .file("vendor/chuffed/globals/bounded_path.cpp")
        .file("vendor/chuffed/mdd/MDD.cpp")
        .file("vendor/chuffed/mdd/mdd_prop.cpp")
        .file("vendor/chuffed/mdd/mdd_to_lgraph.cpp")
        .file("vendor/chuffed/mdd/opcache.cpp")
        .file("vendor/chuffed/mdd/weighted_dfa.cpp")
        .file("vendor/chuffed/mdd/wmdd_prop.cpp")
        .file("vendor/chuffed/mip/mip.cpp")
        .file("vendor/chuffed/mip/recalc.cpp")
        .file("vendor/chuffed/mip/simplex.cpp")
        .file("vendor/chuffed/primitives/element.cpp")
        .file("vendor/chuffed/primitives/bool.cpp")
        .file("vendor/chuffed/primitives/linear.cpp")
        .file("vendor/chuffed/primitives/arithmetic.cpp")
        .file("vendor/chuffed/primitives/domain.cpp")
        .file("vendor/chuffed/primitives/binary.cpp")
        .file("vendor/chuffed/branching/branching.cpp")
        .file("vendor/chuffed/core/init.cpp")
        .file("vendor/chuffed/core/stats.cpp")
        .file("vendor/chuffed/core/engine.cpp")
        .file("vendor/chuffed/core/options.cpp")
        .file("vendor/chuffed/core/sat.cpp")
        .file("vendor/chuffed/core/conflict.cpp")
        .file("vendor/chuffed/support/union_find.cpp")
        .file("vendor/chuffed/support/lengauer_tarjan.cpp")
        .file("vendor/chuffed/support/dijkstra.cpp")
        .file("vendor/chuffed/support/kosaraju_scc.cpp");
      
    builder
        .flag_if_supported("-D__STDC_LIMIT_MACROS")
        .flag_if_supported("-D__STDC_FORMAT_MACROS")
        .flag_if_supported("-DNDEBUG")
        .flag_if_supported("-fomit-frame-pointer")
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-xc++")
        .flag_if_supported("-w")
        .opt_level(3)
        .compile("fzn-vendor/chuffed");
}
