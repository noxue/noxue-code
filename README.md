
## 功能

* 实现在线编译的接口

* 依赖这个项目实现 <https://github.com/noxue/noxue-compiler>

## 对应的前端界面仓库

<https://github.com/noxue/noxue-code-ui>


## 部署教程

<https://blog.noxue.com/tutorial/4e1aaedab55f43de89b20a7c26aef019.html>

## 需要安装的docker镜像

```shell
docker pull gcc
docker pull rust
docker pull php:5.6
docker pull php:7.4
docker pull php:8
docker pull golang
docker pull python:2
docker pull python:3
docker pull adityai/jdk13
docker pull ruby
docker pull node
docker pull swift
docker pull huntprod/asm
```



## 设计说明

本来写好`noxue-compiler` 这个工具包之后，可以很自然而然的想到，不同语言组装不同的命令调用下函数即可，最初版本也是这么做的，于是就有很多if else if的分支，非常糟糕。

如果不懂 rust 那不是无法部署这个程序了

于是经过一番修改，目前达到的效果是，不懂rust，甚至不懂编程都可以配置支持自己想支持的语言。


### 配置方式

添加一个语言只需要添加一个如下格式的json文件，无需修改任何其他的代码:

```json
{
    "image": "python:2",        // 使用哪个docker镜像
    "file": "test.py",          // 代码保存在哪个文件
    "cmd": "python test.py",    // 运行代码的命令
    "timeout": 5,               // 运行超时时间(单位秒)，超过这个时间就自动结束
    "memory": "100MB",          // 分配给容器的内存大小
    "cpuset":"0-3"              // 允许容器使用的cpu核心，0-3表示可以使用 0,1,2,3 这四个cpu，我是8核限制他最多使用一半
}
```

来看个复杂一点的配置，这是java的：
```json
{
    "image": "adityai/jdk13",
    "file": "regex::class\\s+([a-zA-Z_][a-zA-Z0-9_]*)::java",
    "cmd": "javac ./$cap1.java\njava $cap1\n",
    "timeout": 5,
    "memory": "100MB",
    "cpuset": "0-3"
}
```
他复杂的原因是，java类必须和文件名一样，这就意味着我们的文件名不能写死了，文件名要根据提交的代码来分析才能确定。

所以我通过 `regex::正则表达式::文件后缀` 这样的格式来自定义文件名，正则表达式从提交代码中去提取。

在cmd这个字段中 `javac ./$cap1.java\njava $cap1\n` 其中的 $cap1 就是上面正则表达式中括号捕获的第一个值，如果有多个就会是 $cap2 $cap3 以此类推

比如： `javac ./$cap1.java\njava $cap1\n` 命令，如果java类名是`Test`，那么通过正则匹配会得到 `Test` 这个值，
程序就会设置`cap1=Test && ext=java` 于是命令就变成了`javac ./Test.java\njava Test\n`



## 原理说明

1. 读取配置文件，分析出文件名，编译命令，限制资源的参数，拼接成最终的 shell命令

