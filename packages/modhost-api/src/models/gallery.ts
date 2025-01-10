export interface GalleryImage {
    id: number;
    name: string;
    description?: string;
    ordering: number;
    project: number;
    url: string;

    /**
     * Can be converted to a {@link Date}.
     */
    created_at: string;

    /**
     * Can be converted to a {@link Date}.
     */
    updated_at: string;
}

export interface GalleryImageInit {
    name: string;
    ordering: number;
    project: number;
    file: File | Blob;
    description?: string;
}
