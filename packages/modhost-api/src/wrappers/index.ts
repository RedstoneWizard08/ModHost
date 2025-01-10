import { Client } from "../client";
import type { Facet, ProjectInit, SortDirection, SortMode } from "../models";
import { MetaWrapper } from "./meta";
import { ProjectWrapper } from "./project";
import { UserWrapper } from "./user";

export * from "./authors";
export * from "./gallery";
export * from "./project";
export * from "./versions";
export * from "./files";
export * from "./meta";
export * from "./user";

/**
 * The ergonomic API client meant to be used by other applications.
 * This wraps the {@link Client} to provide a more simplified API while
 * still being just as powerful.
 */
export class ModHostClient {
    private _client: Client;

    public constructor(client: Client);
    public constructor(baseUrl?: string, token?: string);

    public constructor(baseUrlOrClient: string | Client = "/api/v1", token?: string) {
        this._client =
            baseUrlOrClient instanceof Client
                ? baseUrlOrClient
                : new Client(baseUrlOrClient, token);
    }

    public setToken(token: string) {
        this._client.setToken(token);
    }

    public unsetToken() {
        this._client.unsetToken();
    }

    public hasToken() {
        return this._client.hasToken();
    }

    public search(
        query?: string,
        page = 1,
        perPage = 25,
        sort: SortMode = "none",
        dir: SortDirection = "desc",
        filters: Facet[] = [],
    ) {
        return this._client.searchProjects(query, page, perPage, sort, dir, filters);
    }

    public project(project: string | number) {
        return new ProjectWrapper(this._client, project);
    }

    public meta() {
        return new MetaWrapper(this._client);
    }

    public currentUser() {
        return this._client.currentUser();
    }

    public user(user: string | number) {
        return new UserWrapper(this._client, user);
    }

    public createProject(data: ProjectInit) {
        return this._client.createProject(data);
    }
}
