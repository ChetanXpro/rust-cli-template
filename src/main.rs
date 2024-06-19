use clap::{Parser, Subcommand};




#[derive(Parser)]
#[command(author, version)]
#[command(name = "go", version = "1.0", about = "Manages and navigates directories", long_about = None)]
struct Args {
    #[arg(help = "The directory name to navigate to or manage", required = false)]
    dirname: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}


#[derive(Subcommand, Debug)]
enum Commands {
    /// Adds a new directory shortcut
    Add {
        /// Name of the shortcut to add
        #[arg(help = "The shortcut name")]
        name: String,
        /// Path of the directory to add
        #[arg(help = "The directory path to add")]
        path: String,
    },
    /// Removes an existing directory shortcut
    Remove {
        /// Name of the shortcut to remove
        #[arg(help = "The shortcut name")]
        name: String,
        /// Path of the directory to remove
        #[arg(help = "The directory path to remove")]
        path: String,
    },
    /// Views details of a specific directory shortcut
    View {
        /// Name of the shortcut to view
        #[arg(help = "The shortcut name to view")]
        name: String,
    },
}


fn main() {
    let args = Args::parse();

   

    // Handle optional subcommands
    match (&args.dirname, &args.command) {
        (Some(dirname), None) => {
            // If only a directory name is provided, treat it as a request to navigate
            println!("Changing to directory: {}", dirname);
            // Here you would typically trigger some action to handle this
        },
        (_, Some(Commands::Add { name, path })) => {
            println!("Adding new directory '{}': '{}'", name, path);
            // Code to add the directory shortcut
        },
        (_, Some(Commands::Remove { name, path })) => {
            println!("Removing directory '{}': '{}'", name, path);
            // Code to remove the directory shortcut
        },
        (_, Some(Commands::View { name })) => {
            println!("Viewing details for directory '{}'", name);
            // Code to display details of the directory shortcut
        },
        (None, None) => {
            println!("No directory specified and no command provided. Please specify a directory or use a subcommand.");
        },

    }
}




// const LINUX_HOSTS_PATH: &str = "/etc/hosts";
// const MACOS_HOSTS_PATH: &str = "/etc/hosts";
// const WINDOWS_HOSTS_PATH: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

// #[inline]
// const fn get_hosts_path() -> &'static str {
//     if cfg!(target_os = "linux") {
//         LINUX_HOSTS_PATH
//     } else if cfg!(target_os = "windows") {
//         WINDOWS_HOSTS_PATH
//     } else if cfg!(target_os = "macos") {
//         MACOS_HOSTS_PATH
//     } else {
//         panic!("Unsupported operating system");
//     }
// }

// fn print_events_with_timer(timer_duration: Duration) -> Result<()> {
//     let start_time = Instant::now();
//     println!("  ESC or 'e' to exit early");
//     loop {
//         // Wait up to 1s for another event
//         if poll(Duration::from_millis(1_000))? {
//             // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
//             let event = read()?;

//             if event == Event::Key(KeyCode::Char('e').into()) {
//                 break;
//             }
//             if event == Event::Key(KeyCode::Esc.into()) {
//                 break;
//             }
//         } else {
//             if start_time.elapsed() >= timer_duration {
//                 break;
//             } else {
//             }
//         }
//     }

//     Ok(())
// }
// fn block_websites(time_to_sleep: u64, task: &String, user_input_time: &String) -> io::Result<()> {
//     let hosts_path: &str = get_hosts_path();
//     let mut backup_path: PathBuf = PathBuf::new();
//     let mut toml_config_path: PathBuf = PathBuf::new();

//     if let Some(proj_dirs) = ProjectDirs::from("com", "chetanxpro", "focusguard") {
//         let config_dir = proj_dirs.config_dir();

//         if !config_dir.join("config.toml").exists() {
//             println!("Please run `focus setup --list <exact path to website list>` to setup focus");
//             std::process::exit(1);
//         }

//         if !config_dir.exists() {
//             fs::create_dir_all(config_dir).expect("Error while creating config directory");

//             backup_path = config_dir.join("hosts_backup");

//             fs::File::create(&backup_path).expect("Error while creating hosts backup file");

//             let mut backup_host_file_for_emergency =
//                 fs::File::create(config_dir.join("hosts_backup_for_revert"))
//                     .expect("Error while creating hosts backup file");

//             backup_host_file_for_emergency
//                 .write_all(fs::read_to_string(hosts_path).unwrap().as_bytes())
//                 .expect("Error while writing to backup file");
//         }

//         backup_path = config_dir.join("hosts_backup");
//         toml_config_path = config_dir.join("config.toml");

//         // dbg!(config_dir);
//     }

//     let website_file_option = get_websites_path(toml_config_path);

//     let websites_file_path = website_file_option.as_deref().unwrap_or("default");

//     let mut hosts_content: String =
//         fs::read_to_string(hosts_path).expect("Error while reading host file content");
//     let websites_list_content: String =
//         fs::read_to_string(websites_file_path).expect("Error while reading website content");

//     // dbg!(&backup_path);
//     let mut backup_file = OpenOptions::new()
//         .write(true)
//         .truncate(true)
//         .open(&backup_path)
//         .expect("Not able to open backup file");

