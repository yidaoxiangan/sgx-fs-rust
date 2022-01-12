
[Understanding SGX Protected File System](https://www.tatetian.io/2017/01/15/understanding-sgx-protected-file-system)

[已经移植好的库](https://github.com/mesalock-linux/crates-sgx)

[sgx中对libc封装的Rust库](https://github.com/apache/incubator-teaclave-sgx-sdk/tree/master/sgx_libc)

昨天开会提到的 Rust SGX SDK 是这个： https://github.com/apache/incubator-teaclave-sgx-sdk 里面文档有说明如何使用 simulation 模式运行 Intel SGX 应用。
SGX 快速入门可以参考 https://sgx101.gitbook.io/sgx101/

## 开发环境和测试环境

### 开发环境
+ 启动命令：
```sh
docker run \
-v /root/dev/incubator-teaclave-sgx-sdk:/root/sgx \
-v /root/dev/sgx-rust:/root/dev/sgx-fs \
-v /root/dev/sgx_tprotected_fs:/root/dev/sgx_tprotected_fs \
-itd baiduxlab/sgx-rust /bin/bash
```
+ 进入容器：
  + `docker exec -it d3e5 /bin/bash`
+ 数据挂载：
  + `/root/dev/incubator-teaclave-sgx-sdk:/root/sgx` 使用默认版本的sdk。 测试文件也在`samplecode`目录中编写。
  + `/root/dev/sgx-rust:/root/dev/sgx-fs`
  + `/root/dev/sgx_tprotected_fs:/root/dev/sgx_tprotected_fs` 


#### 测试流程
直接拉取最新版本的sdk。

`git clone https://github.com/apache/incubator-teaclave-sgx-sdk.git`

```
cd incubator-teaclave-sgx-sdk/samplecode/helloworld
make SGX_MODE=SW
cd bin
./app
```


