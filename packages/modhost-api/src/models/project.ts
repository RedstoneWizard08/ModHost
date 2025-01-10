import type { User } from "./user";

export type ProjectVisibility = "Public" | "Private" | "Unlisted";

export interface Project {
    id: number;
    slug: string;
    name: string;
    description: string;
    downloads: number;
    issues?: string;
    license?: string;
    readme: string;
    source?: string;
    tags?: string[];
    visibility: ProjectVisibility;
    wiki?: string;

    /**
     * Can be converted to a {@link Date}.
     */
    created_at: string;

    /**
     * Can be converted to a {@link Date}.
     */
    updated_at: string;
}

export interface FullProject extends Project {
    authors: User[];
}

export interface ProjectInit {
    name: string;
    slug: string;
    readme: string;
    description: string;
    issues?: string;
    license?: string;
    source?: string;
    tags?: string[];
    visibility?: ProjectVisibility;
    wiki?: string;
}
