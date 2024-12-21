import type { PackageData, PackageVersion, Tag, User } from "$lib/types";
import type { PublicGalleryImage } from "$lib/types/gallery";
import { getToken, isLoggedIn } from "./auth";

export const getUser = async (id: string | number): Promise<User | undefined> => {
    try {
        return await (
            await fetch(`/api/v1/users/${id}`, {
                headers: isLoggedIn()
                    ? {
                          Authorization: `Bearer ${getToken()}`,
                      }
                    : {},
            })
        ).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getUserPackages = async (id: string | number): Promise<PackageData[] | undefined> => {
    try {
        return await (
            await fetch(`/api/v1/users/${id}/packages`, {
                headers: isLoggedIn()
                    ? {
                          Authorization: `Bearer ${getToken()}`,
                      }
                    : {},
            })
        ).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getPackage = async (
    id: string | number,
    fetcher = fetch,
): Promise<PackageData | undefined> => {
    try {
        return await (
            await fetcher(`/api/v1/packages/${id}`, {
                headers: isLoggedIn()
                    ? {
                          Authorization: `Bearer ${getToken()}`,
                      }
                    : {},
            })
        ).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getPackageAuthors = async (pkg: string | number): Promise<User[] | undefined> => {
    try {
        return await (
            await fetch(`/api/v1/packages/${pkg}/authors`, {
                headers: isLoggedIn()
                    ? {
                          Authorization: `Bearer ${getToken()}`,
                      }
                    : {},
            })
        ).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getPackageVersions = async (
    pkg: string | number,
    fetcher = fetch,
): Promise<PackageVersion[] | undefined> => {
    try {
        return await (
            await fetcher(`/api/v1/packages/${pkg}/versions`, {
                headers: isLoggedIn()
                    ? {
                          Authorization: `Bearer ${getToken()}`,
                      }
                    : {},
            })
        ).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getPackageVersion = async (
    pkg: string | number,
    version: string | number,
): Promise<PackageVersion | undefined> => {
    try {
        return await (
            await fetch(`/api/v1/packages/${pkg}/versions/${version}`, {
                headers: isLoggedIn()
                    ? {
                          Authorization: `Bearer ${getToken()}`,
                      }
                    : {},
            })
        ).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getPackageVersionFile = async (
    pkg: string | number,
    version: string | number,
): Promise<ArrayBuffer | undefined> => {
    try {
        return await (
            await fetch(`/api/v1/packages/${pkg}/versions/${version}/download`, {
                headers: isLoggedIn()
                    ? {
                          Authorization: `Bearer ${getToken()}`,
                      }
                    : {},
            })
        ).arrayBuffer();
    } catch (_err: any) {
        return undefined;
    }
};

export const getPackageGallery = async (
    pkg: string | number,
): Promise<PublicGalleryImage[] | undefined> => {
    try {
        return await (
            await fetch(`/api/v1/packages/${pkg}/gallery`, {
                headers: isLoggedIn()
                    ? {
                          Authorization: `Bearer ${getToken()}`,
                      }
                    : {},
            })
        ).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getGalleryImage = async (
    pkg: string | number,
    img: string | number,
): Promise<PublicGalleryImage | undefined> => {
    try {
        return await (
            await fetch(`/api/v1/packages/${pkg}/gallery/${img}`, {
                headers: isLoggedIn()
                    ? {
                          Authorization: `Bearer ${getToken()}`,
                      }
                    : {},
            })
        ).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getTags = async (): Promise<Tag[]> => {
    try {
        return await (await fetch("/api/v1/meta/tags")).json();
    } catch (_err: any) {
        return [];
    }
};
