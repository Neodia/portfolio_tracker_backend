use crate::common::{AssetFixture, DBFixture, IntoDecimal};
use chrono::{DateTime, Utc};
use itertools::Itertools;
use portfolio_tracker_backend::model::{
    AssetAllocation, AssetHoldings, AssetRate, Holding, PortfolioValueAt, UserHolding,
};
use portfolio_tracker_backend::repository::RateRepository;
use portfolio_tracker_backend::repository::live::{LivePortfolioRepository, LiveRateRepository};
use portfolio_tracker_backend::repository::traits::PortfolioRepository;
use rust_decimal::Decimal;
use std::collections::HashSet;

#[tokio::test]
async fn upsert_inserts_allocation() {
    let db = DBFixture::new().await;
    let asset = AssetFixture::jitosol_test_asset();

    let user_id = db.with_test_user().await;
    let asset_id = db.with_test_asset(&asset).await;
    let repo = LivePortfolioRepository::new_from_pool(db.pool.clone());

    let allocation = Decimal::from_str_exact("0.15").unwrap();
    let now = Utc::now();

    repo.upsert_expected_asset_allocation(user_id, asset_id, allocation, now)
        .await
        .unwrap();
    let allocations = repo.get_expected_asset_allocations(user_id).await.unwrap();
    assert_eq!(
        allocations,
        vec!(AssetAllocation::new(
            asset_id,
            asset.symbol,
            asset.name,
            asset.network,
            asset.contract_address,
            allocation,
        ))
    );
}
#[tokio::test]
async fn upsert_updates_after_inserting() {
    let db = DBFixture::new().await;
    let asset = AssetFixture::jitosol_test_asset();

    let user_id = db.with_test_user().await;
    let asset_id = db.with_test_asset(&asset).await;
    let repo = LivePortfolioRepository::new_from_pool(db.pool.clone());

    let allocation = Decimal::from_str_exact("0.15").unwrap();
    let now = Utc::now();

    repo.upsert_expected_asset_allocation(user_id, asset_id, allocation, now)
        .await
        .unwrap();

    let new_allocation = Decimal::from_str_exact("0.20").unwrap();
    repo.upsert_expected_asset_allocation(user_id, asset_id, new_allocation, now)
        .await
        .unwrap();
    let allocations = repo.get_expected_asset_allocations(user_id).await.unwrap();
    assert_eq!(
        allocations,
        vec!(AssetAllocation::new(
            asset_id,
            asset.symbol,
            asset.name,
            asset.network,
            asset.contract_address,
            new_allocation,
        ))
    );
}
#[tokio::test]
async fn delete_allocation_works() {
    let db = DBFixture::new().await;
    let asset = AssetFixture::jitosol_test_asset();

    let user_id = db.with_test_user().await;
    let asset_id = db.with_test_asset(&asset).await;
    let repo = LivePortfolioRepository::new_from_pool(db.pool.clone());

    let allocation = Decimal::from_str_exact("0.15").unwrap();
    let now = Utc::now();

    repo.upsert_expected_asset_allocation(user_id, asset_id, allocation, now)
        .await
        .unwrap();
    repo.delete_expected_asset_allocation(user_id, asset_id)
        .await
        .unwrap();
    let allocations = repo.get_expected_asset_allocations(user_id).await.unwrap();
    assert_eq!(allocations.len(), 0);
}
#[tokio::test]
async fn insert_holding_works() {
    let db = DBFixture::new().await;
    let asset = AssetFixture::jitosol_test_asset();

    let user_id = db.with_test_user().await;
    let asset_id = db.with_test_asset(&asset).await;
    let repo = LivePortfolioRepository::new_from_pool(db.pool.clone());

    let now = Utc::now();
    let asset_amount = Decimal::from_str_exact("234").unwrap();
    let description = Some("This one is stored on Phantom !".to_string());

    repo.insert_holding(user_id, asset_id, asset_amount, description.clone(), now)
        .await
        .unwrap();
    let holdings = db.get_user_holdings(user_id).await;
    assert_eq!(holdings.len(), 1);
    let holding = &holdings[0];
    assert_eq!(holding.asset_id, asset_id);
    assert_eq!(holding.amount, asset_amount);
    assert_eq!(holding.description, description);
}
#[tokio::test]
async fn update_holding_works() {
    let db = DBFixture::new().await;
    let asset = AssetFixture::jitosol_test_asset();

    let user_id = db.with_test_user().await;
    let asset_id = db.with_test_asset(&asset).await;
    let repo = LivePortfolioRepository::new_from_pool(db.pool.clone());

    let now = Utc::now();
    let asset_amount = Decimal::from_str_exact("234").unwrap();
    let description = Some("This one is stored on Phantom !".to_string());

    repo.insert_holding(user_id, asset_id, asset_amount, description.clone(), now)
        .await
        .unwrap();

    let new_description = Some("This one is stored on Backpack !".to_string());
    let holdings = db.get_user_holdings(user_id).await;
    let holding = holdings.first().unwrap();
    repo.update_holding(
        holding.id,
        user_id,
        asset_amount,
        new_description.clone(),
        now,
    )
    .await
    .unwrap();

    let holdings = db.get_user_holdings(user_id).await;
    assert_eq!(holdings.len(), 1);
    let holding = &holdings[0];
    assert_eq!(holding.asset_id, asset_id);
    assert_eq!(holding.amount, asset_amount);
    assert_eq!(holding.description, new_description);
}
#[tokio::test]
async fn delete_holding_works() {
    let db = DBFixture::new().await;
    let asset = AssetFixture::jitosol_test_asset();

    let user_id = db.with_test_user().await;
    let asset_id = db.with_test_asset(&asset).await;
    let repo = LivePortfolioRepository::new_from_pool(db.pool.clone());

    let now = Utc::now();
    let asset_amount = Decimal::from_str_exact("234").unwrap();
    let description = Some("This one is stored on Phantom !".to_string());

    repo.insert_holding(user_id, asset_id, asset_amount, description.clone(), now)
        .await
        .unwrap();
    let holdings = db.get_user_holdings(user_id).await;
    let holding = holdings.first().unwrap();
    repo.delete_holding(holding.id, user_id).await.unwrap();

    let holdings = db.get_user_holdings(user_id).await;
    assert_eq!(holdings.len(), 0);
}
#[tokio::test]
async fn get_holdings_works() {
    let db = DBFixture::new().await;
    let jitosol = AssetFixture::jitosol_test_asset();
    let jitosol_amount = "2".d();
    let jitosol_rate_usd = "100".d();
    let jitosol_description = Some("This is on Jup !".to_string());
    let weth = AssetFixture::weth_test_asset();
    let weth_amount = "1".d();
    let weth_rate_usd = "2000".d();
    let weth_description = None;

    let user_id = db.with_test_user().await;
    let jitosol_asset_id = db.with_test_asset(&jitosol).await;
    let weth_asset_id = db.with_test_asset(&weth).await;
    let portfolio_repo = LivePortfolioRepository::new_from_pool(db.pool.clone());
    let rates_repo = LiveRateRepository::new_from_pool(db.pool.clone());

    let now = Utc::now();

    // Populate Rates
    let jitosol_rate = AssetRate::new(jitosol, jitosol_rate_usd);
    let weth_rate = AssetRate::new(weth, weth_rate_usd);
    let asset_rates = vec![jitosol_rate.clone(), weth_rate.clone()];
    let mut tx = db.pool.begin().await.unwrap();
    rates_repo
        .insert_rates(&mut tx, asset_rates, now)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    // Populate Holdings
    let jitosol_holding_id = portfolio_repo
        .insert_holding(
            user_id,
            jitosol_asset_id,
            jitosol_amount,
            jitosol_description.clone(),
            now,
        )
        .await
        .unwrap();
    let weth_holding_id = portfolio_repo
        .insert_holding(
            user_id,
            weth_asset_id,
            weth_amount,
            weth_description.clone(),
            now,
        )
        .await
        .unwrap();

    let holdings: Vec<_> = portfolio_repo
        .get_holdings(user_id)
        .await
        .unwrap()
        .into_iter()
        .sorted_by_key(|holdings| holdings.asset_rate.asset.symbol.clone()) // ensures JitoSOL comes before WETH
        .collect();
    assert_eq!(
        holdings,
        vec!(
            AssetHoldings::new(
                jitosol_rate,
                jitosol_rate_usd * jitosol_amount,
                vec!(Holding::new(
                    jitosol_holding_id,
                    jitosol_amount,
                    jitosol_rate_usd * jitosol_amount,
                    jitosol_description
                ))
            ),
            AssetHoldings::new(
                weth_rate,
                weth_rate_usd * weth_amount,
                vec!(Holding::new(
                    weth_holding_id,
                    weth_amount,
                    weth_rate_usd * weth_amount,
                    weth_description
                ))
            )
        )
    );
}

