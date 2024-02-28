pub fn test_eon_build() {
    println!("hello eon build!");
}

use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum BuildCommands {
    /// 创建的包的名称
    Create { package_name: String },

    /// 获取当前包的版本
    Version {},

    /// 安装当前包
    Make {},

    /// 将当前包安装到系统缓存中
    Install {},
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct BuildArgs {
    #[command(subcommand)]
    pub command: Option<BuildCommands>,
}
