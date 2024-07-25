use std::ops::Range;

use crate::{
    data_source::kinds::{
        data_manipulation::map::MapParseTo,
        local_db::{
            parameters::update::batching::parameters::Batch30Days, DirectVecLocalDbChartSource,
        },
        remote_db::{PullAllWithAndSort, RemoteDatabaseSource, StatementFromRange},
    },
    utils::sql_with_range_filter_opt,
    ChartProperties, Named,
};

use chrono::NaiveDate;
use entity::sea_orm_active_enums::ChartType;
use sea_orm::{prelude::*, DbBackend, Statement};

pub struct NewVerifiedContractsStatement;

impl StatementFromRange for NewVerifiedContractsStatement {
    fn get_statement(range: Option<Range<DateTimeUtc>>) -> Statement {
        sql_with_range_filter_opt!(
            DbBackend::Postgres,
            r#"
                SELECT
                    DATE(smart_contracts.inserted_at) as date,
                    COUNT(*)::TEXT as value
                FROM smart_contracts
                WHERE TRUE {filter}
                GROUP BY DATE(smart_contracts.inserted_at)
            "#,
            [],
            "smart_contracts.inserted_at",
            range
        )
    }
}

pub type NewVerifiedContractsRemote =
    RemoteDatabaseSource<PullAllWithAndSort<NewVerifiedContractsStatement, NaiveDate, String>>;

pub struct Properties;

impl Named for Properties {
    fn name() -> String {
        "newVerifiedContracts".into()
    }
}

impl ChartProperties for Properties {
    type Resolution = NaiveDate;

    fn chart_type() -> ChartType {
        ChartType::Line
    }
}

pub type NewVerifiedContracts =
    DirectVecLocalDbChartSource<NewVerifiedContractsRemote, Batch30Days, Properties>;

pub type NewVerifiedContractsInt = MapParseTo<NewVerifiedContracts, i64>;

#[cfg(test)]
mod tests {
    use super::NewVerifiedContracts;
    use crate::tests::simple_test::simple_test_chart;

    #[tokio::test]
    #[ignore = "needs database to run"]
    async fn update_new_verified_contracts() {
        simple_test_chart::<NewVerifiedContracts>(
            "update_new_verified_contracts",
            vec![
                ("2022-11-14", "1"),
                ("2022-11-15", "1"),
                ("2022-11-16", "1"),
            ],
        )
        .await;
    }
}
