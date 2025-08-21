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
rrn	a rename file / directory tool.

	-f <pattern>, necessary: true
		from pattern
	-t <pattern>, necessary: true
		to pattern
	-d, optional, default: none
		rename directories or files, default is rename files.
	-x, optional, default: dry run
		execution the rename process
	-h, output help message
	-v, output version info
------------------------------------------------------ 
```

## 使用样例

```console
$ ls
acca.txt  accca.txt  acccca.txt  a.txt

$ rrn -f a -t b
------------------------------------
| from       | to         | status |
------------------------------------
| a.txt      | b.txt      | OK     |
| acccca.txt | bccccb.txt | OK     |
| accca.txt  | bcccb.txt  | OK     |
| acca.txt   | bccb.txt   | OK     |
------------------------------------
This is dryrun. Execute with '-x' to execute.

$ rrn -f a -t b -x
Move "a.txt" => "b.txt"
Move "acccca.txt" => "bccccb.txt"
Move "accca.txt" => "bcccb.txt"
Move "acca.txt" => "bccb.txt" 
```

## 变更记录

[变更记录](ChangeLog_cn.md)
