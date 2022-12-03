![Travelto](src/www/logo/cover.png)

## About
**Travelto** tries to find the cheapest way to travel between two destinations.

## Configuration
API keys are configured trough environment variables.  
You can either set them up manually, or you can create a `.env` file in the root directory.
Here is the layout:
```
KIWI_SEARCH=...
KIWI_MULTICITY=...
KIWI_NOMAD=...
RAPID_KEY=...
RAPID_SKYSCANNER_HOST=...
```

Here are links to the websites where you can register for API access:
* [tequila.kiwi.com](https://tequila.kiwi.com)
* [rapidapi.com - Skyscanner](https://rapidapi.com/3b-data-3b-data-default/api/skyscanner44)

## Contributing
Feel free to contribute!

Use tools such as [Rustfmt](https://github.com/rust-lang/rustfmt) and [Clippy](https://github.com/rust-lang/rust-clippy) to improve your code.  
Commit messages should follow [conventionalcommits.org](https://www.conventionalcommits.org).  
Where type is one of the following: `feat`, `fix`, `ci`, `docs` or `refactor`.

## License
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) for details.
