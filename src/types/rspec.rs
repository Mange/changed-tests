use options::Options;
use git2::Repository;
use failure::Error;

use diff;

pub fn run(options: Options, repo: &Repository) -> Result<(), Error> {
    let diff = diff::calculate_changed_lines(&options, repo)?;
    println!("These files have diffs:");
    for name in diff.file_names() {
        println!("{} - {:?}", name.display(), diff.lines_of_file(name));
    }
    Ok(())
}
