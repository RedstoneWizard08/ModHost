import type { Client } from "../client";
import type { FullProject, ProjectInit } from "../models";
import { AuthorsWrapper } from "./authors";
import { GalleryWrapper } from "./gallery";
import { VersionsWrapper } from "./versions";

export class ProjectWrapper {
    private _client: Client;
    private _project: string | number;

    public constructor(client: Client, project: string | number) {
        this._client = client;
        this._project = project;
    }

    public get() {
        return this._client.getProject(this._project);
    }

    public delete() {
        return this._client.deleteProject(this._project);
    }

    public update(data: Partial<Omit<ProjectInit, "slug">>) {
        return this._client.updateProject(this._project, data);
    }

    public authors() {
        return new AuthorsWrapper(this._client, this._project);
    }

    public gallery() {
        return new GalleryWrapper(this._client, this._project);
    }

    public versions() {
        return new VersionsWrapper(this._client, this._project);
    }
}
