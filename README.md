# rmdt.nvim

Markdown table plugin for neovim written by rust.

## Use

Using lazy.nvim <https://github.com/folke/lazy.nvim>

```lua
{
    "lhybdv/rmdt.nvim",
    ft = 'markdown',
    keys = {
      { "<leader>tf", "<Cmd>RFormatTable<CR>", desc = "Format markdown table" },
      { "<leader>ts", "<Cmd>RColumnSwap<CR>", desc = "Markdown table column swap" },
    },
    config = true,
    build = "make mac_build_so"
},
```

## Dependencies

[nvim-oxi](https://github.com/noib3/nvim-oxi)

## Limitation

Currently only support macos, neovim-nightly
