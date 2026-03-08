# cmod


通过 rust 实现的go package辅助工具

从 https://pkg.go.dev/ 搜索并获取 package 信息

![](image/gust.png)

## 命令

- 运行
    ```shell
    cmod [OPTIONS] <TARGET>
    # cmod sql
    # cmod -l 30 sql
    ```
- Options:
    ```shell
  -l, --limit <LIMIT>  [default: 25]
  -o, --old            Print Installed Packages
  -h, --help           Print help
  -V, --version        Print version
    ```

## 安装

- linux
  将可执行文件放到 /usr/local/bin 下