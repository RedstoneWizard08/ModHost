import {
    ErrorResponse,
    serializeFacets,
    type Facet,
    type FullProject,
    type GalleryImage,
    type GalleryImageInit,
    type GameVersion,
    type ModLoader,
    type ProjectInit,
    type ProjectVersion,
    type ProjectVersionInit,
    type SearchResults,
    type SortDirection,
    type SortMode,
    type Tag,
    type User,
} from "./models";

/**
 * The base API client.
 * This is used internally for requests and isn't as ergonomic as the wrapped client.
 */
export class Client {
    private _baseUrl: string;
    private _token?: string;

    public constructor(baseUrl = "/api/v1", token?: string) {
        this._baseUrl = baseUrl.endsWith("/") ? baseUrl.substring(0, baseUrl.length - 1) : baseUrl;
        this._token = token;
    }

    public setToken(token: string) {
        this._token = token;
    }

    public unsetToken() {
        this._token = undefined;
    }

    public hasToken() {
        return !!this._token;
    }

    private _fetch(
        requiresAuth: boolean = false,
        method: "GET" | "POST" | "PUT" | "DELETE" | "PATCH",
        path: string,
        headers: Record<string, any> = {},
        body?: Blob | FormData | object | string,
    ) {
        if (requiresAuth && !this._token) {
            throw new Error("Request requires authentication but no token was found!");
        }

        const realPath = path.startsWith("/") ? path.substring(1) : path;

        const data =
            !(body instanceof FormData) && typeof body === "object" ? JSON.stringify(body) : body;

        const fullHeaders = {
            ...headers,
        };

        if (this._token) {
            fullHeaders["Authorization"] = `Bearer ${this._token}`;
        }

        if (body instanceof FormData) {
            fullHeaders["Content-Type"] = "multipart/form-data";
        } else if (typeof body === "object") {
            fullHeaders["Content-Type"] = "application/json";
        }

        return fetch(`${this._baseUrl}/${realPath}`, {
            method: method,
            body: data,
            headers: fullHeaders,
        });
    }

    private async _jsonFetch<T>(
        requiresAuth: boolean = false,
        method: "GET" | "POST" | "PUT" | "DELETE" | "PATCH",
        path: string,
        headers: Record<string, any> = {},
        body?: Blob | FormData | object | string,
    ): Promise<T | ErrorResponse> {
        try {
            return (await (
                await this._fetch(requiresAuth, method, path, headers, body)
            ).json()) as T;
        } catch (err: unknown) {
            return new ErrorResponse(err);
        }
    }

    // =================== USERS ===================

    public async currentUser() {
        return await this._jsonFetch<User>(true, "GET", "/users/me");
    }

    public async getUser(id: number | string) {
        return await this._jsonFetch<User>(false, "GET", `/users/${id}`);
    }

    public async getUserProjects(id: number | string) {
        return await this._jsonFetch<FullProject[]>(false, "GET", `/users/${id}/projects`);
    }

    // =================== PROJECTS ===================

    public async searchProjects(
        query?: string,
        page = 1,
        perPage = 25,
        sort: SortMode = "none",
        dir: SortDirection = "desc",
        filters: Facet[] = [],
    ) {
        const queryObj: Record<string, string> = {};

        if (query) queryObj["q"] = encodeURIComponent(query);
        if (sort) queryObj["sort"] = sort;
        if (dir) queryObj["dir"] = dir;

        queryObj["page"] = page.toString();
        queryObj["per_page"] = perPage.toString();
        queryObj["filters"] = serializeFacets(filters);

        let queryStr = "";

        for (const [k, v] of Object.entries(queryObj)) {
            if (queryStr.startsWith("?")) {
                queryStr += `&${k}=${v}`;
            } else {
                queryStr += `?${k}=${v}`;
            }
        }

        return await this._jsonFetch<SearchResults>(false, "GET", `/projects/search${queryStr}`);
    }

    public async getProject(id: string | number) {
        return await this._jsonFetch<FullProject>(false, "GET", `/projects/${id}`);
    }

    public async getProjectAuthors(id: string | number) {
        return await this._jsonFetch<User[]>(false, "GET", `/projects/${id}/authors`);
    }

    public async createProject(data: ProjectInit) {
        return await this._jsonFetch<FullProject>(true, "PUT", "/projects", {}, data);
    }

    public async deleteProject(id: string | number) {
        return await this._fetch(true, "DELETE", `/projects/${id}`);
    }

