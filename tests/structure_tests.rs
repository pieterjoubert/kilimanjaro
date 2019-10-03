extern crate kilimanjaro;

use kilimanjaro::*;

#[test]
fn module_link_test() {
    assert_eq!(asset_manager::audio::mod_found(), true);
    assert_eq!(asset_manager::materials::mod_found(), true);
    assert_eq!(asset_manager::mesh::mod_found(), true);
    assert_eq!(asset_manager::model::mod_found(), true);
    assert_eq!(event_manager::events::mod_found(), true);
    assert_eq!(event_manager::events::mod_found(), true);
    assert_eq!(game_loop::event_queue::mod_found(), true);
    assert_eq!(game_loop::game_queue::mod_found(), true);
    assert_eq!(game_state::definitions::mod_found(), true);
    assert_eq!(game_state::state::mod_found(), true);
    assert_eq!(utilities::file::mod_found(), true);
    assert_eq!(utilities::math::mod_found(), true);
    assert_eq!(vulkan_wrapper::compute_pipeline::mod_found(), true);
    assert_eq!(vulkan_wrapper::graphics_pipeline::mod_found(), true);
    assert_eq!(vulkan_wrapper::object_loader::mod_found(), true);
    assert_eq!(vulkan_wrapper::shader::mod_found(), true);
    assert_eq!(vulkan_wrapper::transformations::mod_found(), true);
    assert_eq!(window_manager::window_builder::mod_found(), true);
    assert_eq!(window_manager::window_event_manager::mod_found(), true);
}
