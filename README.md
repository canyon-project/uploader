# Canyon Uploader

用于扫描本地 `.canyon_output` 文件中的覆盖率数据，并将其上报到指定的服务器。

## 安装

```sh npm2yarn
npm install -g canyon-uploader
```

## 使用方法

```shell
canyon-uploader map --dsn=http://xxx.com/coverage/map/client --provider=tripgl
```

## 参数

| 配置项       | 描述                                         | 是否必填 | 默认值        |
| ------------ | -------------------------------------------- | -------- | ------------- |
| dsn          | 覆盖率报告 URL，CI 流水线变量键用于检测 DSN | 是       | 无            |
| provider     | 源代码提供者（可选），默认为 GitLab         | 可选     | gitlab        |
| projectID    | 仓库 ID                                     | 一般无需手动配置（从 CI 提供者自动检测） | 无            |
| sha          | Git 提交 SHA                                 | 一般无需手动配置（从 CI 提供者自动检测） | 无            |