//     backup_file
//         .write_all(hosts_content.as_bytes())
//         .expect("Error while writing to backup file");

//     // website_list_path.clone();

//     let website_list: std::str::Split<'_, &str> = websites_list_content.split("\n");

//     hosts_content.push_str(&format!("\n# ========== Temp Hosts ========="));
//     for website in website_list {
//         println!("Website: {}", website);

//         if !hosts_content.contains(website) {
//             hosts_content.push_str(&format!("\n127.0.0.1\t{}", website));
//         }
//     }
//     hosts_content.push_str(&format!("\n# ========== Temp Hosts ========="));
//     println!("Content:\n {}", hosts_content);

//     let mut file = OpenOptions::new()
//         .write(true)
//         .truncate(true)
//         .open(hosts_path)?;
//     file.write_all(hosts_content.as_bytes())?;

//     let formated_message = format!(
//         "Blocked websites for {} for task: {}",
//         user_input_time, task
//     );
//     let mut sp = Spinner::new(Spinners::Dots9, formated_message.into());
//     enable_raw_mode()?;

//     let timer_duration = Duration::from_millis(time_to_sleep);

//     if let Err(e) = print_events_with_timer(timer_duration) {
//         println!("Error: {:?}\r", e);
//     }

//     disable_raw_mode()?;
//     sp.stop();

//     let backup_file_content: String =
//         fs::read_to_string(backup_path).expect("Error while reading backup file");

//     let mut backup_file = OpenOptions::new()
//         .write(true)
//         .truncate(true)
//         .open(hosts_path)
//         .unwrap();

//     backup_file
//         .write_all(backup_file_content.as_bytes())
//         .unwrap();

//     println!("\n  Unblocked websites ✅");
//     Ok(())
// }

// fn get_websites_path(config_path: PathBuf) -> Option<String> {
//     let config_content = fs::read_to_string(config_path).unwrap();
//     let value: Value = toml::from_str(&config_content).unwrap();

//     let website_list_path = value
//         .get("website_list_path")
//         .and_then(Value::as_str)
//         .map(String::from);
//     // .unwrap();

//     website_list_path
// }

// fn main() {
//     let cli = Cli::parse();

    // match &cli.command {
    //     Some(Commands::Setup(setup)) => {
    //         // println!("List path: {}", setup.list);
    //         if let Some(proj_dirs) = ProjectDirs::from("com", "chetanxpro", "focusguard") {
    //             let config_dir = proj_dirs.config_dir();

    //             if !config_dir.exists() {
    //                 fs::create_dir_all(config_dir).expect("Error while creating config directory");

    //                 fs::File::create(config_dir.join("hosts_backup"))
    //                     .expect("Error while creating hosts backup file");
    //             }

    //             let config = Config {
    //                 website_list_path: setup.list.clone(),
    //             };

    //             let toml = toml::to_string(&config).unwrap();

    //             fs::File::create(config_dir.join("config.toml")).unwrap();

    //             let mut config_file = OpenOptions::new()
    //                 .write(true)
    //                 .truncate(true)
    //                 .open(config_dir.join("config.toml"))
    //                 .unwrap();

    //             config_file.write_all(toml.as_bytes()).unwrap();

    //             println!("Website file path saved in config ✅")
    //         }
    //     }
    //     Some(Commands::Reset) => {
    //         let hosts_path: &str = get_hosts_path();
    //         let mut backup_path: PathBuf = PathBuf::new();

    //         if let Some(proj_dirs) = ProjectDirs::from("com", "chetanxpro", "focusguard") {
    //             let config_dir = proj_dirs.config_dir();

    //             backup_path = config_dir.join("hosts_backup");
    //         }

    //         let backup_file_content: String =
    //             fs::read_to_string(backup_path).expect("Error while reading backup file");

    //         let mut host_file = OpenOptions::new()
    //             .write(true)
    //             .truncate(true)
    //             .open(hosts_path)
    //             .unwrap();

    //         host_file.write_all(backup_file_content.as_bytes()).unwrap();

    //         println!("Hosts file reset ✅")
    //     }
    //     None => {
    //         if let (Some(time), Some(task)) = (cli.time, cli.task) {
    //             // println!("lol")
    //             // println!("Time: {:#?} , Task: {}", time, task);

    //             let mut time_in_milliseconds: u64;
    //             // let time: String = cli.time;
    //             // let task: String = args.task;

    //             if time.contains("m") {
    //                 time_in_milliseconds = time.replace("m", "").parse().unwrap();
    //                 time_in_milliseconds = time_in_milliseconds * 60 * 1000
    //             } else if time.contains("s") {
    //                 time_in_milliseconds = time.replace("s", "").parse().unwrap();
    //                 time_in_milliseconds = time_in_milliseconds * 1000
    //             } else if time.contains("h") {
    //                 time_in_milliseconds = time.replace("h", "").parse().unwrap();
    //                 time_in_milliseconds = time_in_milliseconds * 60 * 60 * 1000
    //             } else {
    //                 time_in_milliseconds = 0
    //             }

    //             block_websites(time_in_milliseconds, &task, &time).expect("Error")
    //         } else {
    //             println!("No command provided");
    //         }
    //     }
    // }
// }
