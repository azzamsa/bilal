## Contributing

### Running The Demo

The demo is shown using [doitlive](https://github.com/sloria/doitlive).
Set the window dimension to `x: 638 y: 230 w: 780 h: 557`, then run:

``` shell
doitlive play docs/demo.sh
```


### Release

After the release has been pushed, run the code below to produce
pre-compiled binaries. Then upload them to the release page.

``` bash
$ rm -rf target
$ source docs/dev/relase.sh
```

or if you use `fish`

``` fish
❯ rm -rf target
❯ bash -c "source docs/dev/release.sh"
```
