pub mod eon_build;
pub mod eon_env;

use clap::{Parser, Subcommand};

use self::{eon_build::BuildArgs, eon_env::EnvArgs};

#[derive(Debug, Parser)]
#[command(name = "eon 命令行工具")]
#[command(version = "0.1.0")]
#[command(about = "eon cli", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// 安装一个软件
    #[command(arg_required_else_help = true)]
    Install { soft_name: String },

    /// 环境相关指令
    Env(EnvArgs),

    /// 构建相关指令
    Build(BuildArgs),
}
