# rrn: regex-repalce-name

[Chinese version](README_cn.md)

batch rename files/dirs by regex.

Tribute to [f2](https://github.com/ayoisaiah/f2/). 

Currently, it has an issue [#120](https://github.com/ayoisaiah/f2/issues/120) which it cannot replace filenames or folders containing spaces. 

However, this is a common usage scenario. As a temporary solution, I've created a small tool to address this. 

It's just a simple toy.

## Usage

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

## Example

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

## Change Log

[change log](ChangeLog.md)
