# 下载插件

cargo install diesel_cli --no-default-features --features "mysql"

# 生成表脚本

diesel setup

# 根据数据库表生成实体类

diesel migration run
