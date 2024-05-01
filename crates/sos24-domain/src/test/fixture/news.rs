use crate::entity::file_data::FileId;
use crate::entity::news::{News, NewsBody, NewsId, NewsTitle};
use crate::entity::project::{ProjectAttributes, ProjectCategories};

use super::datetime;

pub fn id1() -> NewsId {
    NewsId::new(uuid::Uuid::from_u128(1))
}

pub fn title1() -> NewsTitle {
    NewsTitle::new("タイトル1".to_string())
}

pub fn body1() -> NewsBody {
    NewsBody::new("本文1".to_string())
}

pub fn attachments1() -> Vec<FileId> {
    vec![]
}

pub fn categories1() -> ProjectCategories {
    ProjectCategories::GENERAL
}

pub fn attributes1() -> ProjectAttributes {
    ProjectAttributes::ACADEMIC
}

pub fn news1() -> News {
    News::new(
        id1(),
        title1(),
        body1(),
        attachments1(),
        categories1(),
        attributes1(),
        datetime::now(),
        datetime::now(),
    )
}

pub fn id2() -> NewsId {
    NewsId::new(uuid::Uuid::from_u128(2))
}

pub fn title2() -> NewsTitle {
    NewsTitle::new("タイトル2".to_string())
}

pub fn body2() -> NewsBody {
    NewsBody::new("本文2".to_string())
}

pub fn attachments2() -> Vec<FileId> {
    vec![]
}

pub fn categories2() -> ProjectCategories {
    ProjectCategories::STAGE_1A
}

pub fn attributes2() -> ProjectAttributes {
    ProjectAttributes::OFFICIAL
}

pub fn news2() -> News {
    News::new(
        id2(),
        title2(),
        body2(),
        attachments2(),
        categories2(),
        attributes2(),
        datetime::now(),
        datetime::now(),
    )
}
