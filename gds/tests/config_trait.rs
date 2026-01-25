use gds::config::GraphStoreConfig;

#[test]
fn config_trait_object_validate() {
    let cfg: Box<dyn gds::config::ValidatedConfig> = Box::new(GraphStoreConfig::default());
    assert!(cfg.validate().is_ok());
}
