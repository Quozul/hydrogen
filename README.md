# Hydrogen
Hydrogen `/ˈhaɪ.drə.dʒən/`, static site generator simple but powerful just like the hydrogen element.

The aim of the project is to make a fast and easy to use with batteries included site generator.

## Installation
1. Clone the repository
   ```shell
   git clone https://github.com/Quozul/hydrogen.git
   ```
2. Install using `cargo`
   ```shell
   cargo install --path .
   ```

## Usage
Here is a example command to get you started:

```shell
hydrogen build --input data
```

## Features
Checked features are implemented, others are planned.

- [x] Templating using handlebars
- [x] Page creation in markdown
- [x] Simple CLI usage
- [x] Looping over all directories from a template
- [x] SCSS/SASS support
- [ ] Automatic HTML meta tags generation in `<head>`
- [ ] Assets optimization
  - [x] HTML and CSS minification
  - [ ] Image compression
  - [ ] Remove unused classes from CSS
- [ ] Integrated web server
  - [ ] Development server with hot reload
  - [ ] Low footprint production-like server
- [ ] Docker images
- [ ] Custom helpers written in [rhai](https://rhai.rs/)
- [ ] Use Handlebars' partials in markdown pages
- [ ] Building performance (only if it gets too slow on single thread)
  - [ ] Incremental building
  - [ ] Multithreaded building
- [ ] Everything the future holds…
