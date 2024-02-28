pub mod eon;

use clap::Parser;
use eon::{eon_build::BuildCommands, eon_env::EnvCommands};

fn main() {
    let args = eon::Cli::parse();
    match args.command {
        eon::Commands::Install { soft_name } => {
            println!("install {}", soft_name);
        }
        eon::Commands::Env(env) => match env.command {
            Some(env_args) => match env_args {
                EnvCommands::Info { env_soft_name } => match env_soft_name {
                    Some(sf) => {
                        println!("显示 {} 系统软件的信息!", sf);
                    }
                    None => {}
                },
                EnvCommands::Install { env_soft_name } => match env_soft_name {
                    Some(sf) => {
                        println!("安装 {} 系统软件!", sf);
                    }
                    None => (),
                },
                _ => (),
            },
            None => (),
        },

        eon::Commands::Build(build) => match build.command {
            Some(build_args) => match build_args {
                BuildCommands::Create { package_name } => {
                    println!("eon 创建一个包工程 {}", package_name);
                }
                BuildCommands::Install {} => {
                    println!("当前系统包的版本为：x.x.x");
                }
                BuildCommands::Make {} => {
                    println!("构建当当前工程");
                }
                BuildCommands::Version {} => {
                    println!("显示当前工程的版本!");
                }
            },
            None => (),
        },
        // _ => (),
    }
}
