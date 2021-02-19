# CaBr2

[![License: GPL3+](https://img.shields.io/badge/License-GPL3+-blue.svg)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![GH Actions](https://action-badges.now.sh/Calciumdibromid/CaBr2)](https://github.com/Calciumdibromid/CaBr2/actions)

Generate "experiment wise safety sheets" in compliance to European law.

## Develop

CaBr2 is a [tauri](https://tauri.studio) project.

Frontend is written in Angular 11 and backend is in Rust.

To start execute `yarn start` to build and serve UI,
execute `yarn tauri-start` to get the aplication window.

## Build

You need the tauri-builder. Get it  via `cargo install -f tauri-bundler`.

To convert the html template to a pdf file you need to install `wkhtmltopdf`.
`wkhtmltox` (that can be downloaded from [here](https://wkhtmltopdf.org/downloads.html)) is needed to build the app.

Just execute `yarn build && yarn tauri build` afterwards.
