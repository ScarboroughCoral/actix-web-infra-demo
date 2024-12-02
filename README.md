# actix-web 简单功能实现记录（方便快速查阅）

## 工程化

- rust-toolchain 自动切换rust版本 https://stackoverflow.com/questions/58226545/how-to-switch-between-rust-toolchains

## 项目结构组织方式

暂时不考虑微服务架构，单服务可以类比 SpringBoot 项目。分为 controller、service、repository、model。

1. 通过`configure(submodule::web_config)`来进行业务拆分

## auth

### 第三方登录

#### Github
1. 原理参考：https://www.ruanyifeng.com/blog/2019/04/github-oauth.html
2. 通过`octocrab` 这个cargo包来处理github登录
3. 流程
   1. 前端请求一个后端接口
   2. 后端接口通过github_client_id 重定向到github 登录地址
   3. github 登录重定向到后端接口，后端拿到code
   4. 通过code,github_client_id,github_client_secret获取到token和用户信息
   5. 返回前端


## 数据库

### diesel orm

1. actix-web diesel sqlite example: https://github.com/actix/examples/tree/master/databases/diesel
2. sql类型到rust类型的映射参考对应的类型实现的From/Into trait：https://docs.diesel.rs/master/diesel/sql_types/index.html
3. 接入文档：https://diesel.rs/guides/getting-started

#### 常用命令
```shell
diesel migration generate create_users # 通过diesel cli 创建迁移脚本
diesel migration run # 根据迁移脚本生成数据库表
diesel migration revert # 回滚最近一个sql操作，过个操作需要回滚多次
```

### 数据库设计

1. 设计范式：https://github.com/CyC2018/CS-Notes/blob/master/notes/%E6%95%B0%E6%8D%AE%E5%BA%93%E7%B3%BB%E7%BB%9F%E5%8E%9F%E7%90%86.md#%E8%8C%83%E5%BC%8F

#### github 第三方登录如何设计 user 表？

## actix-web 实现原理

1. app.service 重载使用了魔法类型系统：https://github.com/alexpusch/rust-magic-patterns/tree/master/axum-style-magic-function-param


## 系统架构

1. 参考 system design primer：https://github.com/donnemartin/system-design-primer