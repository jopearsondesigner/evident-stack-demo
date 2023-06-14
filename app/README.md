# Evident Stack UI

Powered by:

* [Svelte](https://svelte.dev) components and [SvelteKit](https://kit.svelte.dev) as our application framework
* [Tailwind CSS](https://tailwindcss.com/) and [DaisyUI](https://daisyui.com/) for styling and components
* [Storybook](https://storybook.js.org/) to help manage our Design System and test our components
* [Vite](https://vitejs.dev/) for building/packaging/etc.
* [Rust/WASM](https://rustwasm.github.io/book/) for state management and client/edge/server code sharing
* [Automerge](https://automerge.org/) for collaboration and state management
* [Supabase](https://supabase.com/) for auth, DB, and real-time streams

## Install dependencies

Before any of the command below will work, install the dependencies:

```bash
npm install
npm install -g esbuild <This globally installs the correct version of ESBuild for your environment mac vs linux without causing conflicts by directly installing in package.json>
```

## Install Supabase

We use a Supabase container in the backend to serve as our database and realtime channels
```bash
npx supabase start
```

This will print out some environment variables to the console (if you ever need to reference them again you can see them with ```npx supabase status```).
The following keys will be set as variables in your .env file in /app:

supabase         | .env
-------------------------------------------
API URL          | PUBLIC_SUPABASE_URL
anon secret      | PUBLIC_SUPABASE_ANON_KEY
service_role key | PUBLIC_SERVICE_KEY

### Accessing authentication emails when running local

Visit the local [InBucket](http://localhost:54324) service to get all
email sent by the local service.

## Running Storybook

To begin interactive development on our [library of components](./src/lib/components/), start the Storybook server:

```bash
npm run storybook
```

## Running the App

To run the app based on the SvelteKit [routes](./src/routes/), start a development server:

```bash
npm run dev

npm run dev -- --open
```

## Building

To create a production version of the app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.
