//! The built-in template.

use std::path::Path;
use tera::Tera;

pub fn builtin() -> Result<Tera, tera::Error> {
    #[cfg(debug_assertions)] {
        Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/template/*.html"))
    }

    #[cfg(not(debug_assertions))] {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![
            ("macros.html", include_str!("macros.html")),
            ("base.html", include_str!("base.html")),
            ("dm_index.html", include_str!("dm_index.html")),
            ("dm_type.html", include_str!("dm_type.html")),
            ("dm_module.html", include_str!("dm_module.html")),
        ])?;
        Ok(tera)
    }
}

pub fn save_resources(output_path: &Path) -> std::io::Result<()> {
    let mut progress = crate::Progress::default();

    #[cfg(debug_assertions)]
    macro_rules! resources {
        ($($name:expr,)*) => {
            let env = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/template"));
            $(
                progress.update($name);
                std::fs::copy(&env.join($name), &output_path.join($name))?;
            )*
        }
    }

    #[cfg(not(debug_assertions))]
    macro_rules! resources {
        ($($name:expr,)*) => {{
            use std::io::Write;
            $(
                progress.update($name);
                crate::create(&output_path.join($name))?.write_all(include_bytes!($name))?;
            )*
        }}
    }

    resources! {
        "dmdoc.css",
        "dmdoc.js",
        "git.png",
    }

    Ok(())
}
