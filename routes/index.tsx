import ArticleSummary from "../components/ArticleSummary.tsx";
import { getAllPosts } from "../utils/parsing.ts";

export default async function Home() {
  const posts = (await getAllPosts()).filter((p) => {
    return p.attributes.status === "published";
  });

  return (
    <div class="flex flex-col items-center p-4">
      {posts.map((post) => {
        return <ArticleSummary key={post.slug} post={post} />;
      })}
    </div>
  );
}
