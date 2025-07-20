import { getBlogConfig } from "../utils/config.ts";

export default function Header() {
  const { title } = getBlogConfig();
  return (
    <div class="navbar bg-base-300 col-span-2">
      <div class="navbar-start">
      </div>
      <div class="navbar-center">
        <a
          href="/"
          class="no-underline hover:no-underline visited:text-current"
        >
          <h1 class="text-2xl primary">{title}</h1>
        </a>
      </div>
      <div class="navbar-end">
      </div>
    </div>
  );
}
