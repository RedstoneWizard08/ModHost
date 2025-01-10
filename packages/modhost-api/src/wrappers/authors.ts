import type { Client } from "../client";

export class AuthorsWrapper {
    private _client: Client;
    private _project: string | number;

    public constructor(client: Client, project: string | number) {
        this._client = client;
        this._project = project;
    }

    public get() {
        return this._client.getProjectAuthors(this._project);
    }

    public add(author: string | number) {
        return this._client.addProjectAuthor(this._project, author);
    }

    public remove(author: string | number) {
        return this._client.removeProjectAuthor(this._project, author);
    }
}
