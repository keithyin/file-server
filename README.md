基于http的文件下载服务器。

在某个目录下执行执行该程序，该目录机器子目录的内容都可以通过http下载

运行方式
```shell

nohup ./simple-file-download-server --ip 192.168.3.44 --port 10010 --serve_dir /data/PUBLIC_RESOURCES > nohup.log 2>&1 &
```