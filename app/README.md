# oNote Editor

Powered by:

* [Svelte](https://svelte.dev) components and [SvelteKit](https://kit.svelte.dev) as our application framework
* [Tailwind CSS](https://tailwindcss.com/) and [DaisyUI](https://daisyui.com/) for styling and components
* [Storybook](https://storybook.js.org/) to help manage our Design System and test our components
* [Vite](https://vitejs.dev/) for building/packaging/etc.

## Install dependencies

Before any of the command below will work, install the dependencies:

```bash
yarn
```

## Running Storybook

To begin interactive development on our [library of components](./src/lib/components/), start the Storybook server:

```bash
yarn storybook
```

## Running the App

To run the app based on the SvelteKit [routes](./src/routes/), start a development server:

```bash
yarn dev

yarn dev -- --open
```

## Building

To create a production version of the app:

```bash
yarn build
```

You can preview the production build with `yarn preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.
