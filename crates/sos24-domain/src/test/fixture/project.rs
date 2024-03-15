use crate::entity::{
    project::{
        Project, ProjectAttributes, ProjectCategory, ProjectGroupName, ProjectId, ProjectIndex,
        ProjectKanaGroupName, ProjectKanaTitle, ProjectTitle,
    },
    user::UserId,
};

pub fn id() -> ProjectId {
    ProjectId::new(uuid::Uuid::from_u128(1))
}

pub fn index() -> ProjectIndex {
    ProjectIndex::new(0)
}

pub fn title() -> ProjectTitle {
    ProjectTitle::new("そぽたん焼き".to_string())
}

pub fn kana_title() -> ProjectKanaTitle {
    ProjectKanaTitle::new("そぽたんやき".to_string())
}

pub fn group_name() -> ProjectGroupName {
    ProjectGroupName::new("そぽたん愛好会".to_string())
}

pub fn kana_group_name() -> ProjectKanaGroupName {
    ProjectKanaGroupName::new("そぽたんあいこうかい".to_string())
}

pub fn project(
    category: ProjectCategory,
    attributes: ProjectAttributes,
    owner_id: UserId,
) -> Project {
    Project::new(
        id(),
        index(),
        title(),
        kana_title(),
        group_name(),
        kana_group_name(),
        category,
        attributes,
        owner_id,
        None,
        None,
    )
}
