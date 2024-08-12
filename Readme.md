# 基于领域驱动设计 (DDD) 原则的 Axum 项目目录架构
my_ddd_axum_project/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config/
│   │   ├── mod.rs
│   │   └── settings.rs
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── user/
│   │   │   ├── mod.rs
│   │   │   ├── model.rs
│   │   │   ├── repository.rs
│   │   │   ├── service.rs
│   │   │   └── value_objects.rs
│   │   └── other_domain.rs
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   ├── database.rs
│   │   ├── user/
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   └── other_infra.rs
│   ├── presentation/
│   │   ├── mod.rs
│   │   ├── routes/
│   │   │   ├── mod.rs
│   │   │   ├── user_routes.rs
│   │   │   └── other_routes.rs
│   │   ├── handlers/
│   │   │   ├── mod.rs
│   │   │   ├── user_handlers.rs
│   │   │   └── other_handlers.rs
│   ├── application/
│   │   ├── mod.rs
│   │   ├── user/
│   │   │   ├── mod.rs
│   │   │   ├── commands.rs
│   │   │   ├── queries.rs
│   │   │   └── other_application.rs
│   └── utils/
│       ├── mod.rs
│       ├── error.rs
│       └── other_utils.rs
└── tests/
├── mod.rs
├── user_tests.rs
└── other_tests.rs



# 目录和文件说明
Cargo.toml: 项目的配置文件，包含依赖项、元数据等。

src/: 源代码目录

main.rs: 程序入口，包含应用启动和配置代码。
lib.rs: 如果你希望将项目打包为库，可以在这里定义库接口。
config/: 配置模块，包含应用程序的配置管理

mod.rs: 配置模块的入口。
settings.rs: 配置读取和管理代码。
domain/: 领域层，包含核心业务逻辑和领域模型

mod.rs: 领域层模块的入口。
user/: 用户领域模块
mod.rs: 用户领域模块的入口。
model.rs: 用户领域模型。
repository.rs: 用户领域的仓库接口。
service.rs: 用户领域服务。
value_objects.rs: 用户领域的值对象。
other_domain.rs: 其他领域模块。
infrastructure/: 基础设施层，包含与外部系统的集成

mod.rs: 基础设施层模块的入口。
database.rs: 数据库连接和操作代码。
user/: 用户领域的基础设施实现
mod.rs: 用户基础设施模块的入口。
repository.rs: 用户领域的仓库实现。
other_infra.rs: 其他基础设施代码。
presentation/: 表现层，包含路由和请求处理逻辑

mod.rs: 表现层模块的入口。
routes/: 路由模块
mod.rs: 路由模块的入口。
user_routes.rs: 用户相关的路由。
other_routes.rs: 其他路由。
handlers/: 请求处理模块
mod.rs: 请求处理模块的入口。
user_handlers.rs: 用户请求处理器。
other_handlers.rs: 其他请求处理器。
application/: 应用层，包含应用服务和用例

mod.rs: 应用层模块的入口。
user/: 用户相关的应用服务
mod.rs: 用户应用服务模块的入口。
commands.rs: 用户相关的命令处理。
queries.rs: 用户相关的查询处理。
other_application.rs: 其他应用服务。
utils/: 工具模块，包含辅助函数和工具

mod.rs: 工具模块的入口。
error.rs: 错误处理代码。
other_utils.rs: 其他工具。
tests/: 测试目录，包含集成测试

mod.rs: 测试模块的入口。
user_tests.rs: 用户相关的测试。
other_tests.rs: 其他测试。