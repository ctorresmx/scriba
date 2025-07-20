import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { ParsedPost } from "../../../../types/blog.ts";
import { getAllPosts } from "../../../../utils/parsing.ts";
import AuthorDateDetails from "../../../../components/AuthorDateDetails.tsx";
import { getBlogHeaderTitle } from "../../../../utils/config.ts";
import { CSS, render } from "@deno/gfm";
import "npm:prismjs@1.29.0/components/prism-typescript.js";
import "npm:prismjs@1.29.0/components/prism-java.js";
import "npm:prismjs@1.29.0/components/prism-json.js";
import "npm:prismjs@1.29.0/components/prism-diff.js";

interface ArticlePageProps {
  post: ParsedPost;
}

export const handler: Handlers<ArticlePageProps> = {
  async GET(_req, ctx) {
    const { year, month, day, slug } = ctx.params;

    const posts = await getAllPosts();
    const post = posts.find((p) => {
      const formattedDate = `${year}/${month}/${day}`;
      return formattedDate === p.formattedDate &&
        slug === p.slug && p.attributes.status === "published";
    });

    if (!post) {
      return ctx.renderNotFound();
    }
    return ctx.render({ post });
  },
};

export default function ArticlePage({ data }: PageProps<ArticlePageProps>) {
  const { post } = data;

  return (
    <>
      <Head>
        <title>{getBlogHeaderTitle(post.attributes.title)}</title>
        <style dangerouslySetInnerHTML={{ __html: CSS }} />
        <style>
          {`
          .markdown-body {
            background-color: #2a323c !important;
            background: #2a323c !important;
          }
        `}
        </style>
      </Head>

      <article class="max-w-4xl mx-auto px-4 py-8">
        <header class="mb-8">
          <h1 class="text-4xl font-bold mb-4">{post.attributes.title}</h1>

          <div class="flex flex-wrap items-center gap-1 text-sm text-base-content/70 mb-4">
            <AuthorDateDetails post={post} />
          </div>

          {post.attributes.tags.length > 0 && (
            <div class="flex flex-wrap gap-2">
              {post.attributes.tags.map((tag) => (
                <span
                  key={tag}
                  class="badge badge-outline"
                >
                  {tag}
                </span>
              ))}
            </div>
          )}
        </header>

        <div
          data-color-mode="dark"
          data-dark-theme="dark"
          class="markdown-body p-6 rounded-lg"
          dangerouslySetInnerHTML={{ __html: render(post.content) }}
        />
      </article>
    </>
  );
}
