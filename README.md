# Ica
ICA is a terminal application for scraping grocery stores websites for extra prices and presents them in a readable fashion. All written in Rust.

## Features


## Installation
Build from scratch by cloning or install with cargo:
``` 
$ cargo install ica
```

To use ica you need a config file located in ```{your home directory}/.config/ica/config.json``` .
An example config file can look like thus: 
```
{
  "word_list" : [
    "a list of all the words you want to search for",
    "For example:",
    "pizza",
    "pasta"
  ],
  "urls" : [
     "The urls of the stores you want to look through",
     "example:",
     "https://www.ica.se/butiker/kvantum/lulea/ica-kvantum-stormarknad-lulea-71/erbjudanden/"
  ]
}
```
The config may consist of one or more urls, these can later be chosen from when running.

## Usage

``` ica ```   Will select one of the urls from the config file and match deals towards the wordlist and print them.

``` ica -a ``` Will print out all the deals from one of the urls from the config file.

``` ica -c <path to config file> ``` Will use config file from the specified file.

``` ica -u <url> ``` Instead of reading a url from a config file ica will use the specified url.

## Support
link mail or something here

## Planned features
- [x] Port to crossterm
- [ ] Arg for creating a config file
- [ ] Add support for other stores than ICA
- [ ] Finish -d arg option
- [ ] Make sure -a works
- [ ] Introduce tests for support functions
- [ ] Decide on a license lmao

## License
TODO
[MIT](https://choosealicense.com/licenses/mit/)
