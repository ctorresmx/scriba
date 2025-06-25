interface Props {
  title: string;
  summary: string;
}

export default function ArticleSummary({ title, summary }: Props) {
  return (
    <div class="card card-border bg-base-300 w-3/4 shadow-sm m-4">
      <div class="card-body">
        <h2 class="card-title">{title}</h2>
        <p>{summary}</p>
        <div class="card-actions justify-end">
          <button class="btn btn-primary">Read more</button>
        </div>
      </div>
    </div>
  );
}