#[tokio::test]
async fn get_all_users_holdings_works() {
    let db = DBFixture::new().await;

    let user_1 = db.with_test_user().await;
    let user_2 = db.with_test_user().await;

    let jitosol = AssetFixture::jitosol_test_asset();
    let jitosol_amount = "2".d();
    db.with_test_asset(&jitosol).await;

    let now = Utc::now();

    let portfolio_repo = LivePortfolioRepository::new_from_pool(db.pool.clone());
    let user_1_holding = portfolio_repo
        .insert_holding(user_1, jitosol.id, jitosol_amount, None, now)
        .await
        .unwrap();
    let user_2_holding = portfolio_repo
        .insert_holding(user_2, jitosol.id, jitosol_amount, None, now)
        .await
        .unwrap();

    // Hashset to not care about ordering
    let all_users_holdings: HashSet<_> = portfolio_repo
        .get_all_users_holdings()
        .await
        .unwrap()
        .into_iter()
        .collect();

    assert_eq!(
        all_users_holdings,
        HashSet::from_iter(vec![
            (
                user_1,
                vec![UserHolding::new(
                    user_1_holding,
                    user_1,
                    jitosol.id,
                    jitosol_amount,
                    None
                )],
            ),
            (
                user_2,
                vec![UserHolding::new(
                    user_2_holding,
                    user_2,
                    jitosol.id,
                    jitosol_amount,
                    None
                )],
            ),
        ])
    );
}

#[tokio::test]
async fn insert_portfolio_snapshots_works() {
    let db = DBFixture::new().await;
    let user_id = db.with_test_user().await;

    let first_dt = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let first_value_usd = "15_000".d();
    let second_dt = DateTime::parse_from_rfc3339("2024-01-02T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let second_value_usd = "15_000".d();

    let portfolio_repo = LivePortfolioRepository::new_from_pool(db.pool.clone());
    let mut tx = db.pool.begin().await.unwrap();
    portfolio_repo
        .insert_portfolio_snapshots(&mut tx, vec![(&user_id, first_value_usd)], first_dt)
        .await
        .unwrap();
    portfolio_repo
        .insert_portfolio_snapshots(&mut tx, vec![(&user_id, second_value_usd)], second_dt)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    let historical_portfolio_values = portfolio_repo
        .get_historical_portfolio_values(user_id)
        .await
        .unwrap();

    // Ordered DESC on date
    assert_eq!(
        historical_portfolio_values,
        vec![
            PortfolioValueAt::new(second_value_usd, second_dt),
            PortfolioValueAt::new(first_value_usd, first_dt),
        ]
    );
}
