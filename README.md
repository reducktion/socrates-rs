<p align="center">
    <img src="https://raw.githubusercontent.com/AlexOlival/socrates/master/docs/logo.png" alt="Socrates logo" width="480">
</p>

```rust
use socrates_rs;

socrates_rs::validate_id("14349483 0 ZV3", socrates_rs::country::Code::PT);

let citizen = socrates_rs::extract_information("2820819398814 09", socrates_rs::country::Code::FR).unwrap();
assert_eq!(citizen.gender, 'F');
assert_eq!(citizen.year_of_birth, 1982);
assert_eq!(citizen.month_of_birth.unwrap(), 8);
assert_eq!(citizen.place_of_birth.unwrap(), "Corrèze");
```
------
# Introduction
This rust crate is a port of the [php package socrates](https://github.com/reducktion/socrates).

**socrates-rs** allows you to validate and retrieve personal data from [National Identification Numbers](https://en.wikipedia.org/wiki/National_identification_number) across the world with the goal of eventually supporting as many countries in the world as possible.
<p>Some countries also encode personal information of the citizen, such as gender or the place of birth. This package allows you to extract that information in a consistent way.</p>
<p>This crate can be useful for many things, such as validating a user's ID for finance related applications or verifying a user's age without asking for it explicitly. However, we recommend you review your country's data protection laws before storing any information.</p>

## Usage
Two functions are available with socrates-rs:
 * `validate_id` which returns a boolean indicating if an id is valid in a specific country
 * `extract_information` which returns an Optional `Citizen` with information retrievable from the identifier (gender, date of birth, ...)
 
The list of supported countries is available via the [`Country::code`](https://github.com/reducktion/socrates-rs/blob/master/src/country.rs) enum.
 

## Contributing
Did you find a problem in any of the algorithms? 
Do you know how to implement a country which we have missed?
Are there any improvements that you think should be made to the codebase?
Any help is appreciated! Take a look at our [contributing guidelines](https://github.com/reducktion/socrates/blob/master/CONTRIBUTING.md).

## License
The MIT License (MIT). Please see [License File](LICENSE.md) for more information. 
