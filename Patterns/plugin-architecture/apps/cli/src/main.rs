use plugin_interface::PluginContext;
use plugin_manager::PluginManager;
use std::path::PathBuf;

fn main() {
    println!("=== Core Application Starting ===\n");

    // Initialize PluginManager
    let mut plugin_manager = PluginManager::new();

    // Create plugins directory path
    let plugin_dir = PathBuf::from("target/debug/plugins");

    // Discover and load plugins
    println!("Discovering plugins in: {:?}\n", plugin_dir);
    if let Err(e) = plugin_manager.discover_plugins(&plugin_dir) {
        eprintln!("Error discovering plugins: {}", e);
    }

    // Create PluginContext with sample data
    let mut context = PluginContext::new();
    context.data.insert("name".to_string(), "Alice".to_string());
    context.data.insert("operation".to_string(), "add".to_string());
    context.data.insert("a".to_string(), "10".to_string());
    context.data.insert("b".to_string(), "5".to_string());

    // Execute all plugins
    println!("\n=== Executing Plugins ===\n");
    let results = plugin_manager.execute_all(&context);

    for (i, result) in results.iter().enumerate() {
        match result {
            | Ok(output) => println!("Plugin {} output: {}", i + 1, output),
            | Err(e) => eprintln!("Plugin {} error: {}", i + 1, e),
        }
    }

    // Shutdown
    println!("\n=== Shutting Down ===\n");
    if let Err(e) = plugin_manager.shutdown() {
        eprintln!("Error during shutdown: {}", e);
    }

    println!("\n=== Core Application Exiting ===");
}
