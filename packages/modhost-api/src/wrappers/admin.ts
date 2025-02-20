import type { Client } from "../client";
import { AdminUserWrapper } from "./admin-user";
import { AdminStatsSocketWrapper } from "./stats-socket";

export class AdminWrapper {
    private _client: Client;

    public constructor(client: Client) {
        this._client = client;
    }

    public stats() {
        return this._client.getAdminStats();
    }

    public add(user: string | number) {
        return this._client.addAdmin(user);
    }

    public remove(user: string | number) {
        return this._client.removeAdmin(user);
    }

    public admins() {
        return this._client.listAdmins();
    }

    public allUsers() {
        return this._client.adminGetAllUsers();
    }

    public user(user: string | number) {
        return new AdminUserWrapper(this._client, user);
    }

    public statsSocket(host: string, proto = "wss") {
        const token = this._client.getToken();

        if (!token) throw new Error("Token is required for admin requests!");

        return new AdminStatsSocketWrapper(host, token, proto);
    }
}
