import type { Client } from "../client";
import { ErrorResponse, type GalleryImageInit } from "../models";

export class GalleryWrapper {
    private _client: Client;
    private _project: string | number;

    public constructor(client: Client, project: string | number) {
        this._client = client;
        this._project = project;
    }

    public list() {
        return this._client.getGalleryImages(this._project);
    }

    public async upload(data: Omit<GalleryImageInit, "project">) {
        const project = await this.getProjectId();

        if (project instanceof ErrorResponse) {
            return project;
        }

        return this._client.uploadGalleryImage(this._project, {
            ...data,
            project,
        });
    }

    public image(image: string | number) {
        return new GalleryImageWrapper(this._client, this._project, image);
    }

    private async getProjectId() {
        if (typeof this._project === "string") {
            const res = await this._client.getProject(this._project);

            if (res instanceof ErrorResponse) {
                return res;
            }

            return res.id;
        }

        return this._project;
    }
}

export class GalleryImageWrapper {
    private _client: Client;
    private _project: string | number;
    private _image: string | number;

    public constructor(client: Client, project: string | number, image: string | number) {
        this._client = client;
        this._project = project;
        this._image = image;
    }

    public get() {
        return this._client.getGalleryImage(this._project, this._image);
    }

    public update(data: Partial<Omit<Omit<GalleryImageInit, "project">, "file">>) {
        return this._client.updateGalleryImage(this._project, this._image, data);
    }

    public delete() {
        return this._client.deleteGalleryImage(this._project, this._image);
    }
}
