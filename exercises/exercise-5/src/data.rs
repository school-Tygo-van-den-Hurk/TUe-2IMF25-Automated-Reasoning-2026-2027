/// The contents of the `data/buildroot.dimacs` file.
pub const BUILD_ROOT: &str = include_str!("../data/buildroot.dimacs");

/// The contents of the `data/busybox.dimacs` file.
pub const BUSYBOX: &str = include_str!("../data/busybox.dimacs");

/// The contents of the `data/embtoolkit.dimacs` file.
pub const EMBTOOLKIT: &str = include_str!("../data/embtoolkit.dimacs");

/// The contents of the `data/toybox.dimacs` file.
pub const TOYBOX: &str = include_str!("../data/toybox.dimacs");

/// The contents of the `data/uClinux.dimacs` file.
pub const U_CLINUX: &str = include_str!("../data/uClinux.dimacs");

/// Gets the dimacs dataset by name, or if no such dataset exists, read fro
/// disk instead.
pub fn get_dimacs(dataset: &str) -> String {
    match &dataset.to_lowercase() as &str {

        "buildroot" | "build_root" | "build-root" => {
            println!("using builtin dataset: 'buildroot'");
            BUILD_ROOT.to_owned()
        },
        
        "busybox"  => {
            println!("using builtin dataset: 'busybox'");
            BUSYBOX.to_owned()
        },

        "embtoolkit" | "emb_toolkit" | "emb-toolkit" => {
            println!("using builtin dataset: 'embtoolkit'");
            EMBTOOLKIT.to_owned()
        },

        "toybox" => {
            println!("using builtin dataset: 'toybox'");
            TOYBOX.to_owned()
        },

        "uclinux" | "u_clinux" | "u-clinux" => {
            println!("using builtin dataset: 'uClinux'");
            U_CLINUX.to_owned()
        },
        
        _otherwise => {
            print!("No such dataset known internally: '{dataset}',");
            println!("attempting to read from disk instead.");
            std::fs::read_to_string(&dataset).expect("could not open file")
        }
    }
}
