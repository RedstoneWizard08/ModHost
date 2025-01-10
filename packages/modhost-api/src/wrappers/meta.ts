import type { Client } from "../client";

export class MetaWrapper {
    private _client: Client;

    public constructor(client: Client) {
        this._client = client;
    }

    public gameVersions() {
        return this._client.getGameVersions();
    }

    public loaders() {
        return this._client.getModLoaders();
    }

    public tags() {
        return this._client.getTags();
    }
}
