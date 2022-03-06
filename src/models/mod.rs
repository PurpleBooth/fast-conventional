pub mod fast_conventional_config;

mod conventional_commit;

pub use conventional_commit::change::Change as ConventionalChange;
pub use conventional_commit::commit::Commit as ConventionalCommit;
pub use conventional_commit::scope::Scope as ConventionalScope;
