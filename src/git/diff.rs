use git2::Repository;

pub fn get_last_commit_diff(repo: &Repository) -> Result<String, git2::Error> {
    let obj = repo.head()?.resolve()?.peel_to_commit()?;
    let tree = obj.tree()?;
    let parent = obj.parent(0)?.tree()?;
    let diff = repo.diff_tree_to_tree(Some(&parent), Some(&tree), None)?;
    let mut result = String::new();
    diff.print(git2::DiffFormat::Patch, |_, _, l| {
        result.push_str(std::str::from_utf8(l.content()).unwrap());
        true
    })?;
    Ok(result)
}
