# Mercury CMS

Mercury CMS is a high-performance headless Content Management System (CMS) designed to provide a lightning-fast and flexible backend solution for building highly customizable frontends. Inspired by the Roman god of speed and messengers, MercuryCMS delivers unparalleled performance and scalability, empowering developers to create exceptional digital experiences.

## Project Structure

MercuryCMS follows a modular structure, separating the admin panel and backend server into their respective directories:

- **admin/**: Contains the SvelteKit and Bun code for the admin panel.
  - **src/**: The main source code for the admin panel.
  - **static/**: Static assets like images, fonts, etc.
  - **tests/**: Tests for the admin panel.
  - Configuration files and package managers.

- **sdk/**: Contains the Bun and Typescript Client SDK.
  - **src/**: The main source code for the SDK.
  - **tests/**: Tests for the SDK.
  - Configuration files and package managers.

- **server/**: Contains the Rust and Actix code for the backend server.
  - **src/**: The main source code for the backend server.
  - **migrations/**: Database migration files.
  - **tests/**: Tests for the backend server.
  - Configuration files and package managers.

- **docs/**: Documentation files, including API reference, guides, and tutorials.
- **.gitignore**: Git ignore file for excluding unnecessary files from version control.
- **README.md**: The project's main README file with installation instructions, usage examples, and other information.

## Features

- **Blazing-Fast Performance**: Built with cutting-edge technologies like Rust, Actix, SvelteKit, and Bun, MercuryCMS offers unparalleled speed and responsiveness, ensuring a seamless user experience.
- **Headless Architecture**: MercuryCMS decouples the backend from the frontend, allowing developers to build frontends using any framework or technology of their choice, ensuring maximum flexibility and customization.
- **Robust Content Management**: Manage all your content types with ease, including posts, pages, products, media, and more. Create, edit, and organize your content efficiently with a user-friendly admin interface.
- **Rich Text Editing**: Utilize a powerful WYSIWYG editor to craft rich and engaging content, with support for formatting, images, videos, and other media.
- **Media Library**: Upload, manage, and serve media files (images, videos, documents) from a centralized location.
- **Taxonomies and Categories**: Organize your content with custom taxonomies, categories, and tags for easy navigation and discoverability.
- **User Management and Roles**: Manage user accounts, assign roles (admin, editor, author), and control access permissions.
- **Form Builder**: Create and manage custom forms for data collection, such as contact forms, surveys, and more.
- **SEO Optimization**: Enhance your website's search engine visibility with built-in SEO features, including meta tags, sitemaps, and more.
- **Multilingual Support**: Enable multiple languages for your content and interfaces, catering to a global audience.
- **Extensible and Customizable**: MercuryCMS is designed to be highly extensible, allowing developers to add new features and functionality through plugins and custom integrations.
- **API and Webhooks**: Integrate MercuryCMS with other systems and services through a powerful API and webhook system.

## Getting Started

Follow the instructions below to get MercuryCMS up and running on your local machine.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/) (with npm)
- [Bun](https://bun.sh/docs/installation)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/your-username/Mercury.git
```

2. Navigate to the project directory:
```bash
cd Mercury
```

3. Install server-side dependencies:
```bash
cd server
cargo build
```

4. Install client-side dependencies:
```bash
cd ../admin
bun install
```

5. Start the development server:
```bash
bun run start
```
The MercuryCMS admin panel should now be running locally at `http://localhost:3000`, and the backend server at `http://localhost:8000`.

## Contributing

We welcome contributions from the community! If you'd like to contribute to MercuryCMS, please follow our [Contributing Guidelines](CONTRIBUTING.md).

## License

 Copyright (C) Mercury CMS - All Rights Reserved.
 Unauthorized copying of this file, via any medium is strictly prohibited.
 Written By Jordan Robinson <jordan@stateot.art>, June 2024.


## Acknowledgements

MercuryCMS is built with the following technologies:

- [Rust](https://www.rust-lang.org/)
- [Actix](https://actix.rs/)
- [SvelteKit](https://kit.svelte.dev/)
- [Bun](https://bun.sh/)

We'd like to express our gratitude to the developers and communities behind these amazing projects.