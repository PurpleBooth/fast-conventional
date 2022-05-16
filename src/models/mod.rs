pub mod fast_conventional_config;

mod conventional_commit;

pub use conventional_commit::body::Body as ConventionalBody;
pub use conventional_commit::change::Change as ConventionalChange;
pub use conventional_commit::commit::Commit as ConventionalCommit;
pub use conventional_commit::scope::Scope as ConventionalScope;
pub use conventional_commit::subject::Subject as ConventionalSubject;
pub use git::revision_selection::RevisionSelection as GitRevisionSelection;
pub use git::short_ref::ShortRef as GitShortRef;

mod git;
