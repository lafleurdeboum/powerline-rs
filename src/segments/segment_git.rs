use format::*;
use git2::{self, BranchType, ObjectType, Repository, StatusOptions, StatusShow};
use segment::Segment;
use std::collections::VecDeque;

pub fn segment_git(segments: &mut VecDeque<Segment>, git: &Option<Repository>) {
    if git.is_none() {
        return;
    }
    let git = git.as_ref().unwrap();

    let branches = git.branches(Some(BranchType::Local));
    if branches.is_err() {
        return;
    }

    let mut current = None;

    for branch in branches.unwrap() {
        if let Ok((branch, _)) = branch {
            if branch.is_head() {
                if let Ok(name) = branch.name() {
                    if let Some(name) = name {
                        current = Some(name.to_string());
                        break;
                    }
                }
            }
        }
    }

    if current.is_none() {
        // Could be a detached head
        if let Ok(head) = git.head() {
            if let Some(target) = head.target() {
                current = git.find_object(target, Some(ObjectType::Any))
                            .ok()
                            .and_then(|obj| obj.short_id().ok())
                            .and_then(|buf| buf.as_str()
                                                .map(|s| s.to_string()))
            }
        } else {
            segments.push_back(Segment::new(REPO_DIRTY_BG, REPO_DIRTY_FG, "Big Bang"));
            return;
        }
    }

    let statuses = git.statuses(Some(
        StatusOptions::new()
            .show(StatusShow::IndexAndWorkdir)
            .include_untracked(true)
    ));
    if statuses.is_err() {
        return;
    }

    let (mut bg, mut fg) = (REPO_DIRTY_BG, REPO_DIRTY_FG);
    if statuses.unwrap().len() == 0 {
        bg = REPO_CLEAN_BG;
        fg = REPO_CLEAN_FG;
    }
    segments.push_back(Segment::new(bg, fg, current.unwrap()));
}

pub fn segment_gitstage(segments: &mut VecDeque<Segment>, git: &Option<Repository>) {
    if git.is_none() {
        return;
    }
    let git = git.as_ref().unwrap();

    let statuses = git.statuses(Some(
        StatusOptions::new()
            .show(StatusShow::IndexAndWorkdir)
            .include_untracked(true)
            .renames_from_rewrites(true)
            .renames_head_to_index(true)
    ));
    if statuses.is_err() {
        return;
    }

    let mut staged = 0;
    let mut notstaged = 0;
    let mut untracked = 0;
    let mut conflicted = 0;

    for status in statuses.unwrap().iter() {
        let status = status.status();
        if status.contains(git2::STATUS_INDEX_NEW)
            || status.contains(git2::STATUS_INDEX_MODIFIED)
            || status.contains(git2::STATUS_INDEX_TYPECHANGE)
            || status.contains(git2::STATUS_INDEX_RENAMED)
            || status.contains(git2::STATUS_INDEX_DELETED) {
            staged += 1;
        }
        if status.contains(git2::STATUS_WT_MODIFIED)
            || status.contains(git2::STATUS_WT_TYPECHANGE)
            || status.contains(git2::STATUS_WT_DELETED) {
            notstaged += 1;
        }
        if status.contains(git2::STATUS_WT_NEW) {
            untracked += 1;
        }
        if status.contains(git2::STATUS_CONFLICTED) {
            conflicted += 1;
        }
    }

    if staged > 0 {
        let mut string = if staged == 1 { String::with_capacity(1) } else { staged.to_string() };
        string.push('✔');
        segments.push_back(Segment::new(GIT_STAGED_BG, GIT_STAGED_FG, string));
    }
    if notstaged > 0 {
        let mut string = if notstaged == 1 { String::with_capacity(1) } else { notstaged.to_string() };
        string.push('✎');
        segments.push_back(Segment::new(GIT_NOTSTAGED_BG, GIT_NOTSTAGED_FG, string));
    }
    if untracked > 0 {
        let mut string = if untracked == 1 { String::with_capacity(1) } else { untracked.to_string() };
        string.push('+');
        segments.push_back(Segment::new(GIT_UNTRACKED_BG, GIT_UNTRACKED_FG, string));
    }
    if conflicted > 0 {
        let mut string = if conflicted == 1 { String::with_capacity(1) } else { conflicted.to_string() };
        string.push('*');
        segments.push_back(Segment::new(GIT_CONFLICTED_BG, GIT_CONFLICTED_FG, string));
    }
}