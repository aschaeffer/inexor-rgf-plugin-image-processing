# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module       | Functionality                                                        | Tests                                                                                                                                                                         |
|---------------------|--------|------------------|----------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | Image Processing | <img src="https://img.shields.io/badge/state-inprogress-yellow">     | [<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-image-processing">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-image-processing)  |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-image-processing/main/docs/images/inexor_2.png">
</a>

* Inexor will be a new first-person shooter game which is based on a new octree-based game engine.
* Inexor focuses on classic gameplay as we've seen in Cube2 or the Quake series.
* Inexor will be written from ground up new in C++17 and Rust.
* You can contribute anything you want: code, content, ideas..
* Inexor and all its content is 100% open source!

### About Inexor Reactive Graph Flow

The Inexor Reactive Graph Flow (RGF) manages reactive flows based on a graph database. The main interface is GraphQL.

* Semantic: Graph database with entities and relationships as first class citizens
* Reactive: entities and relationships are/can be reactive: If the input has been altered the entity processes its new state
* Interoperable: Use GraphQL for queries and mutations
* Extendable: Built in type system: components, entity types and relation types
* Memory efficient: Rust
* Fast: Rust
* Secure: Rust

### About this plugin

This plugin provides functionality for processing images like rotation, scale, blending, ...

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/aschaeffer/inexor-rgf-plugin-image-processing/Rust">](https://github.com/aschaeffer/inexor-rgf-plugin-image-processing/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/aschaeffer/inexor-rgf-plugin-image-processing">]()
[<img src="https://img.shields.io/github/languages/code-size/aschaeffer/inexor-rgf-plugin-image-processing">]()
[<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-image-processing">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-image-processing)

[<img src="https://img.shields.io/github/license/aschaeffer/inexor-rgf-plugin-image-processing">](https://github.com/aschaeffer/inexor-rgf-plugin-image-processing/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Entity Types

| Name          | Property          | Data Type | Socket Type |     |
|---------------|-------------------|-----------|-------------|-----|
| GenerateNoise | trigger           | bool      | input       |     |
|               | data_url          | string    | output      |     |
| RotateImage   | source_data_url   | string    | input       | TBD |
|               | rotate            | number    | input       |     |
|               | result_data_url   | string    | output      |     |
| BlendImages   | source_1_data_url | string    | input       | TBD |
|               | source_2_data_url | string    | input       |     |
|               | result_data_url   | string    | output      |     |

#### Generate Noise

<img width="500" height="500" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-image-processing/main/docs/images/generate_noise.png">

### TODO

* Implement entity behaviours for these [image processing functions](https://github.com/image-rs/image#image-processing-functions):
  * blur: Performs a Gaussian blur on the supplied image.
  * brighten: Brighten the supplied image. 
  * huerotate: Hue rotate the supplied image by degrees. 
  * contrast: Adjust the contrast of the supplied image. 
  * crop: Return a mutable view into an image. 
  * filter3x3: Perform a 3x3 box filter on the supplied image. 
  * flip_horizontal: Flip an image horizontally. 
  * flip_vertical: Flip an image vertically. 
  * grayscale: Convert the supplied image to grayscale. 
  * invert: Invert each pixel within the supplied image This function operates in place. 
  * resize: Resize the supplied image to the specified dimensions. 
  * rotate180: Rotate an image 180 degrees clockwise. 
  * rotate270: Rotate an image 270 degrees clockwise. 
  * rotate90: Rotate an image 90 degrees clockwise. 
  * unsharpen: Performs an unsharpen mask on the supplied image.

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                               |           |                                                                   |
|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-logical/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
