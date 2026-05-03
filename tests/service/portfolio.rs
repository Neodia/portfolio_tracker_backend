use crate::common::{AssetFixture, IntoDecimal, TestApp};
use chrono::{DateTime, Utc};
use portfolio_tracker_backend::model::{
    AssetAllocation, AssetHoldingsWithDrift, AssetRate, HoldingWithAllocation, PortfolioHoldings,
    PortfolioValueAt,
};
use portfolio_tracker_backend::repository::live::LiveRateRepository;
use portfolio_tracker_backend::repository::traits::PortfolioRepository;
use portfolio_tracker_backend::repository::{OutboxRepository, RateRepository};

/*
   USDC Rate:    1$
   JitoSOL Rate: 100$
   WETH Rate:    2'000$

  Expected portfolio:
       WETH:    50%      200$
       JitoSOL: 50%      200$

  Holdings:
       JitoSOL: 2      200$
       USDC:    300    300

  Portfolio value: JitoSOL (200$) + USDC (300$): 500$

  Drifts:
       JitoSOL:
           Expected:             50%
           Total: 200$ / 500$ =  40%
           Drift: 40% - 50%   = -10%
       USDC:
           Expected:             0%
           Total: 300$ / 500$ = 60%
           Drift: 60% - 0%    = 60%
       WETH:
           *No Holdings*
*/
#[tokio::test]
async fn get_portfolio_works() {
    let TestApp {
        appstate,
        router: _,
        db,
    } = TestApp::new().await;

    // USDC
    let usdc = AssetFixture::usdc_test_asset();
    let usdc_amount = "300".d();
    let usdc_rate_usd = "1".d();
    let usdc_description = None;
    let usdc_expected_allocation = "0".d();
    let usdc_current_expected_allocation = "0.6".d();
    let usdc_expected_drift = "0.6".d();

    // JitoSOL
    let jitosol = AssetFixture::jitosol_test_asset();
    let jitosol_amount = "2".d();
    let jitosol_rate_usd = "100".d();
    let jitosol_description = Some("This is on Jup !".to_string());
    let jitosol_expected_allocation = "0.5".d();
    let jitosol_current_expected_allocation = "0.4".d();
    let jitosol_expected_drift = "-0.1".d();

    // WETH
    let weth = AssetFixture::weth_test_asset();
    let weth_rate_usd = "2000".d();
    let weth_expected_allocation = "0.5".d();

    let expected_portfolio_value = "500".d();

    let user_id = db.with_test_user().await;
    let usdc_asset_id = db.with_test_asset(&usdc).await;
    let jitosol_asset_id = db.with_test_asset(&jitosol).await;
    let weth_asset_id = db.with_test_asset(&weth).await;
    let rates_repo = LiveRateRepository::new_from_pool(db.pool.clone());

    let now = Utc::now();

    // Populate Rates
    let usdc_rate = AssetRate::new(usdc.clone(), usdc_rate_usd);
    let jitosol_rate = AssetRate::new(jitosol.clone(), jitosol_rate_usd);
    let weth_rate = AssetRate::new(weth.clone(), weth_rate_usd);
    let asset_rates = vec![jitosol_rate.clone(), weth_rate.clone(), usdc_rate.clone()];
    let mut tx = db.pool.begin().await.unwrap();
    rates_repo
        .insert_rates(&mut tx, asset_rates, now)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    // Populate expected allocations [No USDC]
    appstate
        .services
        .portfolio_service
        .upsert_expected_asset_allocation(user_id, weth_asset_id, weth_expected_allocation)
        .await
        .unwrap();
    appstate
        .services
        .portfolio_service
        .upsert_expected_asset_allocation(user_id, jitosol_asset_id, jitosol_expected_allocation)
        .await
        .unwrap();

    // Populate holdings [No WETH]
    let jitosol_holding_id = appstate
        .services
        .portfolio_service
        .insert_holding(
            user_id,
            jitosol_asset_id,
            jitosol_amount,
            jitosol_description.clone(),
        )
        .await
        .unwrap();
    let usdc_holding_id = appstate
        .services
        .portfolio_service
        .insert_holding(
            user_id,
            usdc_asset_id,
            usdc_amount,
            usdc_description.clone(),
        )
        .await
        .unwrap();

    // Populate portfolio snapshots
    let first_dt = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let first_value_usd = "15_000".d();
    let second_dt = DateTime::parse_from_rfc3339("2024-01-02T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let second_value_usd = "15_000".d();
    let portfolio_repo = appstate.repositories.portfolio;
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

    let portfolio = appstate
        .services
        .portfolio_service
        .get_portfolio(user_id)
        .await
        .unwrap();

    let expected_allocations = vec![
        AssetAllocation::new(
            jitosol_asset_id,
            jitosol.symbol,
            jitosol.name,
            jitosol.network,
            jitosol.contract_address,
            jitosol_expected_allocation,
        ),
        AssetAllocation::new(
            weth_asset_id,
            weth.symbol,
            weth.name,
            weth.network,
            weth.contract_address,
            weth_expected_allocation,
        ),
    ];

    let expected_portfolio_holdings = PortfolioHoldings::new(
        vec![
            AssetHoldingsWithDrift::new(
                usdc_rate,
                usdc_amount * usdc_rate_usd,
                vec![HoldingWithAllocation::new(
                    usdc_holding_id,
                    usdc_amount,
                    usdc_amount * usdc_rate_usd,
                    usdc_description,
                    usdc_current_expected_allocation,
                )],
                usdc_current_expected_allocation,
                usdc_expected_allocation,
                usdc_expected_drift,
            ),
            AssetHoldingsWithDrift::new(
                jitosol_rate,
                jitosol_amount * jitosol_rate_usd,
                vec![HoldingWithAllocation::new(
                    jitosol_holding_id,
                    jitosol_amount,
                    jitosol_amount * jitosol_rate_usd,
                    jitosol_description,
                    jitosol_current_expected_allocation,
                )],
                jitosol_current_expected_allocation,
                jitosol_expected_allocation,
                jitosol_expected_drift,
            ),
        ],
        expected_portfolio_value,
    );

    assert_eq!(portfolio.expected_asset_allocations, expected_allocations);
    assert_eq!(portfolio.holdings, expected_portfolio_holdings);
    assert_eq!(
        portfolio.historical_portfolio_value,
        vec![
            // Ordered DESC on date
            PortfolioValueAt::new(second_value_usd, second_dt),
            PortfolioValueAt::new(first_value_usd, first_dt),
        ]
    );
}

/*
   2 assets: WETH & JitoSOL

   2 dates:
       first_dt: 2024-01-01T00:00:00Z
       second_dt: 2024-01-02T00:00:00Z

   Rates:
       first_dt:
           WETH:    1'500$
           JitoSOL: 100$
       second_dt:
           WETH:    2'000$
           JitoSOL: 80$

    User Holdings:
       WETH:    1
       JitoSOL: 2

    Expected portfolio values:
       first_dt:
           WETH:       1   *   1'500   =   1'500
           JitoSOL:    2   *   100     =   200
           TOTAL: 1'500 + 200 = 1'700
       second_dt:
           WETH:       1   *   2'000   =   2'000
           JitoSOL:    2   *   80      =   160
           TOTAL: 2'000 + 160 = 2'160
*/
#[tokio::test]
async fn portfolio_snapshots_computation_works() {
    let TestApp {
        appstate,
        router: _,
        db,
    } = TestApp::new().await;
    let user_id = db.with_test_user().await;

    // Two Assets
    let weth = AssetFixture::weth_test_asset();
    let jitosol = AssetFixture::jitosol_test_asset();
    db.with_test_asset(&weth).await;
    db.with_test_asset(&jitosol).await;

    // Two Dates
    let first_dt = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let second_dt = DateTime::parse_from_rfc3339("2024-01-02T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);

    // Rates
    let weth_first_dt_rate = "1_500".d();
    let jitosol_first_dt_rate = "100".d();
    let weth_second_dt_rate = "2_000".d();
    let jitosol_second_dt_rate = "80".d();
    let mut tx = db.pool.begin().await.unwrap();
    appstate
        .repositories
        .rate
        .insert_rates(
            &mut tx,
            vec![
                AssetRate::new(weth.clone(), weth_first_dt_rate),
                AssetRate::new(jitosol.clone(), jitosol_first_dt_rate),
            ],
            first_dt,
        )
        .await
        .unwrap();
    appstate
        .repositories
        .rate
        .insert_rates(
            &mut tx,
            vec![
                AssetRate::new(weth.clone(), weth_second_dt_rate),
                AssetRate::new(jitosol.clone(), jitosol_second_dt_rate),
            ],
            second_dt,
        )
        .await
        .unwrap();
    appstate
        .repositories
        .outbox
        .insert_rates_inserted(&mut tx, first_dt)
        .await
        .unwrap();
    appstate
        .repositories
        .outbox
        .insert_rates_inserted(&mut tx, second_dt)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    // User Holdings
    let weth_holding_amount = "1".d();
    let jitosol_holding_amount = "2".d();
    appstate
        .services
        .portfolio_service
        .insert_holding(user_id, weth.id, weth_holding_amount, None)
        .await
        .unwrap();
    appstate
        .services
        .portfolio_service
        .insert_holding(user_id, jitosol.id, jitosol_holding_amount, None)
        .await
        .unwrap();

    let computation_result = appstate
        .services
        .portfolio_service
        .compute_pending_snapshots()
        .await
        .unwrap();

    let user_historical_portfolio = appstate
        .repositories
        .portfolio
        .get_historical_portfolio_values(user_id)
        .await
        .unwrap();

    assert_eq!(computation_result.number_of_users, 1);
    assert_eq!(computation_result.number_of_snapshots_events, 2);
    
    let expected_first_dt_value = "1_700".d();
    let expected_second_dt_value = "2_160".d();

    // Ordered DESC on date
    assert_eq!(user_historical_portfolio,
    vec![
        PortfolioValueAt::new(expected_second_dt_value, second_dt),
        PortfolioValueAt::new(expected_first_dt_value, first_dt),
    ])
}
