import { ParsedPost } from "../types/blog.ts";
import { render } from "@deno/gfm";

interface Props {
  post: ParsedPost;
}

export default function ArticleSummary({ post }: Props) {
  return (
    <div class="card card-border bg-base-300 w-3/4 shadow-sm m-4">
      <div class="card-body">
        <h2 class="card-title">{post.attributes.title}</h2>
        {post.attributes.excerpt ?
          <p>{post.attributes.excerpt}</p>
          : <div dangerouslySetInnerHTML={{ __html: render(post.content) }} />
        }
        <div class="card-actions justify-end">
          <button class="btn btn-primary">Read more</button>
        </div>
      </div>
    </div>
  );
}