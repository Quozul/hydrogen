# Hydrogen
Hydrogen `/ˈhaɪ.drə.dʒən/` is a simple static site generator but powerful just like the hydrogen element.  
The aim of the project is to make a fast and easy to use site generator with batteries included.
You can find a list of features below.

## Documentation
Documentation is available at [hydrogen.quozul.dev](https://hydrogen.quozul.dev/).  
The website is made using this framework and deployed on GitHub Pages.
You can view the [GitHub Action here](/.github/workflows/gh-pages.yml).

## Installation
Install using `cargo`:
```shell
cargo install --git https://github.com/Quozul/hydrogen.git
```

## Usage
Here is a example command to get you started:

```shell
hydrogen build --input docs
```

## Features
Checked features are implemented, others are planned in order of priority.

- [x] Templating using Handlebars
- [x] Page creation in markdown or HTML
- [x] Simple CLI usage
- [x] Looping over all directories from a template (to improve)
- [x] SCSS/SASS support
- [x] Custom Handlebars' helpers written in [rhai](https://rhai.rs/)
- [ ] Automatic HTML meta tags generation in `<head>` inspired by [jekyll-seo-tag](https://github.com/jekyll/jekyll-seo-tag)
- [ ] Assets optimization
  - [x] HTML and CSS minification (to improve)
  - [ ] Rename assets using a hash for better caching
  - [ ] Image compression
  - [ ] Remove unused classes from CSS
- [ ] Docker images
- [ ] Use Handlebars' partials in markdown pages
- [ ] Integrated web server
  - [ ] Development server with hot reload
  - [ ] Low footprint production-like server
- [ ] Building performance (only if it gets too slow on single thread)
  - [ ] Incremental building
  - [ ] Multithreaded building
- [ ] Everything the future holds…
