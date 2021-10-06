<br/>
<p align="center">
  <a href="https://github.com/bitton-yehonatan/log2graph">
    <img src="https://cdn-icons-png.flaticon.com/512/2165/2165676.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Log2Graph</h3>

  <p align="center">
    A LogParser to graphviz dot format
    <br/>
    <br/>
    <a href="https://github.com/bitton-yehonatan/log2graph"><strong>Explore the docs »</strong></a>
    <br/>
    <br/>
    <a href="https://github.com/bitton-yehonatan/log2graph/issues">Report Bug</a>
    .
    <a href="https://github.com/bitton-yehonatan/log2graph/issues">Request Feature</a>
  </p>
</p>

![License](https://img.shields.io/github/license/bitton-yehonatan/log2graph) 

## Table Of Contents

- [Table Of Contents](#table-of-contents)
- [About The Project](#about-the-project)
- [Built With](#built-with)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
- [Usage](#usage)
- [Contributing](#contributing)
  - [Creating A Pull Request](#creating-a-pull-request)
- [License](#license)

## About The Project

Log2Graph is a CLI tool that transforms log files to Graphviz dot format for better inspection.

## Built With

Built with rust petgraph library

## Getting Started

Running log2graph is very easy you only need to install the supplied create and start using.

### Prerequisites

Make sure that rust is installed on your machine

## Usage

tldr;

`log2graph --pattern "logprefix" --group_by "group_key" my_log.log --keys_to_print "group_key" "another_key"`


The long way

USAGE:
    log2graph [FLAGS] [OPTIONS] <path> --group_by <group-by> --pattern <pattern>

FLAGS:
    -h, --help           Prints help information
    -s, --split_files    split dot file by group
    -V, --version        Prints version information

OPTIONS:
    -g, --group_by <group-by>                    The key for grouping logs
    -k, --keys_delimiter <keys-delimiter>        The delimiter for each key [default: :]
    -v, --keys_to_print <keys-to-print>...       Keys for node labling
    -d, --params_delimiter <params-delimiter>    The delimiter for each param [default: ,]
    -p, --pattern <pattern>                      The pattern for parsed lines

ARGS:
    <path>    The path to the file to read

## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.
* If you have suggestions for adding or removing projects, feel free to [open an issue](https://github.com/bitton-yehonatan/log2graph/issues/new) to discuss it, or directly create a pull request after you edit the *README.md* file with necessary changes.
* Please make sure you check your spelling and grammar.
* Create individual PR for each suggestion.
* Please also read through the [Code Of Conduct](https://github.com/bitton-yehonatan/log2graph/blob/main/CODE_OF_CONDUCT.md) before posting your first idea as well.

### Creating A Pull Request

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See [LICENSE](https://github.com/bitton-yehonatan/log2graph/blob/main/LICENSE.md) for more information.
