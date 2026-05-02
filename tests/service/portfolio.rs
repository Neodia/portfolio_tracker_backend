use crate::common::{AssetFixture, IntoDecimal, TestApp};
use chrono::Utc;
use portfolio_tracker_backend::model::{
    AssetAllocation, AssetHoldingsWithDrift, AssetPrice,
    HoldingWithAllocation, PortfolioHoldings,
};
use portfolio_tracker_backend::repository::live::LiveRateRepository;
use portfolio_tracker_backend::repository::RateRepository;

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
    let rates_repo = LiveRateRepository::default();

    let now = Utc::now();

    // Populate Rates
    let usdc_rate = AssetPrice::new(usdc.clone(), usdc_rate_usd);
    let jitosol_rate = AssetPrice::new(jitosol.clone(), jitosol_rate_usd);
    let weth_rate = AssetPrice::new(weth.clone(), weth_rate_usd);
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
    appstate
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
    appstate
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
    assert_eq!(portfolio.historical_portfolio_value, vec![]); // TODO: Change this when doing the historical portfolio value job
}
