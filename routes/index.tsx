import ArticleSummary from "../components/ArticleSummary.tsx";

export default function Home() {
  return (
    <div class="flex flex-col items-center p-4">
      <ArticleSummary title="First article" summary="This is an example of an article" />
      <ArticleSummary title="Second article" summary="This is another example of an article" />
    </div>
  );
}
