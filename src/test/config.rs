use crate::config::GameConfig;

#[test]
fn generating_positions() {
    let config = GameConfig::create_classic();

    // Return error if too many detectives are requested
    assert!(config.gen_start_positions(config.det_start_pos_list.len() + 1).is_err());
}