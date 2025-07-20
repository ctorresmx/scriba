import { getBlogConfig } from "../utils/config.ts";

export default function Footer() {
  const { copyright } = getBlogConfig();
  return (
    <footer class="col-span-2 footer sm:footer-horizontal footer-center bg-base-300 text-base-content p-4">
      <aside>
        <p>Copyright © {new Date().getFullYear()} {copyright}</p>
      </aside>
    </footer>
  );
}
