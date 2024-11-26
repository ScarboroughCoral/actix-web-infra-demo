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