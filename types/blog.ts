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
  formattedDate: string; // Formatted date like YYYY/MM/DD
}

export interface BlogConfig {
  name: string;
  title: string;
  copyright: string;
  postsDir: string;
  faviconText: string;
}
