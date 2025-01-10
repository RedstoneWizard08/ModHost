export interface ProjectVersion {
    id: number;
    name: string;
    project: number;
    version_number: string;
    changelog?: string;
    downloads: number;
    loaders: string[];
    game_versions: string[];
    files: ProjectFile[];

    /**
     * Can be converted to a {@link Date}.
     */
    created_at: string;

    /**
     * Can be converted to a {@link Date}.
     */
    updated_at: string;
}

export interface ProjectFile {
    id: number;
    file_name: string;
    s3_id: string;
    sha1: string;
    version_id: number;
    size: number;

    /**
     * Can be converted to a {@link Date}.
     */
    uploaded_at: string;
}

export interface ProjectVersionInit {
    name: string;
    file_name: string;
    version_number: string;
    game_versions: string[];
    loaders: string[];
    file: File | Blob;
    changelog?: string;
}
