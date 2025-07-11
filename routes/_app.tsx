import { type PageProps } from "$fresh/server.ts";
import { getBlogHeaderTitle } from "../utils/config.ts";

export default function App({ Component }: PageProps) {
  return (
    <html>
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>{getBlogHeaderTitle()}</title>
        <link rel="stylesheet" href="/styles.css" />
      </head>
      <body class="font-mono">
        <Component />
      </body>
    </html>
  );
}
