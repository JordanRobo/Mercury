# MercuryCMS

MercuryCMS is a high-performance headless Content Management System (CMS) designed to provide a lightning-fast and flexible backend solution for building highly customizable frontends. Inspired by the Roman god of speed and messengers, MercuryCMS delivers unparalleled performance and scalability, empowering developers to create exceptional digital experiences.

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

2. Install server-side dependencies:
    1. Navigate to the backend directory:
    ```bash
    cd Mercury/server
    ```
    2. Install dependencies:
    ```bash
    cargo build
    ```
    3. Start backend server:
    ```bash
    cargo run
    ``` 

3. Install client-side dependencies:
    1. Navigate to the frontend directory:
    ```bash
    cd Mercury/admin
    ```
    2. Install dependencies:
    ```bash
    bun install
    ```
    3. Start frontend server:
    ```bash
    bun run dev
    ``` 

## Contributing

We welcome contributions from the community! If you'd like to contribute to MercuryCMS, please follow our [Contributing Guidelines](CONTRIBUTING.md).

## License

MercuryCMS is released under the [MIT License](LICENSE).

## Acknowledgements

MercuryCMS is built with the following technologies:

- [Rust](https://www.rust-lang.org/)
- [Actix](https://actix.rs/)
- [SvelteKit](https://kit.svelte.dev/)
- [Bun](https://bun.sh/)

We'd like to express our gratitude to the developers and communities behind these amazing projects.