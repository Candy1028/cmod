# cmod

通过 rust 实现的go package辅助工具

从 https://pkg.go.dev/ 搜索并获取 package 信息

## 命令

- 运行
    ```shell
        cmod <packageName>
    ```
- -l -limit
    ```shell
      cmod -l <limit> <packageName>
      # 或
      cmod -limit <limit> <packageName>
    ```

## 安装

- linux
  将可执行文件放到 /usr/local/bin 下