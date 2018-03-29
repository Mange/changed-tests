use options::Options;
use git2::{DiffDelta, DiffHunk, DiffLine, DiffOptions, Repository};
use failure::Error;

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct Diff {
    lines: HashMap<PathBuf, HashSet<u32>>,
}

pub fn calculate_changed_lines(options: &Options, repo: &Repository) -> Result<Diff, Error> {
    // Find files that have changed so far
    let diff_point = match &options.branch_name {
        &Some(ref string) => string,
        &None => "HEAD",
    };
    let diff_base = repo.revparse_single(diff_point)?;
    let commit = match diff_base.as_commit() {
        Some(commit) => commit,
        None => bail!(
            "Provided branch name is not to a commit. {} is a {}",
            diff_base.id(),
            diff_base.kind().unwrap()
        ),
    };
    let mut diff_options = DiffOptions::default();
    diff_options
        .include_untracked(true)
        .ignore_whitespace(true)
        .ignore_whitespace_eol(true)
        .minimal(true)
        .indent_heuristic(true)
        .context_lines(0);
    let git_diff = repo.diff_tree_to_workdir(Some(&commit.tree()?), Some(&mut diff_options))?;

    let mut diff = Diff::default();

    git_diff.foreach(
        &mut new_file,
        None, /* binary_cb */
        None, /* hunk_cb */
        Some(
            &mut |diff_delta: DiffDelta, _: Option<DiffHunk>, line_diff: DiffLine| {
                diff.record_line_diff(diff_delta, line_diff)
            },
        ),
    )?;

    Ok(diff)
}

impl Diff {
    pub fn file_names(&self) -> BTreeSet<&Path> {
        self.lines.keys().map(PathBuf::as_ref).collect()
    }

    pub fn lines_of_file(&self, path: &Path) -> BTreeSet<u32> {
        self.lines
            .get(path)
            .cloned()
            .unwrap_or_else(HashSet::new)
            .into_iter()
            .collect()
    }

    fn record_line_diff(&mut self, diff_delta: DiffDelta, line_diff: DiffLine) -> bool {
        // Early return if this isn't an added line
        // TODO: Figure out how to map deleted lines too. Tests with deletions in them still need
        // to run, but if a whole example is removed then nothing needs to run.
        let added_line = match line_diff.new_lineno() {
            Some(line) => line,
            None => return true,
        };

        if let Some(path) = diff_delta.new_file().path() {
            let path = path.to_owned();
            let lines = self.lines.entry(path).or_insert_with(|| HashSet::new());
            lines.insert(added_line);
        }
        true
    }
}

fn new_file(_: DiffDelta, _: f32) -> bool {
    // Ignore this callback
    true
}
