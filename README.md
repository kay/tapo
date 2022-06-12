# Tapo

[![Crates][crates_badge]][crates]
[![CI][ci_badge]][ci]
[![license][license_badge]][license]
[![Crates.io][crates_installs_badge]][crates]\
Unofficial Tapo API Client.

## Device support

| Feature               |    L530 |    L510 |    P110 |    P100 | GenericDevice |
| --------------------- | ------: | ------: | ------: | ------: | ------------: |
| on                    | &check; | &check; | &check; | &check; |       &check; |
| off                   | &check; | &check; | &check; | &check; |       &check; |
| get_device_info       | &check; | &check; | &check; | &check; |       &check; |
| get_device_usage      | &check; | &check; | &check; | &check; |       &check; |
| get_energy_usage      |         |         | &check; |         |               |
| set_brightness        | &check; | &check; |         |         |               |
| set_color             | &check; |         |         |         |               |
| set_hue_saturation    | &check; |         |         |         |               |
| set_color_temperature | &check; |         |         |         |               |

## Examples

```bash
export IP_ADDRESS=
export TAPO_USERNAME=
export TAPO_PASSWORD=

cargo run --example tapo_l530
```

See all examples in [/examples][examples].

## Contributing

Contributions are welcome and encouraged! See [/CONTRIBUTING.md][contributing].

## Credits

Inspired by [petretiandrea/plugp100][inspired_by].

[crates_badge]: https://img.shields.io/crates/v/tapo.svg
[crates]: https://crates.io/crates/tapo
[ci_badge]: https://github.com/mihai-dinculescu/tapo/workflows/CI/badge.svg?branch=main
[ci]: https://github.com/mihai-dinculescu/tapo/actions
[license_badge]: https://img.shields.io/crates/l/tapo.svg
[license]: https://github.com/mihai-dinculescu/tapo/blob/main/LICENSE
[crates_installs_badge]: https://img.shields.io/crates/d/tapo?label=cargo%20installs
[examples]: https://github.com/mihai-dinculescu/tapo/tree/main/examples
[contributing]: https://github.com/mihai-dinculescu/tapo/blob/main/CONTRIBUTING.md
[inspired_by]: https://github.com/petretiandrea/plugp100
