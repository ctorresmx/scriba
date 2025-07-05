import { ParsedPost } from "../types/blog.ts";

interface Props {
  post: ParsedPost;
}

export default function AuthorDateDetails({ post }: Props) {
  return (
    <>
      <span>by {post.attributes.author}</span>
      <span class="mx-2">•</span>
      <time dateTime={post.attributes.date.toISOString()}>
        {post.formattedDate}
      </time>
    </>
  )
}