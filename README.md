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

1. 接入文档：https://diesel.rs/guides/getting-started