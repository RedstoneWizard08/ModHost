import type { Client } from "../client";

export class UserWrapper {
    private _client: Client;
    private _user: string | number;

    public constructor(client: Client, user: string | number) {
        this._client = client;
        this._user = user;
    }

    public get() {
        return this._client.getUser(this._user);
    }

    public projects() {
        return this._client.getUserProjects(this._user);
    }
}
