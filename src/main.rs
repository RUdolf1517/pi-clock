use std::{env, fs, path::PathBuf, process};
#[cfg(feature = "tray-app")]
use std::process::{Child, Command, Stdio};


const HELP: &str = r#"pi-clock — часы и π-время

Использование:
  pi-clock [tui]               запустить TUI и трей (если включён в сборку)
  pi-clock --no-tray            запустить только TUI
  pi-clock tray                запустить приложение только в системном трее
  pi-clock install [--path DIR] установить текущий бинарник
  pi-clock help                показать эту справку
  pi-clock version             показать версию

Клавиши TUI: q / Esc — выход, c — цвет, ? — справка.
Для команды tray соберите приложение с feature tray-app.
"#;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.as_slice() {
        [] => {
            run_tui(true);
            Ok(())
        }
        [arg] if arg == "tui" => {
            run_tui(true);
            Ok(())
        }
        [flag] if flag == "--no-tray" => {
            run_tui(false);
            Ok(())
        }
        [command, flag] if command == "tui" && flag == "--no-tray" => {
            run_tui(false);
            Ok(())
        }
        [arg] if matches!(arg.as_str(), "help" | "--help" | "-h") => {
            print!("{HELP}");
            Ok(())
        }
        [arg] if matches!(arg.as_str(), "version" | "--version" | "-V") => {
            println!("pi-clock {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        [arg] if arg == "tray" => run_tray(),
        [arg] if arg == "install" => install(None),
        [command, flag, dir] if command == "install" && flag == "--path" => {
            install(Some(PathBuf::from(dir.as_str())))
        }
        _ => Err(format!("Неизвестная команда или аргументы: {}\n\n{HELP}", args.join(" "))),
    };

    if let Err(error) = result {
        eprintln!("pi-clock: {error}");
        process::exit(2);
    }
}

fn run_tui(with_tray: bool) {
    #[cfg(feature = "tray-app")]
    let mut tray_child = if with_tray {
        launch_tray()
    } else {
        None
    };

    #[cfg(not(feature = "tray-app"))]
    let _ = with_tray;

    pi_clock::tui::run();

    #[cfg(feature = "tray-app")]
    if let Some(mut child) = tray_child.take() {
        let _ = child.kill();
        let _ = child.wait();
    }
}

#[cfg(feature = "tray-app")]
fn launch_tray() -> Option<Child> {
    let executable = match env::current_exe() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("pi-clock: не удалось определить бинарник для запуска трея: {error}");
            return None;
        }
    };
    let mut command = Command::new(executable);
    command
        .arg("tray")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    match command.spawn() {
        Ok(child) => Some(child),
        Err(error) => {
            eprintln!("pi-clock: не удалось автоматически запустить трей: {error}");
            None
        }
    }
}

#[cfg(feature = "tray-app")]
fn run_tray() -> Result<(), String> {
    pi_clock::tray::run().map_err(|error| format!("ошибка приложения в трее: {error}"))
}

#[cfg(not(feature = "tray-app"))]
fn run_tray() -> Result<(), String> {
    Err("команда tray не включена в эту сборку; пересоберите с --features tray-app".into())
}

fn install(path: Option<PathBuf>) -> Result<(), String> {
    let destination_dir = match path {
        Some(path) => path,
        None => default_install_dir()?,
    };
    fs::create_dir_all(&destination_dir)
        .map_err(|error| format!("не удалось создать {}: {error}", destination_dir.display()))?;

    let source = env::current_exe().map_err(|error| format!("не удалось определить путь бинарника: {error}"))?;
    let file_name = source
        .file_name()
        .ok_or_else(|| "у исполняемого файла нет имени".to_string())?;
    let destination = destination_dir.join(file_name);
    if destination.exists()
        && source.canonicalize().ok() == destination.canonicalize().ok()
    {
        println!("pi-clock уже установлен: {}", destination.display());
        return Ok(());
    }

    fs::copy(&source, &destination)
        .map_err(|error| format!("не удалось установить в {}: {error}", destination.display()))?;
    println!("Установлено: {}", destination.display());
    Ok(())
}

#[cfg(windows)]
fn default_install_dir() -> Result<PathBuf, String> {
    env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .map(|path| path.join("Programs").join("pi-clock"))
        .ok_or_else(|| "переменная LOCALAPPDATA не задана".to_string())
}

#[cfg(not(windows))]
fn default_install_dir() -> Result<PathBuf, String> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .map(|path| path.join(".local").join("bin"))
        .ok_or_else(|| "переменная HOME не задана".to_string())
}
