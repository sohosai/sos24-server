use crate::entity::{
    project::{
        Project, ProjectAttributes, ProjectCategory, ProjectGroupName, ProjectId, ProjectIndex,
        ProjectKanaGroupName, ProjectKanaTitle, ProjectTitle,
    },
    user::UserId,
};

pub fn id1() -> ProjectId {
    ProjectId::new(uuid::Uuid::from_u128(1))
}

pub fn index1() -> ProjectIndex {
    ProjectIndex::new(0)
}

pub fn title1() -> ProjectTitle {
    ProjectTitle::new("そぽたん焼き".to_string())
}

pub fn kana_title1() -> ProjectKanaTitle {
    ProjectKanaTitle::new("そぽたんやき".to_string())
}

pub fn group_name1() -> ProjectGroupName {
    ProjectGroupName::new("そぽたん愛好会".to_string())
}

pub fn kana_group_name1() -> ProjectKanaGroupName {
    ProjectKanaGroupName::new("そぽたんあいこうかい".to_string())
}

pub fn project1(
    category: ProjectCategory,
    attributes: ProjectAttributes,
    owner_id: UserId,
) -> Project {
    Project::new(
        id1(),
        index1(),
        title1(),
        kana_title1(),
        group_name1(),
        kana_group_name1(),
        category,
        attributes,
        owner_id,
        None,
        None,
    )
}

pub fn id2() -> ProjectId {
    ProjectId::new(uuid::Uuid::from_u128(2))
}

pub fn index2() -> ProjectIndex {
    ProjectIndex::new(0)
}

pub fn title2() -> ProjectTitle {
    ProjectTitle::new("そぽたん煮".to_string())
}

pub fn kana_title2() -> ProjectKanaTitle {
    ProjectKanaTitle::new("そぽたんに".to_string())
}

pub fn group_name2() -> ProjectGroupName {
    ProjectGroupName::new("そぽたん連盟".to_string())
}

pub fn kana_group_name2() -> ProjectKanaGroupName {
    ProjectKanaGroupName::new("そぽたんれんめい".to_string())
}

pub fn project2(
    category: ProjectCategory,
    attributes: ProjectAttributes,
    owner_id: UserId,
) -> Project {
    Project::new(
        id2(),
        index2(),
        title2(),
        kana_title2(),
        group_name2(),
        kana_group_name2(),
        category,
        attributes,
        owner_id,
        None,
        None,
    )
}