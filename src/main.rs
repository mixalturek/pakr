use log::error;
use log::info;

fn main() {
    let result = pakr::run();
    let exit_code = match result {
        Ok(_x) => 0,
        Err(e) => {
            error!("Application execution failed: {}", e);

            // TODO: Logging may not be initialized.
            eprintln!("ERROR: Application execution failed: {}", e);
            1
        }
    };

    info!("Exiting with code: {}", exit_code);
    std::process::exit(exit_code);
}
