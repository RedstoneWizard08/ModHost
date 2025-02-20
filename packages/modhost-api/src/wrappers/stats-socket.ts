import { EventEmitter } from "typed-event-emitter";
import type { AdminStats } from "../models";

export class AdminStatsSocketWrapper extends EventEmitter {
    public readonly onOpen = this.registerEvent<[WebSocket]>();
    public readonly onData = this.registerEvent<[AdminStats]>();

    private _host: string;
    private _proto: string;
    private _token: string;
    private _ws: WebSocket;
    private _current?: AdminStats;

    public constructor(host: string, token: string, proto = "wss") {
        super();

        this._host = host;
        this._proto = proto;
        this._token = token;

        this._ws = new WebSocket(
            `${this._proto}://${this._host}/api/v1/admin/stats/ws?t=${this._token}`,
        );

        this._ws.addEventListener("open", this._onOpen.bind(this));
        this._ws.addEventListener("message", this._onMessage.bind(this));
    }

    private _onOpen() {
        this.emit(this.onOpen, this._ws);
    }

    private _onMessage(ev: MessageEvent) {
        const data = JSON.parse(ev.data);

        this._current = data;
        this.emit(this.onData, data);
    }

    public get value() {
        return this._current;
    }
}
