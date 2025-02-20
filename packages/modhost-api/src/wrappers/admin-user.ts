import type { Client } from "../client";

export class AdminUserWrapper {
    private _client: Client;
    private _user: string | number;

    public constructor(client: Client, user: string | number) {
        this._client = client;
        this._user = user;
    }

    public get() {
        return this._client.adminGetUser(this._user);
    }

    public delete() {
        return this._client.adminDeleteUser(this._user);
    }
}
