import type { Client } from "../client";
import type { ProjectVersionInit } from "../models";
import { ProjectFilesWrapper } from "./files";

export class VersionsWrapper {
    private _client: Client;
    private _project: string | number;

    public constructor(client: Client, project: string | number) {
        this._client = client;
        this._project = project;
    }

    public list() {
        return this._client.getProjectVersions(this._project);
    }

    public latest() {
        return this._client.getLatestVersion(this._project);
    }

    public upload(data: ProjectVersionInit) {
        return this._client.uploadProjectVersion(this._project, data);
    }

    public version(version: string | number) {
        return new VersionWrapper(this._client, this._project, version);
    }
}

export class VersionWrapper {
    private _client: Client;
    private _project: string | number;
    private _version: string | number;

    public constructor(client: Client, project: string | number, version: string | number) {
        this._client = client;
        this._project = project;
        this._version = version;
    }

    public get() {
        return this._client.getProjectVersion(this._project, this._version);
    }

    public update(data: Partial<Omit<Omit<ProjectVersionInit, "file">, "file_name">>) {
        return this._client.updateProjectVersion(this._project, this._version, data);
    }

    public delete() {
        return this._client.deleteProjectVersion(this._project, this._version);
    }

    public files() {
        return new ProjectFilesWrapper(this._client, this._project, this._version);
    }
}