    public async updateProject(id: string | number, data: Partial<Omit<ProjectInit, "slug">>) {
        return await this._jsonFetch<FullProject>(true, "PATCH", `/projects/${id}`, {}, data);
    }

    public async addProjectAuthor(id: string | number, author: string | number) {
        return await this._jsonFetch<FullProject>(
            true,
            "PUT",
            `/projects/${id}/authors`,
            {},
            author.toString(),
        );
    }

    public async removeProjectAuthor(id: string | number, author: string | number) {
        return await this._jsonFetch<FullProject>(
            true,
            "DELETE",
            `/projects/${id}/authors`,
            {},
            author.toString(),
        );
    }

    // =================== GALLERY ===================

    public async getGalleryImages(project: string | number) {
        return await this._jsonFetch<GalleryImage[]>(false, "GET", `/projects/${project}/gallery`);
    }

    public async getGalleryImage(project: string | number, image: string | number) {
        return await this._jsonFetch<GalleryImage>(
            false,
            "GET",
            `/projects/${project}/gallery/${image}`,
        );
    }

    public async uploadGalleryImage(project: string | number, data: GalleryImageInit) {
        const form = new FormData();

        form.set("name", data.name);
        form.set("ordering", data.ordering.toString());
        form.set("project", data.project.toString());
        form.set("file", data.file);

        if (data.description) form.set("description", data.description);

        return await this._jsonFetch<GalleryImage>(
            true,
            "PUT",
            `/projects/${project}/gallery`,
            form,
        );
    }

    public async updateGalleryImage(
        project: string | number,
        image: string | number,
        data: Partial<Omit<Omit<GalleryImageInit, "project">, "file">>,
    ) {
        return await this._jsonFetch<GalleryImage>(
            true,
            "PATCH",
            `/projects/${project}/gallery/${image}`,
            {},
            data,
        );
    }

    public async deleteGalleryImage(project: string | number, image: string | number) {
        return await this._fetch(true, "DELETE", `/projects/${project}/gallery/${image}`);
    }

    // =================== VERSIONS ===================

    public async getProjectVersions(project: string | number) {
        return await this._jsonFetch<ProjectVersion[]>(
            false,
            "GET",
            `/projects/${project}/versions`,
        );
    }

    public async getProjectVersion(project: string | number, version: string | number) {
        return await this._jsonFetch<ProjectVersion>(
            false,
            "GET",
            `/projects/${project}/versions/${version}`,
        );
    }

    public async getLatestVersion(project: string | number) {
        return await this._jsonFetch<ProjectVersion>(
            false,
            "GET",
            `/projects/${project}/versions/latest`,
        );
    }

    public async downloadProjectVersion(
        project: string | number,
        version: string | number,
        file: string | number,
    ) {
        return await (
            await this._fetch(
                false,
                "GET",
                `/projects/${project}/versions/${version}/download/${file}`,
            )
        ).arrayBuffer();
    }

    public downloadProjectVersionUrl(
        project: string | number,
        version: string | number,
        file: string | number,
    ) {
        return `${this._baseUrl}/projects/${project}/versions/${version}/download/${file}`;
    }

    public async uploadProjectVersion(project: string | number, data: ProjectVersionInit) {
        const form = new FormData();

        form.set("name", data.name);
        form.set("file_name", data.file_name);
        form.set("version_number", data.version_number.toString());
        form.set("game_versions", data.game_versions.join(","));
        form.set("loaders", data.loaders.join(","));
        form.set("file", data.file);

        if (data.changelog) form.set("changelog", data.changelog);

        return await this._jsonFetch<ProjectVersion>(
            true,
            "PUT",
            `/projects/${project}/versions`,
            form,
        );
    }

    public async updateProjectVersion(
        project: string | number,
        version: string | number,
        data: Partial<Omit<Omit<ProjectVersionInit, "file">, "file_name">>,
    ) {
        return await this._jsonFetch<ProjectVersion>(
            true,
            "PATCH",
            `/projects/${project}/versions/${version}`,
            {},
            data,
        );
    }

    public async deleteProjectVersion(project: string | number, version: string | number) {
        return await this._fetch(true, "DELETE", `/projects/${project}/versions/${version}`);
    }

    // =================== METADATA ===================

    public async getGameVersions() {
        return await this._jsonFetch<GameVersion[]>(false, "GET", "/meta/game_versions");
    }

    public async getModLoaders() {
        return await this._jsonFetch<ModLoader[]>(false, "GET", "/meta/loaders");
    }

    public async getTags() {
        return await this._jsonFetch<Tag[]>(false, "GET", "/meta/tags");
    }
}
