pub fn test_eon_env() {
    println!("hello eon env!");
}

use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum EnvCommands {
    /// 显示环境信息
    List {},

    /// 显示指定软件的版本信息
    Info { env_soft_name: Option<String> },

    /// 安装指定名称的软件（环境软件，比如 CMake gcc eCAL 等）
    Install { env_soft_name: Option<String> },

    /// 卸载指定名称的软件 （仅支持环境软件）
    Uninstall { env_soft_name: Option<String> },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct EnvArgs {
    #[command(subcommand)]
    pub command: Option<EnvCommands>,
}
