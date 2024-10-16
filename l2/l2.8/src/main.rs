fn main() {
    shell::process();
}

mod shell {
    use sysinfo::{System, Process};
    use std::io::stdin;
    use std::path::{Path, PathBuf};
    use std::env::{current_dir, set_current_dir};

    pub fn process() {
        let sys = System::new_all();
        loop {
            // Приглашение в stdout
            let cd = current_dir().unwrap();
            println!("{cd:?} ~ %");

            // Ввод в stdin и выполнение
            // Команда echo поддерживает '*' для просмотра содержимого текущей директории
            let cmds = parse_cmd_args();
            for cmd in cmds {
                cmd.exec(&cd, &sys);
            }

            println!();
        }
    }

    #[derive(Debug)]
    enum Cmd {
        ChangeDirectory(String),
        PresentWorkingDirectory,
        Echo(String),
        Kill(String),
        ProcessStatus,
        Unknown(String, Vec<String>),
    }

    impl Cmd {
        fn exec(&self, cd: &PathBuf, sys: &System) {
            match self {
                Cmd::ChangeDirectory(dir) => {
                    let dir = Path::new(&dir);
                    if dir.exists() {
                        if dir.is_dir() {
                            set_current_dir(dir).unwrap();
                        } else {
                            println!("Choosen path is not a directory");
                        }
                    } else {
                        println!("No sush file or directory");
                    }
                },
                Cmd::PresentWorkingDirectory => {
                    println!("{cd:?}");
                },
                Cmd::Echo(args) => {
                    if args == "*" {
                        if let Ok(dir) = cd.read_dir() {
                            let dir = dir.filter_map(|d|d.ok()).map(|d|d.file_name()).collect::<Vec<_>>();
                            println!("{dir:?}");
                        }
                    } else {
                        println!("{args}");
                    }
                },
                Cmd::Kill(name) => {
                    let mut process: Option<&Process> = None;
                     for (_, pr) in sys.processes() {
                        if pr.name().to_str().unwrap().contains(name) {
                            process = Some(pr);
                            return;
                        }
                    };
                    if let Some(process) = process {
                        let name = process.name().to_str().unwrap().to_string();
                        if process.kill() {
                            println!("Process with name {name} was killed");
                        } else {
                            println!("Process with name {name} was not killed");
                        }
                    }
                },
                Cmd::ProcessStatus => {
                    println!("{: <10} {: <50} {: <10}", "id", "name", "runtime,ms");
                    for (id, process) in sys.processes().iter().take(20) {
                        let name=  process.name().to_str().unwrap().to_string();
                        let runtime = process.run_time() * 1000;
                        println!("{: <10} {: <50} {: <10}", id.to_string(), name, runtime);
                    }
                },
                Cmd::Unknown(cmd, args) => {
                    let mut child = match std::process::Command::new(cmd).args(args).spawn() {
                        Ok(child) => child,
                        _ => return
                    };
                    match child.wait() {
                        Ok(res) => println!("{res}"),
                        _ => println!(),
                    }
                }
                _ => {},
            }
        }
    }

    fn parse_cmd_args() -> Vec<Cmd> {
        let mut input_str = String::new();
        stdin().read_line(&mut input_str).unwrap();
        let commands_str = input_str.split("|");
        let mut commands_exe = vec![];

        for command in commands_str {
            let command = command.trim();
            let (cmd, args) = command.split_once(' ').unwrap_or((command, ""));
            let cmd = match cmd {
                "cd" => Cmd::ChangeDirectory(args.to_string()),
                "pwd" => Cmd::PresentWorkingDirectory,
                "echo" => Cmd::Echo(args.to_string()),
                "kill" => Cmd::Kill(args.to_string()),
                "ps" => Cmd::ProcessStatus,
                _ => Cmd::Unknown(cmd.to_string(), args.split_whitespace().map(|s|s.to_string()).collect()),
            };
            commands_exe.push(cmd);
        }

        return commands_exe;
    }
}