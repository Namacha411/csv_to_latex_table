# csv_to_latex_table

this program read csv file, and write latex table.

# How to use

a csv file is read from stdin, and then, latex table is witten to stdout.

```shell
# csv read example
$echo '
a, b, c
d, e, f
g, h, i
' | csv_to_latextable -c='|c|c|c|'
```

```shell
# output examle
\begin{table}[htb]
        \begin{tabular}{|c|c|c|}
                a & b & c \\ \hline
                d & e & f \\
                g & h & i \\
        \begin{tabular}
\end{table}
```

command options are these.

```shell
$csv_to_latexteble --help
csv2tbl 0.1.0

USAGE:
    csv_to_latexteble.exe [FLAGS] [OPTIONS] --column <column>

FLAGS:
        --caption-is-under
        --help                Prints help information
    -V, --version             Prints version information

OPTIONS:
    -C, --caption <caption>
    -c, --column <column>
    -h, --hline <hline>         [default: 0]
    -l, --label <label>
    -t, --table <table>         [default: htb]
```
