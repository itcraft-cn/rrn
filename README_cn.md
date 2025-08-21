# rrn: 正则改名

[English version](README.md)

通过正则表达式批量修改文件名/文件夹名。

致敬 [f2](https://github.com/ayoisaiah/f2/)。

当前其存在一个问题[#120](https://github.com/ayoisaiah/f2/issues/120)，不能替换名字带空格的文件名/文件夹。

但这又是一个常见的使用场景。所以临时做了一个小工具，先顶上去。

就是一个简单的玩具。

## 使用说明

```console
$ rrn
------------------------------------------------------
rrn	一个文件/目录重命名工具。

	-f <pattern>, 必须指定
		匹配格式
	-t <pattern>, 必须指定
		目标格式
	-d, 可选, 默认: 不指定
		是否重命名目录, 默认模式为重命名文件。
	-x, 可选, 默认: 空跑
		执行重命名操作。
	-h, 输出帮助信息
	-v, 输出版本信息
------------------------------------------------------
```

## 使用样例

```console
$ ls
acca.txt  accca.txt  acccca.txt  a.txt

$ rrn -f a -t b
----------------------------------
| 匹配格式   | 目标格式   | 状态 |
----------------------------------
| a.txt      | b.txt      | 可用 |
| acccca.txt | bccccb.txt | 可用 |
| accca.txt  | bcccb.txt  | 可用 |
| acca.txt   | bccb.txt   | 可用 |
----------------------------------
这是空跑模式, 使用 '-x' 执行重命名操作。

$ rrn -f a -t b -x
重命名 "a.txt" => "b.txt"
重命名 "acccca.txt" => "bccccb.txt"
重命名 "accca.txt" => "bcccb.txt"
重命名 "acca.txt" => "bccb.txt"
```

## 变更记录

[变更记录](ChangeLog_cn.md)
