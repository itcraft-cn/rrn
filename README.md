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
rrn     a rename file / directory tool.

        -f <pattern>, necessary: true
                from pattern
        -t <pattern>, necessary: true
                to pattern
        -d, optional, default: none
                rename directories or files, default is rename files.
        -x, optional, default: dry run
                execution the rename process
        -h, optional, default: none
                output help message
------------------------------------------------------
```

## Change Log

[change log](ChangeLog.md)
