import { ParsedPost } from "../types/blog.ts";
import { render } from "@deno/gfm";
import AuthorDateDetails from "./AuthorDateDetails.tsx";

interface Props {
  post: ParsedPost;
}

export default function ArticleSummary({ post }: Props) {
  return (
    <div class="card card-border bg-base-300 w-3/4 shadow-sm m-4">
      <div class="card-body">
        <h2 class="card-title">
          <a href={post.url} class="hover:text-primary">
            {post.attributes.title}
          </a>
        </h2>

        <div class="text-sm text-base-content/70 mb-2">
          <AuthorDateDetails post={post} />
        </div>

        {post.attributes.tags.length > 0 && (
          <div class="flex flex-wrap gap-1 mb-3">
            {post.attributes.tags.map((tag) => (
              <span
                key={tag}
                class="badge badge-sm badge-outline"
              >
                {tag}
              </span>
            ))}
          </div>
        )}

        {post.attributes.excerpt ?
          <p>{post.attributes.excerpt}</p>
          : <div class="prose prose-sm mb-4 line-clamp-3"
              dangerouslySetInnerHTML={{ __html: render(post.content) }} />
        }
        <div class="card-actions justify-end">
          <a href={post.url} class="btn btn-primary">
            Read more
          </a>
        </div>
      </div>
    </div>
  );
}