use crate::{err::Result, explore, options::ListOptions};

use std::io::{self, prelude::*};

pub fn run(list_opts: &ListOptions) -> Result<()> {
    let dirs = explore::find_git_folders(&list_opts.base.base_dir, list_opts.deep_recurse)?;
    let stdin = io::stdout();
    let mut lock = stdin.lock();

    for d in dirs {
        writeln!(lock, "{}", d.display())?;
    }

    Ok(())
}
