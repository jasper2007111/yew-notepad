# 记事本
主要是为学习如何用Rust写前端而创建的，框架用的是[Yew](https://yew.rs)，存储使用[IndexedDB](https://developer.mozilla.org/zh-CN/docs/Web/API/IndexedDB_API)。目前实现了基本的数据存储以及显示功能，难点部分基本已经解决，目前这方便的资料比较少，希望大家不要重复踩坑，相互学习。

Yew的中文文档跟英文有出入，建议以英文的为主。

### 运行

```sh
trunk serve
```

默认是127.0.0.1:8080，这个是无法在局域网内使用本机IP访问的，可以加参数修改，如下

```sh
trunk serve --address=0.0.0.0
```

![Notepad](./screenshots.png)

### 部署
运行`trunk server`后，会在当前目录下生dist的目录，部署的时候不能使用这个，因为这个里面加了调适的东西。部署的话应该使用`trunk build`命令产生的dist目录内容。
