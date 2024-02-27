use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "eon 命令行工具")]
#[command(version = "0.0.1")]
#[command(about = "eon cli", long_about = "eon 开发工具箱子")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// 安装一个软件
    #[command(arg_required_else_help = true)]
    Install { soft_name: String },

    /// 安装多个软件
    #[command(arg_required_else_help = true)]
    InstallMulti {
        #[arg(required = true)]
        softs: Vec<String>,
    },

    /// 环境相关指令
    Env(EnvArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct EnvArgs {
    #[command(subcommand)]
    command: Option<EnvCommands>,
}

#[derive(Debug, Subcommand)]
enum EnvCommands {
    /// 显示环境信息
    List {},
    /// 显示指定环境软件的版本
    Info { soft_name: Option<String> },
    /// 安装需要的环境软件
    Install { softs: Option<String> },
    /// 卸载环境软件
    Uninstall { softs: Option<String> },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Install { soft_name } => {
            println!("安装软件 {soft_name}");
        }
        Commands::InstallMulti { softs } => {
            for i in 0..softs.len() {
                let item = &softs[i];
                println!("即将安装... {item}");
            }
        }
        Commands::Env(env) => match env.command {
            Some(envargs) => match envargs {
                EnvCommands::Info { soft_name } => match soft_name {
                    Some(sf) => {
                        println!("显示 {} 系统软件的信息!", sf);
                    }
                    None => {
                        println!("Info 必须指定软件的名字");
                    }
                },
                EnvCommands::List {} => {
                    println!("显示当前环境的系统软件环境");
                }
                EnvCommands::Install { softs } => match softs {
                    Some(sfs) => {
                        println!("安装系统软件 {}", sfs);
                    }
                    None => {
                        println!("Install 必须指定一个软件,例如: cmake eCAL");
                    }
                },
                EnvCommands::Uninstall { softs } => match softs {
                    Some(sfs) => {
                        println!("卸载系统软件 {}", sfs);
                    }
                    None => {
                        println!("Uninstall 必须指定一个已安装的软件,例如：cmake eCAL");
                    }
                },
            },
            None => {
                unreachable!("env 必须指定参数!");
            }
        },
    }
}
