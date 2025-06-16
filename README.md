# PS1

Generate configurable colorful `PS1` prompt generator for bash


## Usage

### 1. Create a `~/.config/ps1.toml` with the following content:

```toml
ps1='{220}\u{137}:{220}{cwd:name}{37}{git:branch}{220}${reset}'
```

### 2. Add the following snippet to your `.bashrc`:

```shell
eval "$(~/.cargo/bin/ps1 --env)"
```

## Syntax

### ANSI Colors

set foreground color
> `{fg:number}` or `{number}` where *number* is 0 - 255 -

set background color
> `{bg:number}` where *number* is 0 - 255 -

reset colors
> `{reset}`


### Current Work Dir Name

`{cwd:name}`


### Git Branch

`{git:branch}`


## Demo

[![asciicast](https://asciinema.org/a/vZiqWFL4GshfqAzZcrmhlPUAx.svg)](https://asciinema.org/a/vZiqWFL4GshfqAzZcrmhlPUAx)


## How it works

Formal parsing of [PS1 escape
sequences](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html)
and `{custom:params}` is done through the PEG grammar is defined at
[src/grammar.pest](https://github.com/gabrielfalcao/ps1g/tree/HEAD/src/grammar.pest)
