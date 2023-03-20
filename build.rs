use std::env;
use std::path::{Path, PathBuf};

use coin_build_tools::{utils, link, coinbuilder};

const LIB_NAME: &str = "Cgl";

fn main() {
    println!("cargo:rerun-if-changed={}_lib_sources.txt", LIB_NAME.to_ascii_lowercase());
    println!("cargo:rerun-if-env-changed=CARGO_{}_STATIC", LIB_NAME.to_ascii_uppercase());
    println!("cargo:rerun-if-env-changed=CARGO_{}_SYSTEM", LIB_NAME.to_ascii_uppercase());

    link::link_lib_system_if_defined(LIB_NAME);

    if !Path::new(&format!("{}/AUTHORS", LIB_NAME)).exists() {
        utils::update_submodules(env::var("CARGO_MANIFEST_DIR").unwrap());
    }
    build_lib_and_link();
}

fn build_lib_and_link() {
    let mut config = coinbuilder::init_builder();

    let src_dir = format!(
        "{}",
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(LIB_NAME)
            .join(LIB_NAME)
            .join("src")
            .display()
    );

    let mut includes_dir = vec![
        format!("{}", src_dir),
    ];

    let mut lib_sources = include_str!("cgl_lib_sources.txt")
        .trim()
        .split('\n')
        .map(|file| format!("{}/{}", src_dir, file.trim()))
        .collect::<Vec<String>>();

    let mut coinflags = vec!["CGL".to_string()];

    if cfg!(feature = "CglAllDifferent") {
        lib_sources.push(format!("{}/CglAllDifferent/CglAllDifferent.cpp", src_dir));
        includes_dir.push(format!("{}/CglAllDifferent/", src_dir));
    }
    if cfg!(feature = "CglBKClique") {
        todo!("CglBKClique");
    }
    if cfg!(feature = "CglClique") {
        lib_sources.push(format!("{}/CglClique/CglClique.cpp", src_dir));
        includes_dir.push(format!("{}/CglClique/", src_dir));
    }
    if cfg!(feature = "CglCliqueStrengthening") {
        todo!("CglCliqueStrengthening");
    }
    if cfg!(feature = "CglDuplicateRow") {
        lib_sources.push(format!("{}/CglDuplicateRow/CglDuplicateRow.cpp", src_dir));
        includes_dir.push(format!("{}/CglDuplicateRow/", src_dir));
    }
    if cfg!(feature = "CglFlowCover") {
        lib_sources.push(format!("{}/CglFlowCover/CglFlowCover.cpp", src_dir));
        includes_dir.push(format!("{}/CglFlowCover/", src_dir));
    }
    if cfg!(feature = "CglGMI") {
        lib_sources.push(format!("{}/CglGMI/CglGMI.cpp", src_dir));
        lib_sources.push(format!("{}/CglGMI/CglGMIParam.cpp", src_dir));
        includes_dir.push(format!("{}/CglGMI/", src_dir));
    }
    if cfg!(feature = "CglGomory") {
        lib_sources.push(format!("{}/CglGomory/CglGomory.cpp", src_dir));
        includes_dir.push(format!("{}/CglGomory/", src_dir));
    }
    if cfg!(feature = "CglKnapsackCover") {
        lib_sources.push(format!("{}/CglKnapsackCover/CglKnapsackCover.cpp", src_dir));
        includes_dir.push(format!("{}/CglKnapsackCover/", src_dir));
    }
    if cfg!(feature = "CglLandP") {
        lib_sources.push(format!("{}/CglLandP/CglLandP.cpp", src_dir));
        lib_sources.push(format!("{}/CglLandP/CglLandPMessages.cpp", src_dir));
        lib_sources.push(format!("{}/CglLandP/CglLandPSimplex.cpp", src_dir));
        lib_sources.push(format!("{}/CglLandP/CglLandPTabRow.cpp", src_dir));
        lib_sources.push(format!("{}/CglLandP/CglLandPUtils.cpp", src_dir));
        lib_sources.push(format!("{}/CglLandP/CglLandPValidator.cpp", src_dir));
        includes_dir.push(format!("{}/CglLandP/", src_dir));
    }
    if cfg!(feature = "CglLiftAndProject") {
        lib_sources.push(format!("{}/CglLiftAndProject/CglLiftAndProject.cpp", src_dir));
        includes_dir.push(format!("{}/CglLiftAndProject/", src_dir));
    }
    if cfg!(feature = "CglMixedIntegerRounding") {
        lib_sources.push(format!("{}/CglMixedIntegerRounding/CglMixedIntegerRounding.cpp", src_dir));
        includes_dir.push(format!("{}/CglMixedIntegerRounding/", src_dir));
    }
    if cfg!(feature = "CglMixedIntegerRounding2") {
        lib_sources.push(format!("{}/CglMixedIntegerRounding2/CglMixedIntegerRounding2.cpp", src_dir));
        includes_dir.push(format!("{}/CglMixedIntegerRounding2/", src_dir));
    }
    if cfg!(feature = "OddHole") {
        lib_sources.push(format!("{}/CglOddHole/CglOddHole.cpp", src_dir));
        includes_dir.push(format!("{}/CglOddHole/", src_dir));
    }
    if cfg!(feature = "OddWheel") {
        todo!("OddWheel");
    }
    if cfg!(feature = "CglPreProcess") {
        lib_sources.push(format!("{}/CglPreProcess/CglPreProcess.cpp", src_dir));
        includes_dir.push(format!("{}/CglPreProcess/", src_dir));
    }
    if cfg!(feature = "CglProbing") {
        lib_sources.push(format!("{}/CglProbing/CglProbing.cpp", src_dir));
        includes_dir.push(format!("{}/CglProbing/", src_dir));
    }
    if cfg!(feature = "CglRedSplit") {
        lib_sources.push(format!("{}/CglRedSplit/CglRedSplit.cpp", src_dir));
        lib_sources.push(format!("{}/CglRedSplit/CglRedSplitParam.cpp", src_dir));
        includes_dir.push(format!("{}/CglRedSplit/", src_dir));
    }
    if cfg!(feature = "CglRedSplit2") {
        lib_sources.push(format!("{}/CglRedSplit2/CglRedSplit2.cpp", src_dir));
        lib_sources.push(format!("{}/CglRedSplit2/CglRedSplit2Param.cpp", src_dir));
        includes_dir.push(format!("{}/CglRedSplit2/", src_dir));
    }
    if cfg!(feature = "CglResidualCapacity") {
        lib_sources.push(format!("{}/CglResidualCapacity/CglResidualCapacity.cpp", src_dir));
        includes_dir.push(format!("{}/CglResidualCapacity/", src_dir));
    }
    if cfg!(feature = "CglSimpleRounding") {
        lib_sources.push(format!("{}/CglSimpleRounding/CglSimpleRounding.cpp", src_dir));
        includes_dir.push(format!("{}/CglSimpleRounding/", src_dir));
    }
    if cfg!(feature = "CglTwomir") {
        lib_sources.push(format!("{}/CglTwomir/CglTwomir.cpp", src_dir));
        includes_dir.push(format!("{}/CglTwomir/", src_dir));
    }
    if cfg!(feature = "CglZeroHalf") {
        lib_sources.push(format!("{}/CglZeroHalf/CglZeroHalf.cpp", src_dir));
        lib_sources.push(format!("{}/CglZeroHalf/Cgl012cut.cpp", src_dir));
        includes_dir.push(format!("{}/CglZeroHalf/", src_dir));
    }

    coinbuilder::print_metedata(includes_dir.clone(), coinflags.clone());

    let (include_other, coinflags_other) = coinbuilder::get_metedata_from("CoinUtils");
    includes_dir.extend(include_other);
    coinflags.extend(coinflags_other);

    let (include_other, coinflags_other) = coinbuilder::get_metedata_from("Osi");
    includes_dir.extend(include_other);
    coinflags.extend(coinflags_other);

    if cfg!(feature = "with_clp") {
        let (include_other, coinflags_other) = coinbuilder::get_metedata_from("Clp");
        includes_dir.extend(include_other);
        coinflags.extend(coinflags_other);
    }

    coinflags.iter().for_each(|flag| {
        config.define(&format!("COIN_HAS_{}", flag), None);
    });
    config.includes(includes_dir);
    config.files(lib_sources);

    config.compile("Cgl");
}
