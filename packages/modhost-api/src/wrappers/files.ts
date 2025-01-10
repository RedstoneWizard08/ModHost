import type { Client } from "../client";
import { ErrorResponse } from "../models";

export class ProjectFilesWrapper {
    private _client: Client;
    private _project: string | number;
    private _version: string | number;

    public constructor(client: Client, project: string | number, version: string | number) {
        this._client = client;
        this._project = project;
        this._version = version;
    }

    public async list() {
        const res = await this._client.getProjectVersion(this._project, this._version);

        return res instanceof ErrorResponse ? res : res.files;
    }

    public download(file: string | number) {
        return this._client.downloadProjectVersion(this._project, this._version, file);
    }

    public downloadUrl(file: string | number) {
        return this._client.downloadProjectVersionUrl(this._project, this._version, file);
    }
}
