<h1 align="center">
    simple_csv_reader
</h1> 

<!-- [<img alt="github" src="https://img.shields.io/badge/github-uraneko.ragout-A5915F?style=for-the-badge&logo=github&labelColor=3a3a3a" height="25">](https://github.com/uraneko/ragout)  -->
<!-- [<img alt="crates.io" src="https://img.shields.io/crates/v/ragout.svg?style=for-the-badge&color=E40046&logo=rust&labelColor=3a3a3a" height="25">](https://crates.io/crates/ragout)  -->
<!-- [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-ragout-495c9f?style=for-the-badge&logo=docsdotrs&labelColor=3a3a3a" height="25">](https://docs.rs/ragout)  -->
<!-- [<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/uraneko/ragout/rust.yml?branch=main&style=for-the-badge&labelColor=3a3a3a" height="25">](https://github.com/uraneko/ragout/actions?query=branch%3Amain) -->
<!-- [<img alt="license" src="https://img.shields.io/github/license/uraneko/ragout?style=for-the-badge&labelColor=3a3a3a&color=ECD53F" height="25">](https://github.com/uraneko/ragout/blob/main/LICENSE) -->

<h3>
    A Simple CSV File Reader
</h3>
 
As the name says, this is a simple csv reader. All it does is take a csv file as a command argument and parse its contents into a table gui. 

> [!NOTE] 
> It's a bit slow for medium sized files (1000 rows and more). 
> almost unusable for bigger files (10000+ rows), the files would open normally but the gui would be too laggy.

## Features

✓ gui display: show the title and contents of the csv file in a gui table

✓ copy: you can copy the contents from table cells into the clipboard

### Installation

```bash
git clone https://github.com/uraneko/simple_csv_reader
cd simple_csv_reader
cargo build -r --locked 
```


## Version
This crate follows the [SemVer Spec](https://semver.org/) versioning scheme.

<br>
