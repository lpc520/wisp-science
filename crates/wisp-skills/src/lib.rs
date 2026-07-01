//! SKILL.md discovery + the `use_skill` tool.

pub mod index;
pub mod tool;

pub use index::{bundled_dir, list_resources, Skill, SkillIndex};
pub use tool::UseSkillTool;
