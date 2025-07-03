export interface PostAttributes {
  title: string;
  date: Date;
  author: string;
  tags: string[];
  status: "draft" | "published" | "scheduled";
  excerpt?: string;
}

export interface ParsedPost {
  attributes: PostAttributes;
  content: string;
  slug: string;
  url: string; // Generated URL like YYYY/MM/DD/slug
}

export interface BlogConfig {
  title: string;
  description: string;
  author: string;
  baseUrl: string;
  postsPerPage: number;
}