use crate::types::{BlockAndTopLevelPrimitive, ScratchProject, StageOrSprite};

pub fn project_parser(project: ScratchProject) {
    for sprite in project.targets {
        let blocks = match &sprite {
            StageOrSprite::Stage(v) => &v.blocks,
            StageOrSprite::Sprite(v) => &v.blocks,
        };
        for b in blocks {
            if let BlockAndTopLevelPrimitive::Block(block) = b.1 {}
        }
    }
}
