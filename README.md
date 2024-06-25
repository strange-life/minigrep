# 文件搜索工具

参数列表

- -p / --path 指定文件路径
- -q / --query 指定搜索关键词
- -i / --ignore_case 忽略大小写

环境变量列表

- IGNORE_CASE 忽略大小写

调试示例

```shell
cargo run -- -p poem.txt -q to -i
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.76s
     Running `target\debug\minigrep.exe -p poem.txt -q to -i`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

```shell
cargo run -- -p poem.txt -q to
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\minigrep.exe -p poem.txt -q to`
Are you nobody, too?
How dreary to be somebody!
```
