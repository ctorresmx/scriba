export interface FrontmatterPost {
  title: string;
  date: string; // YYYY-MM-DD format
  author: string;
  tags: string[];
  status: "draft" | "published" | "scheduled";
}

export interface ParsedPost {
  frontmatter: FrontmatterPost;
  content: string;
  slug: string;
  filename: string;
  url: string; // Generated URL like YYYY/MM/DD/slug
}

export interface PostSummary {
  title: string;
  date: string;
  author: string;
  tags: string[];
  slug: string;
  url: string;
  excerpt?: string; // custom or generated
}

export interface BlogConfig {
  title: string;
  description: string;
  author: string;
  baseUrl: string;
  postsPerPage: number;
}