# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [v2.3.7](https://codeberg.org/PurpleBooth/git-mit/compare/197f5b9e2eae8a1292eacc1188590a8fb8ac14bd..v2.3.7) - 2024-11-06
#### Bug Fixes
- **(deps)** update rust crate thiserror to v2 - ([3a6e438](https://codeberg.org/PurpleBooth/git-mit/commit/3a6e438768f6cd1546b0c2bf46579b8cfa87213e)) - Solace System Renovate Fox
- **(deps)** update rust crate clap_complete to v4.5.37 - ([93052af](https://codeberg.org/PurpleBooth/git-mit/commit/93052af9091cdf9dd716c8b41cbc108ad1b3b9ee)) - Solace System Renovate Fox
- **(deps)** update rust crate clap to v4.5.20 - ([4e5c82f](https://codeberg.org/PurpleBooth/git-mit/commit/4e5c82f0031b27bd0b96daf3f45841520ac1b191)) - Solace System Renovate Fox
- **(deps)** update rust crate thiserror to v1.0.68 - ([37a2441](https://codeberg.org/PurpleBooth/git-mit/commit/37a2441705979f6517c09ad83cb781935ae957b4)) - Solace System Renovate Fox
- **(deps)** update rust crate serde_yaml to v0.9.34 - ([f5696d2](https://codeberg.org/PurpleBooth/git-mit/commit/f5696d20d7180e79e662e58a13c606f0a1bfffe6)) - Solace System Renovate Fox
- **(deps)** update rust crate serde to v1.0.214 - ([2df56fc](https://codeberg.org/PurpleBooth/git-mit/commit/2df56fc0dbc637112d63c3ca0c8b9490dec23056)) - Solace System Renovate Fox
- **(deps)** update rust crate inquire to v0.7.5 - ([847a88f](https://codeberg.org/PurpleBooth/git-mit/commit/847a88f88734bf66412362053627c9efc673255d)) - Solace System Renovate Fox
- **(deps)** update rust crate inquire to 0.7.2 - ([62c8c1f](https://codeberg.org/PurpleBooth/git-mit/commit/62c8c1f11bab8304e705b1a7b89a7cac98beee2d)) - Solace System Renovate Fox
- **(deps)** update rust crate serde_yaml to 0.9.33 - ([84f1191](https://codeberg.org/PurpleBooth/git-mit/commit/84f1191c7cd550907352defb74984ab6684bcf2e)) - Solace System Renovate Fox
#### Build system
- Set linkers via the pipeline - ([d2751ca](https://codeberg.org/PurpleBooth/git-mit/commit/d2751ca1e877cd911ac83f5dce71835b0a7117b7)) - Billie Thompson
- Specify linker via environment variable - ([1b727a2](https://codeberg.org/PurpleBooth/git-mit/commit/1b727a2a6e414e31fe6b4e7d2ae2784ef8a212be)) - Billie Thompson
- Correct tripple for linux arm - ([99b6f9d](https://codeberg.org/PurpleBooth/git-mit/commit/99b6f9d82b11ff5f6e9b3f46978ac86c1443df8b)) - Billie Thompson
- Specify windows msvc compiler - ([9d2ee5c](https://codeberg.org/PurpleBooth/git-mit/commit/9d2ee5cf448b11f3a01697fd968ea7ef86e486cb)) - Billie Thompson
- Add a dev container - ([474f53c](https://codeberg.org/PurpleBooth/git-mit/commit/474f53cd5e0f8ba544ac874561c8804bfaf55352)) - Billie Thompson
- Specify gcc linker for aarch64 - ([d15fe3f](https://codeberg.org/PurpleBooth/git-mit/commit/d15fe3fc7b89c5402a83c1c7190127d275f2dcce)) - Billie Thompson
- vendor openssl and libgit2 - ([6341383](https://codeberg.org/PurpleBooth/git-mit/commit/6341383b9b312567af84aef18058513ef9dd7e61)) - Billie Thompson
- Vendor openssl when cross comiling - ([a04f934](https://codeberg.org/PurpleBooth/git-mit/commit/a04f9343e53743ca901b69d1f7c2f601d9b985da)) - Billie Thompson
- Add a temporary dev container - ([89b2d8a](https://codeberg.org/PurpleBooth/git-mit/commit/89b2d8a224c09fb671957014d9b9e7791a8fb92d)) - Billie Thompson
#### Continuous Integration
- Use a PAT rather than permissions - ([349a8eb](https://codeberg.org/PurpleBooth/git-mit/commit/349a8eb70494878ba5c7bd383a9f193872b1ed41)) - Billie Thompson
- Use common rust install step - ([9795c46](https://codeberg.org/PurpleBooth/git-mit/commit/9795c466f9f5eb4243818d113695fcdc00ee1d09)) - Billie Thompson
- set gh token for release action - ([7e59e88](https://codeberg.org/PurpleBooth/git-mit/commit/7e59e881aead4b98c6a843e01a7ce32de6578fc2)) - Billie Thompson
- Pass GH Token to workflow - ([535f0ed](https://codeberg.org/PurpleBooth/git-mit/commit/535f0ed050d5c0f02210f81b235407816b93c1e8)) - Billie Thompson
- Correct target host - ([69512cf](https://codeberg.org/PurpleBooth/git-mit/commit/69512cfd3a81a03faa45395077972b5d9718dba6)) - Billie Thompson
- Try a different release mechanism - ([649ea26](https://codeberg.org/PurpleBooth/git-mit/commit/649ea261ea58567c88e2d2a646a04f6e6ed18e39)) - Billie Thompson
- Use central config - ([dab0be3](https://codeberg.org/PurpleBooth/git-mit/commit/dab0be3dc1be7706b994c49e3ee39b0e27c5f9cd)) - Billie Thompson
- Remove trailing colon - ([d36115d](https://codeberg.org/PurpleBooth/git-mit/commit/d36115d632be188cdbd1f29b01e8a296ae0c4685)) - Billie Thompson
- Format justfile - ([e5b7876](https://codeberg.org/PurpleBooth/git-mit/commit/e5b78763b058fb51ac0dcdff9a466601d4f51093)) - Billie Thompson
- Add mingw i686 - ([b3ac161](https://codeberg.org/PurpleBooth/git-mit/commit/b3ac161fc62f6296f2bf3ad15b08df5a2b2eff26)) - Billie Thompson
- Add a build into the lint test pipeline - ([a912fd3](https://codeberg.org/PurpleBooth/git-mit/commit/a912fd3e86fe2e2adac05b2ab7550cff3384af24)) - Billie Thompson
- Add aarch64 gnu linux compiler - ([be20ff7](https://codeberg.org/PurpleBooth/git-mit/commit/be20ff76921427d5b5a646077995f64c7f4a6311)) - Billie Thompson
- Add libssl to the list of installed deps - ([f79a6c8](https://codeberg.org/PurpleBooth/git-mit/commit/f79a6c86df50d5457a3b60499979427eaceefbf0)) - Billie Thompson
- Add targets to try to build for - ([ebc4613](https://codeberg.org/PurpleBooth/git-mit/commit/ebc4613ce188faae38f02c02d31cda64fef6cf88)) - Billie Thompson
- Pull whole repo to release - ([197f5b9](https://codeberg.org/PurpleBooth/git-mit/commit/197f5b9e2eae8a1292eacc1188590a8fb8ac14bd)) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** pin dependencies - ([ce91afa](https://codeberg.org/PurpleBooth/git-mit/commit/ce91afa83979f3ffe4ef98ae5b799d3e5ad05ba6)) - Solace System Renovate Fox
- **(deps)** update ghcr.io/catthehacker/ubuntu:act-latest docker digest to 50da7b6 - ([f8ea8e4](https://codeberg.org/PurpleBooth/git-mit/commit/f8ea8e487a9b9c71cc09a8700730fea9759eadb8)) - Solace System Renovate Fox
- **(deps)** pin ghcr.io/catthehacker/ubuntu docker tag to f9e2268 - ([4d5cc42](https://codeberg.org/PurpleBooth/git-mit/commit/4d5cc42fbea51dbe4d7a51082b9b3e3ecfab9e18)) - Solace System Renovate Fox
- **(version)** v2.3.7 - ([34b248d](https://codeberg.org/PurpleBooth/git-mit/commit/34b248dfc28075d263d02a00d43737fde6b4e433)) - cog-bot
#### Refactoring
- Sort the lines in the dockerfile - ([5d08a96](https://codeberg.org/PurpleBooth/git-mit/commit/5d08a96c72daf930ba120f32d5f1d14d026adbda)) - Billie Thompson

- - -

## [v2.3.7](https://codeberg.org/PurpleBooth/git-mit/compare/197f5b9e2eae8a1292eacc1188590a8fb8ac14bd..v2.3.7) - 2024-11-06
#### Bug Fixes
- **(deps)** update rust crate thiserror to v2 - ([3a6e438](https://codeberg.org/PurpleBooth/git-mit/commit/3a6e438768f6cd1546b0c2bf46579b8cfa87213e)) - Solace System Renovate Fox
- **(deps)** update rust crate clap_complete to v4.5.37 - ([93052af](https://codeberg.org/PurpleBooth/git-mit/commit/93052af9091cdf9dd716c8b41cbc108ad1b3b9ee)) - Solace System Renovate Fox
- **(deps)** update rust crate clap to v4.5.20 - ([4e5c82f](https://codeberg.org/PurpleBooth/git-mit/commit/4e5c82f0031b27bd0b96daf3f45841520ac1b191)) - Solace System Renovate Fox
- **(deps)** update rust crate thiserror to v1.0.68 - ([37a2441](https://codeberg.org/PurpleBooth/git-mit/commit/37a2441705979f6517c09ad83cb781935ae957b4)) - Solace System Renovate Fox
- **(deps)** update rust crate serde_yaml to v0.9.34 - ([f5696d2](https://codeberg.org/PurpleBooth/git-mit/commit/f5696d20d7180e79e662e58a13c606f0a1bfffe6)) - Solace System Renovate Fox
- **(deps)** update rust crate serde to v1.0.214 - ([2df56fc](https://codeberg.org/PurpleBooth/git-mit/commit/2df56fc0dbc637112d63c3ca0c8b9490dec23056)) - Solace System Renovate Fox
- **(deps)** update rust crate inquire to v0.7.5 - ([847a88f](https://codeberg.org/PurpleBooth/git-mit/commit/847a88f88734bf66412362053627c9efc673255d)) - Solace System Renovate Fox
- **(deps)** update rust crate inquire to 0.7.2 - ([62c8c1f](https://codeberg.org/PurpleBooth/git-mit/commit/62c8c1f11bab8304e705b1a7b89a7cac98beee2d)) - Solace System Renovate Fox
- **(deps)** update rust crate serde_yaml to 0.9.33 - ([84f1191](https://codeberg.org/PurpleBooth/git-mit/commit/84f1191c7cd550907352defb74984ab6684bcf2e)) - Solace System Renovate Fox
#### Build system
- Set linkers via the pipeline - ([d2751ca](https://codeberg.org/PurpleBooth/git-mit/commit/d2751ca1e877cd911ac83f5dce71835b0a7117b7)) - Billie Thompson
- Specify linker via environment variable - ([1b727a2](https://codeberg.org/PurpleBooth/git-mit/commit/1b727a2a6e414e31fe6b4e7d2ae2784ef8a212be)) - Billie Thompson
- Correct tripple for linux arm - ([99b6f9d](https://codeberg.org/PurpleBooth/git-mit/commit/99b6f9d82b11ff5f6e9b3f46978ac86c1443df8b)) - Billie Thompson
- Specify windows msvc compiler - ([9d2ee5c](https://codeberg.org/PurpleBooth/git-mit/commit/9d2ee5cf448b11f3a01697fd968ea7ef86e486cb)) - Billie Thompson
- Add a dev container - ([474f53c](https://codeberg.org/PurpleBooth/git-mit/commit/474f53cd5e0f8ba544ac874561c8804bfaf55352)) - Billie Thompson
- Specify gcc linker for aarch64 - ([d15fe3f](https://codeberg.org/PurpleBooth/git-mit/commit/d15fe3fc7b89c5402a83c1c7190127d275f2dcce)) - Billie Thompson
- vendor openssl and libgit2 - ([6341383](https://codeberg.org/PurpleBooth/git-mit/commit/6341383b9b312567af84aef18058513ef9dd7e61)) - Billie Thompson
- Vendor openssl when cross comiling - ([a04f934](https://codeberg.org/PurpleBooth/git-mit/commit/a04f9343e53743ca901b69d1f7c2f601d9b985da)) - Billie Thompson
- Add a temporary dev container - ([89b2d8a](https://codeberg.org/PurpleBooth/git-mit/commit/89b2d8a224c09fb671957014d9b9e7791a8fb92d)) - Billie Thompson
#### Continuous Integration
- Use a PAT rather than permissions - ([349a8eb](https://codeberg.org/PurpleBooth/git-mit/commit/349a8eb70494878ba5c7bd383a9f193872b1ed41)) - Billie Thompson
- Use common rust install step - ([9795c46](https://codeberg.org/PurpleBooth/git-mit/commit/9795c466f9f5eb4243818d113695fcdc00ee1d09)) - Billie Thompson
- set gh token for release action - ([7e59e88](https://codeberg.org/PurpleBooth/git-mit/commit/7e59e881aead4b98c6a843e01a7ce32de6578fc2)) - Billie Thompson
- Pass GH Token to workflow - ([535f0ed](https://codeberg.org/PurpleBooth/git-mit/commit/535f0ed050d5c0f02210f81b235407816b93c1e8)) - Billie Thompson
- Correct target host - ([69512cf](https://codeberg.org/PurpleBooth/git-mit/commit/69512cfd3a81a03faa45395077972b5d9718dba6)) - Billie Thompson
- Try a different release mechanism - ([649ea26](https://codeberg.org/PurpleBooth/git-mit/commit/649ea261ea58567c88e2d2a646a04f6e6ed18e39)) - Billie Thompson
- Use central config - ([dab0be3](https://codeberg.org/PurpleBooth/git-mit/commit/dab0be3dc1be7706b994c49e3ee39b0e27c5f9cd)) - Billie Thompson
- Remove trailing colon - ([d36115d](https://codeberg.org/PurpleBooth/git-mit/commit/d36115d632be188cdbd1f29b01e8a296ae0c4685)) - Billie Thompson
- Format justfile - ([e5b7876](https://codeberg.org/PurpleBooth/git-mit/commit/e5b78763b058fb51ac0dcdff9a466601d4f51093)) - Billie Thompson
- Add mingw i686 - ([b3ac161](https://codeberg.org/PurpleBooth/git-mit/commit/b3ac161fc62f6296f2bf3ad15b08df5a2b2eff26)) - Billie Thompson
- Add a build into the lint test pipeline - ([a912fd3](https://codeberg.org/PurpleBooth/git-mit/commit/a912fd3e86fe2e2adac05b2ab7550cff3384af24)) - Billie Thompson
- Add aarch64 gnu linux compiler - ([be20ff7](https://codeberg.org/PurpleBooth/git-mit/commit/be20ff76921427d5b5a646077995f64c7f4a6311)) - Billie Thompson
- Add libssl to the list of installed deps - ([f79a6c8](https://codeberg.org/PurpleBooth/git-mit/commit/f79a6c86df50d5457a3b60499979427eaceefbf0)) - Billie Thompson
- Add targets to try to build for - ([ebc4613](https://codeberg.org/PurpleBooth/git-mit/commit/ebc4613ce188faae38f02c02d31cda64fef6cf88)) - Billie Thompson
- Pull whole repo to release - ([197f5b9](https://codeberg.org/PurpleBooth/git-mit/commit/197f5b9e2eae8a1292eacc1188590a8fb8ac14bd)) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** pin dependencies - ([ce91afa](https://codeberg.org/PurpleBooth/git-mit/commit/ce91afa83979f3ffe4ef98ae5b799d3e5ad05ba6)) - Solace System Renovate Fox
- **(deps)** update ghcr.io/catthehacker/ubuntu:act-latest docker digest to 50da7b6 - ([f8ea8e4](https://codeberg.org/PurpleBooth/git-mit/commit/f8ea8e487a9b9c71cc09a8700730fea9759eadb8)) - Solace System Renovate Fox
- **(deps)** pin ghcr.io/catthehacker/ubuntu docker tag to f9e2268 - ([4d5cc42](https://codeberg.org/PurpleBooth/git-mit/commit/4d5cc42fbea51dbe4d7a51082b9b3e3ecfab9e18)) - Solace System Renovate Fox
#### Refactoring
- Sort the lines in the dockerfile - ([5d08a96](https://codeberg.org/PurpleBooth/git-mit/commit/5d08a96c72daf930ba120f32d5f1d14d026adbda)) - Billie Thompson

- - -

## [v2.3.6](https://codeberg.org/PurpleBooth/git-mit/compare/v2.3.5..v2.3.6) - 2024-03-15
#### Bug Fixes
- **(deps)** update rust crate clap to 4.5.3 - ([f4c8e06](https://codeberg.org/PurpleBooth/git-mit/commit/f4c8e06f3f6481281ed65bcce0b385da46baa98d)) - Solace System Renovate Fox
#### Continuous Integration
- Set the git user before releasing - ([a83436b](https://codeberg.org/PurpleBooth/git-mit/commit/a83436b62768c4dee17dd0cc7cf3e9f255e9ad53)) - Billie Thompson

- - -

## [v2.3.5](https://codeberg.org/PurpleBooth/git-mit/compare/v2.3.4..v2.3.5) - 2024-03-15
#### Bug Fixes
- **(deps)** update rust crate miette to 7.1.0 - ([6d71e74](https://codeberg.org/PurpleBooth/git-mit/commit/6d71e741219b35c7e5bc9dcccb9104fce30a0e4b)) - Renovate Bot
- **(deps)** update rust crate inquire to 0.7.0 - ([7611fe8](https://codeberg.org/PurpleBooth/git-mit/commit/7611fe81a895238c23464da579516eb6a02af9b2)) - Renovate Bot
- **(deps)** update rust crate serde_yaml to 0.9.32 - ([0b21e3e](https://codeberg.org/PurpleBooth/git-mit/commit/0b21e3e7e7424367e81d66cb2ceb34157a223eba)) - Renovate Bot
- **(deps)** update rust crate clap_complete to 4.5.1 - ([44db99a](https://codeberg.org/PurpleBooth/git-mit/commit/44db99a7a187515cd7f38b065e708f132f60ba89)) - Renovate Bot
- **(deps)** update rust crate mit-commit to 3.1.8 - ([2a2e049](https://codeberg.org/PurpleBooth/git-mit/commit/2a2e049df0b187e212e2bdff287cc2cb7ebc38d8)) - Renovate Bot
- **(deps)** update rust crate serde to 1.0.197 - ([857cc36](https://codeberg.org/PurpleBooth/git-mit/commit/857cc36543a7fe7f54b79671f5bfbfdba9d9b13e)) - Renovate Bot
- **(deps)** update rust crate clap to 4.5.1 - ([967555f](https://codeberg.org/PurpleBooth/git-mit/commit/967555f143f92328a50b9da322aeda79abfe91bc)) - Renovate Bot
- **(deps)** bump thiserror from 1.0.56 to 1.0.57 - ([a8a5c91](https://codeberg.org/PurpleBooth/git-mit/commit/a8a5c91aec16670c145154b630d75a66b3a0c775)) - dependabot[bot]
- **(deps)** bump clap from 4.4.18 to 4.5.0 - ([fa74db1](https://codeberg.org/PurpleBooth/git-mit/commit/fa74db19ed93233c9ff18b4daace086bf25e27ae)) - dependabot[bot]
- **(deps)** bump clap_complete from 4.4.10 to 4.5.0 - ([f8506b2](https://codeberg.org/PurpleBooth/git-mit/commit/f8506b279c15699e3fc5336fc16761f6d5d94a02)) - dependabot[bot]
- **(deps)** bump git2 from 0.18.1 to 0.18.2 - ([ce481d3](https://codeberg.org/PurpleBooth/git-mit/commit/ce481d3f3068ab02e62844d2b6194016d6e8d33c)) - dependabot[bot]
- **(deps)** bump tempfile from 3.9.0 to 3.10.0 - ([a30e37a](https://codeberg.org/PurpleBooth/git-mit/commit/a30e37a92cd776ee4b630e94092d66ed811b696e)) - dependabot[bot]
- **(deps)** bump miette from 5.10.0 to 7.0.0 - ([9f99012](https://codeberg.org/PurpleBooth/git-mit/commit/9f99012dcc632f36808db5abe678b08c968455ef)) - dependabot[bot]
- **(deps)** bump clap_complete from 4.4.9 to 4.4.10 - ([4a621e2](https://codeberg.org/PurpleBooth/git-mit/commit/4a621e2a9d2dbc499865f20251298df52fc4c5f7)) - dependabot[bot]
- **(deps)** bump serde from 1.0.195 to 1.0.196 - ([fcdab89](https://codeberg.org/PurpleBooth/git-mit/commit/fcdab89d963b8065c95935be8678645e3fcf0acf)) - dependabot[bot]
- **(deps)** bump serde_yaml from 0.9.30 to 0.9.31 - ([e04542f](https://codeberg.org/PurpleBooth/git-mit/commit/e04542f9e4c074307e2b7e20f275a8db2dfaebb1)) - dependabot[bot]
- **(deps)** bump git2 from 0.16.1 to 0.18.1 - ([22e5490](https://codeberg.org/PurpleBooth/git-mit/commit/22e54908b7f6401f89a0b05b06caef2e81046a2a)) - dependabot[bot]
- **(deps)** bump inquire from 0.5.3 to 0.6.2 - ([f40078c](https://codeberg.org/PurpleBooth/git-mit/commit/f40078c462f60c5da2f69815ae97cda75c959a7d)) - dependabot[bot]
- **(deps)** Bump versions - ([745adc8](https://codeberg.org/PurpleBooth/git-mit/commit/745adc87c559e8330a0800ea04598f1b96e1fdd8)) - Billie Thompson
- **(deps)** bump mit-commit from 3.1.4 to 3.1.5 - ([6125a79](https://codeberg.org/PurpleBooth/git-mit/commit/6125a79f07d5f0150ad06b1d53da4181c36dcb4a)) - dependabot[bot]
- **(deps)** bump inquire from 0.5.2 to 0.5.3 - ([8fc3f0a](https://codeberg.org/PurpleBooth/git-mit/commit/8fc3f0a2c9a2e359ecb2a71cbdc14ebde594e984)) - dependabot[bot]
- **(deps)** bump thiserror from 1.0.37 to 1.0.38 - ([9c65a99](https://codeberg.org/PurpleBooth/git-mit/commit/9c65a994a0a4cff85543d816437240c4567199ec)) - dependabot[bot]
- **(deps)** bump nom from 7.1.1 to 7.1.3 - ([8d0fbf5](https://codeberg.org/PurpleBooth/git-mit/commit/8d0fbf5ff62f112c54eb0d8216c943f8626be1c0)) - dependabot[bot]
- **(deps)** bump git2 from 0.15.0 to 0.16.1 - ([b41d281](https://codeberg.org/PurpleBooth/git-mit/commit/b41d2816c6777558a1f213f2c079b205897ccb62)) - dependabot[bot]
- **(deps)** bump libgit2-sys from 0.14.0+1.5.0 to 0.14.2+1.5.1 - ([61d105c](https://codeberg.org/PurpleBooth/git-mit/commit/61d105c0f8f56784c8dd389c8a35b89fc32ea323)) - dependabot[bot]
- **(deps)** bump serde from 1.0.147 to 1.0.152 - ([9996ed5](https://codeberg.org/PurpleBooth/git-mit/commit/9996ed5c4fae11d573102777a241c986fb914bbf)) - dependabot[bot]
- **(deps)** bump miette from 5.4.1 to 5.5.0 - ([6964832](https://codeberg.org/PurpleBooth/git-mit/commit/6964832854c80cd6c5ae08d97a4cc2fc8a271a9d)) - dependabot[bot]
- **(deps)** bump serde_yaml from 0.9.14 to 0.9.17 - ([9fbaafd](https://codeberg.org/PurpleBooth/git-mit/commit/9fbaafd51d6400a1c51296882ffc416b2f083783)) - dependabot[bot]
- **(deps)** bump clap_complete from 4.0.3 to 4.0.5 - ([1dcd8bc](https://codeberg.org/PurpleBooth/git-mit/commit/1dcd8bc5556d52723c416961375150543c243a17)) - dependabot[bot]
- **(deps)** bump inquire from 0.5.1 to 0.5.2 - ([d46eadf](https://codeberg.org/PurpleBooth/git-mit/commit/d46eadfed92392da4bc0977fae33f6f726a0b096)) - dependabot[bot]
- **(deps)** bump miette from 5.3.0 to 5.4.1 - ([4eac481](https://codeberg.org/PurpleBooth/git-mit/commit/4eac481994cd901c604eeea227b7c6e05ab8bf97)) - dependabot[bot]
- **(deps)** bump inquire from 0.4.0 to 0.5.1 - ([37b9bde](https://codeberg.org/PurpleBooth/git-mit/commit/37b9bde73bf55c60c8fe85503607846bffc0d7ff)) - dependabot[bot]
- **(deps)** Bumping dependencies - ([69373bb](https://codeberg.org/PurpleBooth/git-mit/commit/69373bb42e137da920c8f6be426ce69cc9a1afee)) - Billie Thompson
- **(deps)** bump serde from 1.0.145 to 1.0.147 - ([b65ce13](https://codeberg.org/PurpleBooth/git-mit/commit/b65ce1319a532088208222e207b239af852cbdd8)) - dependabot[bot]
- **(src)** Fix clippy advice - ([97f3186](https://codeberg.org/PurpleBooth/git-mit/commit/97f3186358b438f087637991cb89a3d195740868)) - Billie Thompson
- **(src)** Clippy advice - ([21d0d09](https://codeberg.org/PurpleBooth/git-mit/commit/21d0d09bd1bb70074f27ddc6669b183f12c90969)) - Billie Thompson
- disable more things - ([20095b9](https://codeberg.org/PurpleBooth/git-mit/commit/20095b90ebe2effd6dda93030b4ad3e29f173d08)) - Billie Thompson
- Disable dependants thingy - ([25f82a8](https://codeberg.org/PurpleBooth/git-mit/commit/25f82a8d17be39362ce239eee22a6ac39956601a)) - Billie Thompson
- disable the dependants check - ([bffb040](https://codeberg.org/PurpleBooth/git-mit/commit/bffb0404dc47df43450641f98a9fc83dd28ff358)) - Billie Thompson
- Disable upgrade - ([206cb18](https://codeberg.org/PurpleBooth/git-mit/commit/206cb189c7a37b3d36e3313ce2856bf59c9e393c)) - Billie Thompson
- Correct escaping of readme - ([765da3d](https://codeberg.org/PurpleBooth/git-mit/commit/765da3dda0d8e70e3de68c71c3120d6083f3a2de)) - Billie Thompson
#### Build system
- **(Actions)** install cog in the pipeline - ([5dd00d0](https://codeberg.org/PurpleBooth/git-mit/commit/5dd00d03619393ef58bc399af03178b55093cf68)) - Billie Thompson
#### Continuous Integration
- **(Actions)** Switch to our actions repo - ([e4c53c6](https://codeberg.org/PurpleBooth/git-mit/commit/e4c53c688bc0198006692e6f7278a816f57a5447)) - Billie Thompson
- **(Mergify)** configuration update - ([5a067fe](https://codeberg.org/PurpleBooth/git-mit/commit/5a067fe210f3250438fff442392c361ac9e835f7)) - Billie Thompson
- **(deps)** bump PurpleBooth/versio-release-action from 0.1.15 to 0.1.17 - ([1c0896b](https://codeberg.org/PurpleBooth/git-mit/commit/1c0896b5d824ac3768acc635fd7ac5da11bdd95d)) - dependabot[bot]
- Remove duplicated step - ([b24aa39](https://codeberg.org/PurpleBooth/git-mit/commit/b24aa39cb1e5a6a4e23a7c3d79741eb40d6c2874)) - Billie Thompson
- Install rust - ([4c014df](https://codeberg.org/PurpleBooth/git-mit/commit/4c014df3cb269e703b2b1e13a3468bb23c915c87)) - Billie Thompson
- Trim down old pipeline - ([28a398b](https://codeberg.org/PurpleBooth/git-mit/commit/28a398b69fbb54bfc2c5b318412b37b2c21ca4b2)) - Billie Thompson
- Format renovate - ([e0d8fec](https://codeberg.org/PurpleBooth/git-mit/commit/e0d8fec7e3591ea17b5ee48c4a4e17afca1d9d51)) - Billie Thompson
- update renovate config - ([174cc45](https://codeberg.org/PurpleBooth/git-mit/commit/174cc455223bf06bcab5f4187e786adfc8ad90b1)) - Billie Thompson
- Switch to our own installer - ([1426f91](https://codeberg.org/PurpleBooth/git-mit/commit/1426f91566ea0d02cdb4a02adeca36b87183de77)) - Billie Thompson
- switch to our own specdown installerr - ([7200bcf](https://codeberg.org/PurpleBooth/git-mit/commit/7200bcf8815c8aef035cfdd52454e106002fb65c)) - Billie Thompson
- Simplify pipeline - ([5a01848](https://codeberg.org/PurpleBooth/git-mit/commit/5a018484accd8998bd5effe1b9da69c5ad8a7216)) - Billie Thompson
- Move github actions to forgejo actions - ([a074eca](https://codeberg.org/PurpleBooth/git-mit/commit/a074eca767dc5ad43a192ab1434776792f5e07e4)) - Billie Thompson
- Switch to Forgejo - ([d6ecdba](https://codeberg.org/PurpleBooth/git-mit/commit/d6ecdba0c403262640ab84b5d9c92a82361fa1d4)) - Billie Thompson
- Add renovate config - ([8e0680a](https://codeberg.org/PurpleBooth/git-mit/commit/8e0680acdfb8a2c3c022721f4f4408be525235e3)) - Billie Thompson
- Try woodpecker - ([def3fb3](https://codeberg.org/PurpleBooth/git-mit/commit/def3fb33cb83936999be07e2e6dbd9670320a085)) - Billie Thompson
- 1 is the correct disabled value - ([0163f2d](https://codeberg.org/PurpleBooth/git-mit/commit/0163f2d6d62ffce10d1069a3e1ca7afd1229716d)) - Billie Thompson
- Disable updates on brew install - ([4014354](https://codeberg.org/PurpleBooth/git-mit/commit/401435468588393369dca19b00e8feaa26efe4ad)) - Billie Thompson
#### Documentation
- **(src)** Change the tests to match the new output from Miette - ([bbcbb34](https://codeberg.org/PurpleBooth/git-mit/commit/bbcbb34dd154926dda60cbebb797a64297bec247)) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** pin actions/checkout action to f43a0e5 - ([c024136](https://codeberg.org/PurpleBooth/git-mit/commit/c024136beb7ade4469fb4c5fe0691fbed5a45d6b)) - Solace System Renovate Fox
- **(deps)** pin dependencies - ([861ae39](https://codeberg.org/PurpleBooth/git-mit/commit/861ae39551642c88a46befb9e7e8e19492064b09)) - Renovate Bot
- **(deps)** update rust crate tempfile to 3.10.1 - ([3bff262](https://codeberg.org/PurpleBooth/git-mit/commit/3bff2629f3a59d0bf2102a8ad3022ea359e06810)) - Renovate Bot
- **(deps)** update purplebooth/versio-release-action action to v0.1.18 - ([0ed4faa](https://codeberg.org/PurpleBooth/git-mit/commit/0ed4faa14554eb42d63e2bbb14f813b0793b20f7)) - Renovate Bot
#### Refactoring
- **(src)** use standard min length validator - ([4dd652f](https://codeberg.org/PurpleBooth/git-mit/commit/4dd652f0ff8b6475bae42c274c75a586cccba4ef)) - Billie Thompson
- Remove unneeded lifetime - ([bb7aa1d](https://codeberg.org/PurpleBooth/git-mit/commit/bb7aa1d15444c46dbb1bea0704f438c0533f61ef)) - Billie Thompson
#### Tests
- **(src)** Correct output to match new clap format - ([d8f7225](https://codeberg.org/PurpleBooth/git-mit/commit/d8f7225d3b1b45a4bfd9e9ff747b5f5b5e79bf85)) - Billie Thompson

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).